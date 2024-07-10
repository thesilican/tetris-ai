use crate::{Action, Board, Evaluation, PieceQueue, PieceType, BOARD_HEIGHT, BOARD_WIDTH};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Serialized version of PieceQueue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SerializedPieceQueue(Vec<PieceType>);

impl TryFrom<SerializedPieceQueue> for PieceQueue {
    type Error = anyhow::Error;
    fn try_from(value: SerializedPieceQueue) -> Result<Self> {
        let mut queue = PieceQueue::new();
        for piece_type in value.0 {
            queue.enqueue(piece_type);
        }
        Ok(queue)
    }
}

impl From<PieceQueue> for SerializedPieceQueue {
    fn from(mut value: PieceQueue) -> Self {
        let mut vec = Vec::new();
        while value.len() > 0 {
            vec.push(value.dequeue().unwrap());
        }
        SerializedPieceQueue(vec)
    }
}

/// Serialized version of game board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SerializedBoard {
    matrix: Vec<char>,
}

impl TryFrom<SerializedBoard> for Board {
    type Error = anyhow::Error;

    fn try_from(ser: SerializedBoard) -> Result<Self> {
        if ser.matrix.len() < BOARD_WIDTH * BOARD_HEIGHT {
            bail!(
                "expected len >={}, got {}",
                BOARD_WIDTH * BOARD_HEIGHT,
                ser.matrix.len()
            );
        }

        let mut board = Board::new();
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let cell = ser.matrix[j * BOARD_WIDTH + i];
                let val = cell != ' ';
                board.set(i, j, val);
            }
        }
        Ok(board)
    }
}

impl From<Board> for SerializedBoard {
    fn from(board: Board) -> Self {
        let mut matrix = Vec::new();
        for j in 0..BOARD_HEIGHT {
            for i in 0..BOARD_WIDTH {
                matrix.push(if board.get(i, j) { ' ' } else { 'G' });
            }
        }
        SerializedBoard { matrix }
    }
}

// Tagged version of Evaluation
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializedEvaluation {
    Success {
        success: bool,
        actions: Vec<Action>,
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
