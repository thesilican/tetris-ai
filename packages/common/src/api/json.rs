use crate::api::ai::TetrisAIRes;
use crate::misc::GenericErr;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_VISIBLE_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::game::Game;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Deserialize)]
pub(crate) struct JSONInput {
  pub current: i32,
  pub hold: Option<i32>,
  pub queue: Vec<i32>,
  pub matrix: Vec<Vec<bool>>,
}
impl TryFrom<JSONInput> for Game {
  type Error = GenericErr;
  fn try_from(input: JSONInput) -> Result<Game, GenericErr> {
    fn try_parse_piece(num: i32) -> Result<Piece, GenericErr> {
      let piece_type = match PieceType::from_i32(num) {
        Ok(p) => p,
        Err(err) => {
          return Err(format!("Error parsing JSON: {}", err).into());
        }
      };
      Ok(Piece::new(&piece_type))
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
      return Err(
        format!(
          "Error parsing JSON: Invalid matrix dimensions {}x{}",
          matrix_w, matrix_h
        )
        .into(),
      );
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
    game.extend_queue(queue.into_iter());
    game.board.set_matrix(matrix);
    Ok(game)
  }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum JSONOutput {
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
impl From<TetrisAIRes> for JSONOutput {
  fn from(eval: TetrisAIRes) -> Self {
    match eval {
      TetrisAIRes::Success { moves, score } => JSONOutput::Success {
        success: true,
        moves: moves.into_iter().map(|x| x.to_string()).collect(),
        score,
      },
      TetrisAIRes::Fail { reason } => JSONOutput::Fail {
        success: false,
        reason,
      },
    }
  }
}

/// Parses JSON into a valid game
///
/// JSON Schema:
/// - current: number
/// - hold: number or null
/// - queue: number[]
/// - matrix: boolean[][]
///
/// Constraints:
/// - matrix must be 10 by 20
/// - All numbers must be integers between 0-7
pub fn parse(json: String) -> Result<Game, GenericErr> {
  let input = serde_json::from_str::<JSONInput>(&json)?;
  input.try_into()
}

/// Serializes TetrisAI evaluation into JSON
///
/// JSON Schema:
/// - success: boolean
/// - (if success == true)
///     - moves: string[] (a GameMove string)
///     - score: string | null
/// - (if success == false)
///     - reason: string
pub fn stringify(eval: TetrisAIRes) -> String {
  let output = eval.into();
  serde_json::to_string::<JSONOutput>(&output).unwrap()
}

#[cfg(test)]
mod tests {
  use super::{parse, stringify};
  use crate::api::json::TetrisAIRes;
  use crate::model::game::{Game, GameMove};
  use crate::model::piece::{Piece, PieceType};
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
    }
  "#;
    let json_game = parse(json.into()).unwrap();

    let mut game = Game::new();
    game.set_current(Piece::new(&PieceType::O));
    game.set_hold(None);
    game.append_queue(Piece::new(&PieceType::I));
    game.append_queue(Piece::new(&PieceType::T));
    game.board.set(1, 2, true);

    assert_eq!(game, json_game);

    // Empty json
    let json = "";
    let res = parse(json.into());
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
    let res = parse(json.into());
    assert!(res.is_err());
  }

  #[test]
  fn json_should_serialize_properly() {
    // Example JSON
    let json = r#"{"success":true,"moves":["hardDrop","rotate180"],"score":1.5}"#;
    let eval = TetrisAIRes::Success {
      moves: vec![GameMove::HardDrop, GameMove::Rotate180],
      score: Some(1.5),
    };
    let json_eval = stringify(eval);
    assert_eq!(json, json_eval);

    // Empty JSON
    let json = r#"{"success":true,"moves":[],"score":null}"#;
    let eval = TetrisAIRes::Success {
      moves: vec![],
      score: None,
    };
    let json_eval = stringify(eval);
    assert_eq!(json, json_eval);

    // Failed Result
    let json = r#"{"success":false,"reason":"I suck at Tetris"}"#;
    let eval = TetrisAIRes::Fail {
      reason: "I suck at Tetris".into(),
    };
    let json_eval = stringify(eval);
    assert_eq!(json, json_eval);
  }
}
