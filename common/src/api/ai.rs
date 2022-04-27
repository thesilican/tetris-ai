use crate::model::{Bag, ChildState, Game, GameMove, GameMoveRes, MOVES_1F};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "AiResSer")]
#[serde(into = "AiResSer")]
pub enum AiRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}
impl Display for AiRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AiRes::Success { moves, score } => {
                let score = match score {
                    Some(score) => format!("{:.2}", score),
                    None => String::from("None"),
                };
                let moves = moves
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Eval Score: {} Moves: [{}]", score, moves)
            }
            AiRes::Fail { reason } => {
                write!(f, "Eval Failed: {}", reason)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
// Tagged version of AiResSer, used for ser/de
enum AiResSer {
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
impl From<AiRes> for AiResSer {
    fn from(ai_res: AiRes) -> Self {
        match ai_res {
            AiRes::Success { moves, score } => AiResSer::Success {
                success: true,
                moves,
                score,
            },
            AiRes::Fail { reason } => AiResSer::Fail {
                success: false,
                reason,
            },
        }
    }
}
impl From<AiResSer> for AiRes {
    fn from(ai_res_ser: AiResSer) -> Self {
        match ai_res_ser {
            AiResSer::Success { moves, score, .. } => AiRes::Success { moves, score },
            AiResSer::Fail { reason, .. } => AiRes::Fail { reason },
        }
    }
}

pub trait Ai {
    fn evaluate(&mut self, game: &Game) -> AiRes;
    /// Same as ai.evaluate() but using JSON as input/output
    fn api_evaluate(&mut self, req: &str) -> String {
        let game = match serde_json::from_str(&req) {
            Ok(game) => game,
            Err(parse_err) => {
                let output = AiRes::Fail {
                    reason: format!("Invalid request: {}", parse_err),
                };
                return serde_json::to_string(&output).unwrap();
            }
        };
        let res = self.evaluate(&game);
        serde_json::to_string(&res).unwrap()
    }
    /// A quick and easy way to watch an ai play a game
    fn watch_ai(&mut self, seed: u64) {
        let mut bag = Bag::new(seed);
        let mut game = Game::from_bag_shuffled(&mut bag);
        println!("{}\n", game);
        'l: loop {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();
            match res {
                AiRes::Success { moves, score } => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let GameMoveRes::SuccessDrop(drop_res) = res {
                                if drop_res.top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(game_move);
                        }
                    }
                    let score = score
                        .map(|x| format!("{:.2}", x))
                        .unwrap_or(String::from("None"));
                    let moves = moves
                        .into_iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!(
                        "{0}\nEval: {1} in {3:?}\n[{2}]\n",
                        game, score, moves, elapsed
                    );
                }
                AiRes::Fail { reason } => {
                    println!("Evaluation failed: {}", reason);
                    break;
                }
            }
            game.refill_queue_shuffled(&mut bag);
        }
    }
    /// A prettier version of watch_ai(), intended for demoing a bot
    fn watch_ai_demo(&mut self, piece_delay_ms: u64) {
        let mut bag = Bag::new(0);
        let mut game = Game::from_bag_shuffled(&mut bag);
        println!("{}", game);
        'l: loop {
            let res = self.evaluate(&game);
            match res {
                AiRes::Success { moves, .. } => {
                    for &game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(game_move);
                            if let GameMoveRes::SuccessDrop(drop_res) = res {
                                if drop_res.top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(game_move);
                        }
                        println!("{}", game);
                        std::thread::sleep(std::time::Duration::from_millis(piece_delay_ms));
                    }
                }
                AiRes::Fail { reason } => {
                    println!("Evaluation failed: {}", reason);
                    break;
                }
            }
            game.refill_queue_shuffled(&mut bag);
        }
    }
    /// Easy way to benchmark the performance of an Ai
    fn bench_ai(&mut self, eval_count: u32, seed: u64) {
        let mut bag = Bag::new(seed);
        let mut game = Game::from_bag_shuffled(&mut bag);

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
                    game.refill_queue_shuffled(&mut bag);
                }
                AiRes::Fail { .. } => {
                    // Reset game
                    game = Game::from_bag_shuffled(&mut bag);
                }
            }
        }
        println!("Total evaluations: {}", eval_count);
        println!("Total time: {:?}", total_time);
        println!("Time per evaluation: {:?}", total_time / eval_count);
    }
}

/// A very simple ai, useful for testing.
/// Should never top out...
pub struct SimpleAi;
impl SimpleAi {
    pub fn new() -> Self {
        SimpleAi
    }
}
impl Ai for SimpleAi {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        let child_states = game.child_states(&MOVES_1F);
        let mut best_moves = None;
        let mut best_height = i32::MAX;
        let mut best_holes = i32::MAX;
        for child_state in child_states.iter().rev() {
            let ChildState { game, moves } = child_state;
            let height = game
                .board
                .height_map()
                .iter()
                .map(|&x| {
                    // Square so that higher heights are punished more
                    let x = x as i32;
                    x * x
                })
                .sum();
            let holes = game.board.holes().iter().sum();
            if height < best_height || (height == best_height && holes < best_holes) {
                best_height = height;
                best_holes = holes;
                best_moves = Some(*moves);
            }
        }
        match best_moves {
            Some(moves) => AiRes::Success {
                moves: Vec::from(moves),
                score: Some(child_states.len() as f64),
            },
            None => AiRes::Fail {
                reason: "No valid moves".into(),
            },
        }
    }
}
