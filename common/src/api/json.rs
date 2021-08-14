use crate::api::ai::AiRes;
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

#[derive(Serialize, Deserialize)]
struct JsonGame {
    pub current: i32,
    pub hold: Option<i32>,
    pub queue: Vec<i32>,
    pub matrix: [[bool; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
}
impl TryFrom<JsonGame> for Game {
    type Error = GenericErr;
    fn try_from(input: JsonGame) -> Result<Game, GenericErr> {
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

        let mut matrix = [0; BOARD_HEIGHT as usize];
        for (i, row) in input.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell {
                    matrix[j] |= 1 << i;
                }
            }
        }
        let mut game = Game::from_pieces(current, hold, &queue);
        game.board.set_matrix(matrix);
        Ok(game)
    }
}
impl FromStr for Game {
    type Err = GenericErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = serde_json::from_str::<JsonGame>(s)?;
        input.try_into()
    }
}
impl From<Game> for JsonGame {
    fn from(game: Game) -> Self {
        let mut matrix = [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];
        for j in 0..BOARD_WIDTH {
            for i in 0..BOARD_HEIGHT {
                let cell = game.board.get(i, j);
                matrix[i as usize][j as usize] = cell;
            }
        }
        JsonGame {
            current: game.current_piece.piece_type.into(),
            hold: game.hold_piece.map(|x| x.into()),
            queue: game.queue_pieces.iter().map(|x| (*x).into()).collect(),
            matrix,
        }
    }
}
impl Game {
    pub fn serialize(&self) -> String {
        let json = JsonGame::from(*self);
        serde_json::to_string(&json).unwrap()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum JsonAiRes {
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
impl From<AiRes> for JsonAiRes {
    fn from(eval: AiRes) -> Self {
        match eval {
            AiRes::Success { moves, score } => JsonAiRes::Success {
                success: true,
                moves: moves.into_iter().map(|x| x.to_string()).collect(),
                score,
            },
            AiRes::Fail { reason } => JsonAiRes::Fail {
                success: false,
                reason,
            },
        }
    }
}
impl AiRes {
    pub fn serialize(self) -> String {
        let output = JsonAiRes::from(self);
        serde_json::to_string(&output).unwrap()
    }
}
