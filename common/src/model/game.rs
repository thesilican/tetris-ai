use crate::misc::{ArrDeque, GenericErr};
use crate::model::board::Board;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use crate::model::BAG_LEN;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::hash::Hash;
use std::str::FromStr;

use super::piece::PieceMove;
use super::{Bag, Stream, GAME_MAX_QUEUE_LEN};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SwapHoldRes {
    Success,
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameDropInfo {
    pub lines_cleared: i32,
    pub top_out: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameMoveRes {
    SuccessNorm,
    SuccessDrop(GameDropInfo),
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameMove {
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Rotate180,
    SoftDrop,
    Hold,
    HardDrop,
}
impl FromStr for GameMove {
    type Err = GenericErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shiftLeft" => Ok(GameMove::ShiftLeft),
            "shiftRight" => Ok(GameMove::ShiftRight),
            "rotateLeft" => Ok(GameMove::RotateLeft),
            "rotateRight" => Ok(GameMove::RotateRight),
            "rotate180" => Ok(GameMove::Rotate180),
            "hold" => Ok(GameMove::Hold),
            "softDrop" => Ok(GameMove::SoftDrop),
            "hardDrop" => Ok(GameMove::HardDrop),
            _ => Err("Invalid game move".into()),
        }
    }
}
impl Display for GameMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMove::ShiftLeft => write!(f, "shiftLeft"),
            GameMove::ShiftRight => write!(f, "shiftRight"),
            GameMove::RotateLeft => write!(f, "rotateLeft"),
            GameMove::RotateRight => write!(f, "rotateRight"),
            GameMove::Rotate180 => write!(f, "rotate180"),
            GameMove::Hold => write!(f, "hold"),
            GameMove::SoftDrop => write!(f, "softDrop"),
            GameMove::HardDrop => write!(f, "hardDrop"),
        }
    }
}
impl From<PieceMove> for GameMove {
    fn from(val: PieceMove) -> Self {
        match val {
            PieceMove::ShiftLeft => GameMove::ShiftLeft,
            PieceMove::ShiftRight => GameMove::ShiftRight,
            PieceMove::RotateLeft => GameMove::RotateLeft,
            PieceMove::Rotate180 => GameMove::Rotate180,
            PieceMove::RotateRight => GameMove::RotateRight,
            PieceMove::SoftDrop => GameMove::SoftDrop,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    #[serde(rename = "current")]
    pub current_piece: Piece,
    #[serde(rename = "hold")]
    pub hold_piece: Option<PieceType>,
    #[serde(rename = "queue")]
    pub queue_pieces: ArrDeque<PieceType, GAME_MAX_QUEUE_LEN>,
    #[serde(rename = "canHold")]
    pub can_hold: bool,
}
impl Game {
    pub fn from_bag(bag: &Bag) -> Self {
        let mut iter = bag.pieces().iter();
        Game {
            board: Board::new(),
            current_piece: Piece::from(*iter.next().unwrap()),
            hold_piece: None,
            queue_pieces: iter.copied().collect(),
            can_hold: true,
        }
    }
    pub fn from_bag_shuffled(bag: &mut Bag) -> Self {
        bag.shuffle();
        Game::from_bag(bag)
    }
    pub fn from_pieces(
        current_piece: PieceType,
        hold_piece: Option<PieceType>,
        queue_pieces: &[PieceType],
    ) -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::from(current_piece),
            hold_piece,
            queue_pieces: queue_pieces.into_iter().collect(),
            can_hold: true,
        }
    }
    pub fn from_stream(queue: &mut Stream) -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::from(queue.dequeue().unwrap()),
            hold_piece: None,
            queue_pieces: {
                let mut arr = ArrDeque::new();
                while arr.len() < GAME_MAX_QUEUE_LEN {
                    arr.push_back(queue.dequeue().unwrap());
                }
                arr
            },
            can_hold: true,
        }
    }

    pub fn set_current(&mut self, piece: PieceType) {
        self.current_piece.piece_type = piece;
        self.current_piece.reset(&self.board);
        self.can_hold = true;
    }
    pub fn set_hold(&mut self, piece: Option<PieceType>) {
        self.hold_piece = piece;
        self.can_hold = true;
    }
    pub fn set_queue(&mut self, pieces: &[PieceType]) {
        self.clear_queue();
        self.extend_queue(pieces);
    }
    pub fn append_queue(&mut self, piece: PieceType) {
        self.queue_pieces.push_back(piece);
    }
    pub fn extend_queue(&mut self, pieces: &[PieceType]) {
        self.queue_pieces.extend(pieces);
    }
    pub fn clear_queue(&mut self) {
        self.queue_pieces.clear();
    }
    pub fn refill_queue(&mut self, bag: &Bag) {
        const THRESHOLD: usize = GAME_MAX_QUEUE_LEN - BAG_LEN;
        if self.queue_pieces.len() <= THRESHOLD {
            self.extend_queue(bag.pieces());
        }
    }
    pub fn refill_queue_shuffled(&mut self, bag: &mut Bag) {
        const THRESHOLD: usize = GAME_MAX_QUEUE_LEN - BAG_LEN;
        if self.queue_pieces.len() <= THRESHOLD {
            bag.shuffle();
            self.extend_queue(bag.pieces());
        }
    }
    pub fn refill_queue_stream(&mut self, stream: &mut Stream) {
        while self.queue_pieces.len() < GAME_MAX_QUEUE_LEN && stream.len() > 0 {
            self.queue_pieces.push_back(stream.dequeue().unwrap());
        }
    }
    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }
    pub fn swap_hold(&mut self) -> SwapHoldRes {
        let hold = match self.hold_piece {
            Some(hold) => hold,
            None => match self.queue_pieces.pop_front() {
                Some(piece) => piece,
                None => return SwapHoldRes::Failed,
            },
        };
        self.hold_piece = Some(self.current_piece.piece_type);
        self.current_piece.piece_type = hold;
        self.current_piece.reset(&self.board);
        SwapHoldRes::Success
    }

    pub fn make_move(&mut self, game_move: GameMove) -> GameMoveRes {
        match game_move {
            GameMove::ShiftLeft
            | GameMove::ShiftRight
            | GameMove::RotateLeft
            | GameMove::Rotate180
            | GameMove::RotateRight
            | GameMove::SoftDrop => {
                let piece_move = game_move.try_into().unwrap();
                self.current_piece.make_move(piece_move, &self.board);
                GameMoveRes::SuccessNorm
            }
            GameMove::Hold => {
                if !self.can_hold {
                    return GameMoveRes::Failed;
                }
                match self.swap_hold() {
                    SwapHoldRes::Success => {
                        self.can_hold = false;
                        GameMoveRes::SuccessNorm
                    }
                    SwapHoldRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::HardDrop => {
                if self.queue_pieces.len() == 0 {
                    return GameMoveRes::Failed;
                }

                self.current_piece.soft_drop(&self.board);
                let res = self.board.lock(&self.current_piece);
                self.current_piece.piece_type = self.queue_pieces.pop_front().unwrap();
                self.current_piece.reset(&self.board);
                self.can_hold = true;

                GameMoveRes::SuccessDrop(GameDropInfo {
                    lines_cleared: res.lines_cleared,
                    top_out: res.top_out,
                })
            }
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Board + Current Piece
        let piece = &self.current_piece;
        let piece_shape = piece.get_shape(None);
        let (p_x, p_y) = piece.location;
        let (p_x, p_y) = (p_x as i32, p_y as i32);
        for j in (0..BOARD_HEIGHT).rev() {
            for i in 0..BOARD_WIDTH {
                let in_piece_bounds = i - p_x >= 0
                    && i - p_x < PIECE_SHAPE_SIZE
                    && j - p_y >= 0
                    && j - p_y < PIECE_SHAPE_SIZE;
                let in_piece =
                    in_piece_bounds && piece_shape[(i - p_x) as usize][(j - p_y) as usize];

                if in_piece {
                    // write!(f, "██")?;
                    write!(f, "[]")?;
                } else if self.board.get(i, j) {
                    write!(f, "[]")?;
                } else if in_piece_bounds {
                    write!(f, "▒▒")?;
                } else {
                    write!(f, "░░")?;
                }
            }
            writeln!(f)?;
        }
        // Board height info
        for i in 0..BOARD_WIDTH {
            let height = self.board.height_map[i as usize];
            write!(f, "{:2}", height)?;
        }
        writeln!(f)?;

        // Curr, Hold, and Queue pieces
        let curr = format!("{}", &self.current_piece);
        let hold = match &self.hold_piece {
            Some(piece) => {
                let can_hold = if self.can_hold { "✓" } else { "✗" };
                format!("{0} {1}", piece, can_hold)
            }
            None => format!(""),
        };
        const MAX_QUEUE_DISPLAY: usize = 8;
        let queue_text = {
            let mut text = self
                .queue_pieces
                .iter()
                .take(MAX_QUEUE_DISPLAY)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if self.queue_pieces.len() > MAX_QUEUE_DISPLAY {
                let amount = self.queue_pieces.len() - MAX_QUEUE_DISPLAY;
                write!(text, " +{}", amount)?;
            }
            text
        };
        write!(f, "[{1}] ({0}) {2}", curr, hold, queue_text)?;

        Ok(())
    }
}
