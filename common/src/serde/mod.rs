use crate::{Board, Evaluation, GameAction, BOARD_HEIGHT, BOARD_WIDTH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct SerializedBoard([[u8; BOARD_HEIGHT]; BOARD_WIDTH]);
impl From<SerializedBoard> for Board {
    fn from(ser: SerializedBoard) -> Self {
        let mut board = Board::new();
        for (i, col) in ser.0.iter().enumerate() {
            for (j, cell) in col.iter().enumerate() {
                let val = match cell {
                    0 => false,
                    _ => true,
                };
                board.set(i, j, val);
            }
        }
        board
    }
}
impl From<Board> for SerializedBoard {
    fn from(board: Board) -> Self {
        let mut arr = [[0u8; BOARD_HEIGHT]; BOARD_WIDTH];
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                arr[i][j] = match board.get(i, j) {
                    false => 0,
                    true => 1,
                };
            }
        }
        SerializedBoard(arr)
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
// Tagged version of AiResult
pub enum SerializedEvaluation {
    Success {
        success: bool,
        actions: Vec<GameAction>,
        score: Option<f64>,
    },
    Fail {
        success: bool,
        message: String,
    },
}

impl From<Evaluation> for SerializedEvaluation {
    fn from(ai_res: Evaluation) -> Self {
        match ai_res {
            Evaluation::Success { actions, score } => SerializedEvaluation::Success {
                success: true,
                actions,
                score,
            },
            Evaluation::Fail { message } => SerializedEvaluation::Fail {
                success: false,
                message,
            },
        }
    }
}

impl From<SerializedEvaluation> for Evaluation {
    fn from(value: SerializedEvaluation) -> Self {
        match value {
            SerializedEvaluation::Success { actions, score, .. } => {
                Evaluation::Success { actions, score }
            }
            SerializedEvaluation::Fail { message, .. } => Evaluation::Fail { message },
        }
    }
}
