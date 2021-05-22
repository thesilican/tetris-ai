use crate::api::json::{parse, stringify, JSONOutput};
use crate::model::consts::PIECE_NUM_TYPES;
use crate::model::game::{Game, GameMove, GameMoveRes};
use crate::model::piece::{Piece, PieceType};
use std::fmt::{self, Display, Formatter};
use std::time::Instant;

pub enum TetrisAIRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}
impl Display for TetrisAIRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TetrisAIRes::Success { moves, score } => {
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
            TetrisAIRes::Fail { reason } => {
                write!(f, "Eval Failed: {}", reason)
            }
        }
    }
}

pub trait TetrisAI {
    fn api_evaluate(&mut self, game: &Game) -> TetrisAIRes;
    /// Convenience function to handle JSON requests
    /// Also returns JSON if request is invalid
    fn api_evaluate_json(&mut self, req: String) -> String {
        let game = match parse(req) {
            Ok(game) => game,
            Err(parse_err) => {
                let output = JSONOutput::Fail {
                    success: false,
                    reason: format!("Invalid request: {}", parse_err),
                };
                return serde_json::to_string(&output).unwrap();
            }
        };
        let res = self.api_evaluate(&game);
        stringify(res)
    }
    /// A quick and easy way to watch an ai play a game
    fn watch_ai(&mut self, mut seed: i32) {
        let mut gen_bag = || {
            let mut bag = PieceType::iter_types().collect::<Vec<_>>();
            // Fisher-Yates shuffle
            for i in (1..PIECE_NUM_TYPES).rev() {
                let j = seed % i;
                bag.swap(i as usize, j as usize);
                // Epic way to randomize
                seed = (seed + j + 123) % 456_789;
            }
            bag.into_iter().map(|p| Piece::new(&p))
        };
        let mut game = Game::new();
        game.extend_queue(gen_bag().into_iter());
        println!("{}\n", game);
        'l: loop {
            if game.queue_pieces.len() < 7 {
                game.extend_queue(gen_bag().into_iter());
            }
            let start = Instant::now();
            let res = self.api_evaluate(&mut game);
            let elapsed = start.elapsed();
            if let TetrisAIRes::Success { moves, score } = res {
                for game_move in &moves {
                    if let GameMove::HardDrop = game_move {
                        let res = game.make_move(&game_move);
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
            } else if let TetrisAIRes::Fail { reason } = res {
                println!("Evaluation failed: {}", reason);
                break;
            }
        }
    }
}

/// Hard drops, returns error if unable to hard drop
pub struct DummyAI;
impl DummyAI {
    pub fn new() -> Self {
        DummyAI
    }
}
impl TetrisAI for DummyAI {
    fn api_evaluate(&mut self, game: &Game) -> TetrisAIRes {
        match game.clone().make_move(&GameMove::HardDrop) {
            GameMoveRes::SuccessDrop(..) => TetrisAIRes::Success {
                moves: vec![GameMove::HardDrop],
                score: Some(1.0),
            },
            GameMoveRes::Failed => TetrisAIRes::Fail {
                reason: "Unable to hard drop".into(),
            },
            GameMoveRes::SuccessNorm => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TetrisAI;
    use crate::api::ai::DummyAI;

    #[test]

    fn dummy_ai_should_work() {
        let mut ai = DummyAI::new();

        // Empty Board
        let json = r#"{
            "current": 0,
            "hold": null,
            "queue": [1, 2],
            "matrix": [
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ],
              [
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false
              ]
            ]
          }
        "#;
        let output = r#"{"success":true,"moves":["hardDrop"],"score":1.0}"#;

        let res = ai.api_evaluate_json(json.into());
        assert_eq!(res, output);

        // Invalid json
        let json = r#"I like cookies"#;
        let output = r#"{"success":false,"reason":"Invalid request: Serde Error: expected value at line 1 column 1"}"#;
        let res = ai.api_evaluate_json(json.into());
        assert_eq!(res, output);
    }
}
