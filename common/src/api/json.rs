use crate::api::ai::TetrisAiRes;
use crate::misc::GenericErr;
use crate::model::Game;
use crate::model::PieceType;
use crate::model::BOARD_HEIGHT;
use crate::model::BOARD_VISIBLE_HEIGHT;
use crate::model::BOARD_WIDTH;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Deserialize)]
struct JsonInput {
    pub current: i32,
    pub hold: Option<i32>,
    pub queue: Vec<i32>,
    pub matrix: Vec<Vec<bool>>,
}
impl TryFrom<JsonInput> for Game {
    type Error = GenericErr;
    fn try_from(input: JsonInput) -> Result<Game, GenericErr> {
        fn try_parse_piece(num: i32) -> Result<PieceType, GenericErr> {
            let piece_type = match PieceType::try_from(num) {
                Ok(p) => p,
                Err(err) => {
                    return Err(format!("Error parsing JSON: {}", err).into());
                }
            };
            Ok(piece_type)
        }
        let current = try_parse_piece(input.current)?;
        let hold = match input.hold {
            Some(p) => Some(try_parse_piece(p)?),
            None => None,
        };
        let mut queue = Vec::new();
        for piece in input.queue {
            queue.push(try_parse_piece(piece)?);
        }
        let matrix_w = input.matrix.len();
        let matrix_h = input.matrix.get(0).map(|x| x.len()).unwrap_or(0);
        if matrix_w != BOARD_WIDTH as usize || matrix_h != BOARD_VISIBLE_HEIGHT as usize {
            return Err(format!(
                "Error parsing JSON: Invalid matrix dimensions {}x{}",
                matrix_w, matrix_h
            )
            .into());
        }
        let mut matrix = [0; BOARD_HEIGHT as usize];
        for (i, row) in input.matrix.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if cell {
                    matrix[j] |= 1 << i;
                }
            }
        }

        let mut game = Game::new();
        game.set_current(current);
        game.set_hold(hold);
        game.extend_queue(&queue);
        game.board.set_matrix(matrix);
        Ok(game)
    }
}
impl FromStr for Game {
    type Err = GenericErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = serde_json::from_str::<JsonInput>(&s)?;
        input.try_into()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum JsonOutput {
    Success {
        success: bool,
        moves: Vec<String>,
        score: Option<f64>,
    },
    Fail {
        success: bool,
        reason: String,
    },
}
impl From<TetrisAiRes> for JsonOutput {
    fn from(eval: TetrisAiRes) -> Self {
        match eval {
            TetrisAiRes::Success { moves, score } => JsonOutput::Success {
                success: true,
                moves: moves.into_iter().map(|x| x.to_string()).collect(),
                score,
            },
            TetrisAiRes::Fail { reason } => JsonOutput::Fail {
                success: false,
                reason,
            },
        }
    }
}
impl TetrisAiRes {
    pub fn to_json(self) -> String {
        let output = JsonOutput::from(self);
        serde_json::to_string(&output).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::TetrisAiRes,
        model::{Game, GameMove, PieceType},
    };
    use std::str::FromStr;
    #[test]

    fn json_should_parse_properly() {
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
                    false, false, true, false, false, false, false, false, false, false,
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
        }"#;
        let json_game = Game::from_str(json).unwrap();

        let mut game = Game::new();
        game.set_current(PieceType::O);
        game.set_hold(None);
        game.append_queue(PieceType::I);
        game.append_queue(PieceType::T);
        game.board.set(1, 2, true);

        assert_eq!(game, json_game);

        // Empty json
        let json = "";
        let res = Game::from_str(json);
        assert!(res.is_err());

        // Invalid matrix size
        let json = r#"{
            "current": 0,
            "hold": null,
            "queue": [1, 2],
            "matrix": [
              [
                false, false
              ],
            ]
          }
        "#;
        let res = Game::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn json_should_serialize_properly() {
        // Example JSON
        let json = r#"{"success":true,"moves":["hardDrop","rotate180"],"score":1.5}"#;
        let eval = TetrisAiRes::Success {
            moves: vec![GameMove::HardDrop, GameMove::Rotate180],
            score: Some(1.5),
        };
        let json_eval = eval.to_json();
        assert_eq!(json, json_eval);

        // Empty JSON
        let json = r#"{"success":true,"moves":[],"score":null}"#;
        let eval = TetrisAiRes::Success {
            moves: vec![],
            score: None,
        };
        let json_eval = eval.to_json();
        assert_eq!(json, json_eval);

        // Failed Result
        let json = r#"{"success":false,"reason":"I suck at Tetris"}"#;
        let eval = TetrisAiRes::Fail {
            reason: "I suck at Tetris".into(),
        };
        let json_eval = eval.to_json();
        assert_eq!(json, json_eval);
    }
}
