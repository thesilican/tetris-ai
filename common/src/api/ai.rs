use crate::model::{Bag, Game, GameActionRes, GameMove, PERMS_1F};
use serde::Serialize;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AiRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}
impl AiRes {
    fn expected(&self, mut game: Game) -> Option<Game> {
        match self {
            AiRes::Success { moves, .. } => {
                for &game_move in moves {
                    game.make_move(game_move);
                }
                Some(game)
            }
            AiRes::Fail { .. } => None,
        }
    }
}
impl Display for AiRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AiRes::Success { moves, score } => {
                let score = match score {
                    Some(score) => format!("{score:.2}"),
                    None => String::from("None"),
                };
                let moves = moves
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Eval Score: {score} Moves: [{moves}]")
            }
            AiRes::Fail { reason } => {
                write!(f, "Eval Failed: {reason}")
            }
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
// Tagged version of AiResSer
enum AiResSer {
    Success {
        success: bool,
        moves: Vec<GameMove>,
        score: Option<f64>,
        expected: Game,
    },
    Fail {
        success: bool,
        reason: String,
    },
}
impl AiResSer {
    fn from(ai_res: AiRes, game: Game) -> Self {
        let expected = ai_res.expected(game);
        match ai_res {
            AiRes::Success { moves, score } => AiResSer::Success {
                success: true,
                moves,
                score,
                expected: expected.unwrap(),
            },
            AiRes::Fail { reason } => AiResSer::Fail {
                success: false,
                reason,
            },
        }
    }
}

pub trait Ai {
    fn evaluate(&mut self, game: &Game) -> AiRes;
    /// Same as ai.evaluate() but using JSON as input/output
    fn api_evaluate(&mut self, req: &str) -> String {
        let game = match serde_json::from_str(req) {
            Ok(game) => game,
            Err(parse_err) => {
                let res_ser = AiResSer::Fail {
                    success: false,
                    reason: format!("Invalid request: {parse_err}"),
                };
                return serde_json::to_string(&res_ser).unwrap();
            }
        };
        let res = self.evaluate(&game);
        let res_ser = AiResSer::from(res, game);
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
                AiRes::Success { moves, score } => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let GameActionRes::SuccessDrop { top_out, .. } = res {
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
                    println!(
                        "{game}\nEval: {score} in {elapsed:?}\n[{moves}]\n"
                    );
                }
                AiRes::Fail { reason } => {
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
                AiRes::Success { moves, .. } => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let GameActionRes::SuccessDrop { top_out, .. } = res {
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
                AiRes::Fail { reason } => {
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
                AiRes::Success { moves, .. } => {
                    for game_move in moves {
                        game.make_move(game_move);
                    }
                    game.refill_queue(&mut bag);
                }
                AiRes::Fail { .. } => {
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
    fn evaluate(&mut self, game: &Game) -> AiRes {
        let child_states = game.child_states(&PERMS_1F);
        let mut best_child = None;
        let mut best_height = i32::MAX;
        let mut best_holes = i32::MAX;
        for child in child_states.iter().rev() {
            let height = child
                .game
                .board
                .height_map()
                .iter()
                .map(|&x| {
                    // Square so that higher heights are punished more
                    let x = x as i32;
                    x * x
                })
                .sum();
            let holes = child.game.board.holes().iter().sum();
            if height < best_height || (height == best_height && holes < best_holes) {
                best_height = height;
                best_holes = holes;
                best_child = Some(child);
            }
        }
        match best_child {
            Some(child) => AiRes::Success {
                moves: child.moves().collect(),
                score: Some(child_states.len() as f64),
            },
            None => AiRes::Fail {
                reason: "No valid moves".into(),
            },
        }
    }
}
