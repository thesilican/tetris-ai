use crate::api::json::{parse, stringify, JSONOutput};
use crate::model::game::{Game, GameMove, GameMoveRes};
use std::fmt::{self, Display, Formatter};

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
    fn api_evaluate(&mut self, game: &mut Game) -> TetrisAIRes;
    /// Convenience function to handle JSON requests
    /// Also returns JSON if request is invalid
    fn api_evaluate_json(&mut self, req: String) -> String {
        let mut game = match parse(req) {
            Ok(game) => game,
            Err(parse_err) => {
                let output = JSONOutput::Fail {
                    success: false,
                    reason: format!("Invalid request: {}", parse_err),
                };
                return serde_json::to_string(&output).unwrap();
            }
        };
        let res = self.api_evaluate(&mut game);
        stringify(res)
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
    fn api_evaluate(&mut self, game: &mut Game) -> TetrisAIRes {
        match game.make_move(&GameMove::HardDrop) {
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
