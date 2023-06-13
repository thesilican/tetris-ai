use crate::model::{ActionResult, Bag, Game, GameMove};
use serde::Serialize;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

pub type AiResult = Result<AiEval, String>;

pub struct AiEval {
    pub moves: Vec<GameMove>,
    pub score: Option<f64>,
}

#[derive(Serialize)]
#[serde(untagged)]
// Tagged version of AiResult
enum AiResultSer {
    Success {
        success: bool,
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        success: bool,
        reason: String,
    },
}
impl From<AiResult> for AiResultSer {
    fn from(ai_res: AiResult) -> Self {
        match ai_res {
            Ok(AiEval { moves, score }) => AiResultSer::Success {
                success: true,
                moves,
                score,
            },
            Err(reason) => AiResultSer::Fail {
                success: false,
                reason,
            },
        }
    }
}

pub trait Ai {
    fn evaluate(&mut self, game: &Game) -> AiResult;
    /// Same as ai.evaluate() but using JSON as input/output
    fn api_evaluate(&mut self, req: &str) -> String {
        let game = match serde_json::from_str(req) {
            Ok(game) => game,
            Err(parse_err) => {
                let res_ser = AiResultSer::Fail {
                    success: false,
                    reason: format!("Invalid request: {parse_err}"),
                };
                return serde_json::to_string(&res_ser).unwrap();
            }
        };
        let res = self.evaluate(&game);
        let res_ser = AiResultSer::from(res);
        serde_json::to_string(&res_ser).unwrap()
    }
    /// A quick and easy way to watch an ai play a game
    fn watch_ai(&mut self, seed: u64) {
        let mut bag = Bag::new_rng7(seed);
        let mut game = Game::from_bag(&mut bag);
        println!("{game}\n");
        'l: loop {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();
            match res {
                Ok(AiEval { moves, score }) => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let ActionResult::SuccessDrop { top_out, .. } = res {
                                if top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(game_move);
                        }
                    }
                    let score = score
                        .map(|x| format!("{x:.2}"))
                        .unwrap_or(String::from("None"));
                    let moves = moves
                        .into_iter()
                        .map(|x| format!("{x}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("{game}\nEval: {score} in {elapsed:?}\n[{moves}]\n");
                }
                Err(reason) => {
                    println!("Evaluation failed: {reason}");
                    break;
                }
            }
            game.refill_queue(&mut bag);
        }
    }
    /// A prettier version of watch_ai(), intended for demoing a bot
    fn watch_ai_demo(&mut self, piece_delay_ms: u64) {
        let mut bag = Bag::new_rng7(0);
        let mut game = Game::from_bag(&mut bag);
        println!("{game}");
        'l: loop {
            let res = self.evaluate(&game);
            match res {
                Ok(AiEval { moves, .. }) => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let ActionResult::SuccessDrop { top_out, .. } = res {
                                if top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(game_move);
                        }
                        println!("{game}");
                        std::thread::sleep(std::time::Duration::from_millis(piece_delay_ms));
                    }
                }
                Err(reason) => {
                    println!("Evaluation failed: {reason}");
                    break;
                }
            }
            game.refill_queue(&mut bag);
        }
    }
    /// Easy way to benchmark the performance of an Ai
    fn bench_ai(&mut self, eval_count: u32, seed: u64) {
        let mut bag = Bag::new_rng7(seed);
        let mut game = Game::from_bag(&mut bag);

        let mut total_time = Duration::new(0, 0);

        for _ in 0..eval_count {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();
            total_time += elapsed;

            match res {
                Ok(AiEval { moves, .. }) => {
                    for game_move in moves {
                        game.make_move(game_move);
                    }
                    game.refill_queue(&mut bag);
                }
                Err(_) => {
                    // Reset game
                    game = Game::from_bag(&mut bag);
                }
            }
        }
        println!("Total evaluations: {eval_count}");
        println!("Total time: {total_time:?}");
        println!("Time per evaluation: {:?}", total_time / eval_count);
    }
}

/// A very simple ai, useful for testing.
/// Should never top out...
#[derive(Debug)]
pub struct SimpleAi;
impl SimpleAi {
    pub fn new() -> Self {
        SimpleAi
    }
}
impl Ai for SimpleAi {
    fn evaluate(&mut self, game: &Game) -> AiResult {
        let children = match game.children() {
            Ok(children) => children,
            Err(_) => {
                return Ok(AiEval {
                    moves: [GameMove::HardDrop].into_iter().collect(),
                    score: None,
                });
            }
        };
        let mut best_child = None;
        let mut best_height = u32::MAX;
        let mut best_holes = u32::MAX;
        for child in children.iter().rev() {
            let height = child.game.board.height_map().iter().map(|&x| x * x).sum();
            let holes = child.game.board.holes().iter().sum();
            if height < best_height || (height == best_height && holes < best_holes) {
                best_height = height;
                best_holes = holes;
                best_child = Some(child);
            }
        }
        match best_child {
            Some(child) => Ok(AiEval {
                moves: child.moves().collect(),
                score: Some(children.len() as f64),
            }),
            None => Err("No valid moves".to_string()),
        }
    }
}
