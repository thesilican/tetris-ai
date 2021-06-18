use rand::SeedableRng;

use crate::api::json::JsonOutput;
use crate::model::{gen_child_states_dr, Bag, Game, GameMove, GameMoveRes};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum TetrisAiRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}
impl Display for TetrisAiRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TetrisAiRes::Success { moves, score } => {
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
            TetrisAiRes::Fail { reason } => {
                write!(f, "Eval Failed: {}", reason)
            }
        }
    }
}

pub trait TetrisAi {
    fn evaluate(&mut self, game: &Game) -> TetrisAiRes;
    fn api_evaluate(&mut self, req: String) -> String {
        let game = match Game::from_str(&req) {
            Ok(game) => game,
            Err(parse_err) => {
                let output = JsonOutput::Fail {
                    success: false,
                    reason: format!("Invalid request: {}", parse_err),
                };
                return serde_json::to_string(&output).unwrap();
            }
        };
        let res = self.evaluate(&game);
        res.to_json()
    }
    /// A quick and easy way to watch an ai play a game
    fn watch_ai(&mut self, seed: u64) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut bag = Bag::new();
        let mut game = Game::new_with_bag(&bag);
        println!("{}\n", game);
        'l: loop {
            if game.queue_pieces.len() < 7 {
                bag.shuffle(&mut rng);
                game.extend_bag(&bag);
            }
            let start = Instant::now();
            let res = self.evaluate(&mut game);
            let elapsed = start.elapsed();
            match res {
                TetrisAiRes::Success { moves, score } => {
                    for game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(*game_move);
                            if let GameMoveRes::SuccessDrop(drop_res) = res {
                                if drop_res.top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(*game_move);
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
                TetrisAiRes::Fail { reason } => {
                    println!("Evaluation failed: {}", reason);
                    break;
                }
            }
        }
    }
}

pub struct DummyAi;
impl DummyAi {
    pub fn new() -> Self {
        DummyAi
    }
}
impl TetrisAi for DummyAi {
    fn evaluate(&mut self, game: &Game) -> TetrisAiRes {
        let child_states = gen_child_states_dr(game);
        match child_states.first() {
            Some(&(_, moves)) => TetrisAiRes::Success {
                moves: moves
                    .iter()
                    .map(|x| *x)
                    .chain(std::iter::once(GameMove::HardDrop))
                    .collect(),
                score: Some(child_states.len() as f64),
            },
            _ => TetrisAiRes::Fail {
                reason: "No valid moves".into(),
            },
        }
    }
}
