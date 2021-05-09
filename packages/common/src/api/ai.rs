use crate::api::json::parse;
use crate::api::json::stringify;
use crate::misc::GenericErr;
use crate::model::game::{Game, GameMove, GameMoveRes};

pub enum TetrisAIRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}

pub trait TetrisAI {
    fn api_evaluate(&mut self, game: &mut Game) -> TetrisAIRes;
    /// Convenience function that uses JSON parse/stringify
    fn api_evaluate_json(&mut self, req: String) -> Result<String, GenericErr> {
        let mut game = parse(req)?;
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
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), output);

        // Invalid json
        let json = r#"I like cookies"#;
        let res = ai.api_evaluate_json(json.into());
        assert!(res.is_err());
    }
}
