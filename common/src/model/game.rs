use crate::model::board::Board;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use crate::model::{Bag, GAME_MAX_QUEUE_LEN};
use crate::util::ArrDeque;
use crate::LockInfo;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GameAction {
    ShiftLeft,
    ShiftRight,
    ShiftDown,
    RotateCw,
    Rotate180,
    RotateCcw,
    SoftDrop,
    HardDrop,
    Hold,
    Lock,
}

impl GameAction {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(GameAction::ShiftLeft),
            1 => Ok(GameAction::ShiftRight),
            2 => Ok(GameAction::ShiftDown),
            3 => Ok(GameAction::RotateCw),
            4 => Ok(GameAction::Rotate180),
            5 => Ok(GameAction::RotateCcw),
            6 => Ok(GameAction::SoftDrop),
            7 => Ok(GameAction::HardDrop),
            8 => Ok(GameAction::Hold),
            9 => Ok(GameAction::Lock),
            x => Err(anyhow!("unknown game action {x}")),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            GameAction::ShiftLeft => 0,
            GameAction::ShiftRight => 1,
            GameAction::ShiftDown => 2,
            GameAction::RotateCw => 3,
            GameAction::Rotate180 => 4,
            GameAction::RotateCcw => 5,
            GameAction::SoftDrop => 6,
            GameAction::HardDrop => 7,
            GameAction::Hold => 8,
            GameAction::Lock => 9,
        }
    }
}

impl Default for GameAction {
    fn default() -> Self {
        GameAction::ShiftLeft
    }
}

impl Display for GameAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GameAction::ShiftLeft => "shift-left",
            GameAction::ShiftRight => "shift-right",
            GameAction::ShiftDown => "shift-down",
            GameAction::RotateCw => "rotate-cw",
            GameAction::Rotate180 => "rotate-180",
            GameAction::RotateCcw => "rotate-ccw",
            GameAction::SoftDrop => "soft-drop",
            GameAction::HardDrop => "hard-drop",
            GameAction::Hold => "hold",
            GameAction::Lock => "lock",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<PieceType>,
    pub queue: ArrDeque<PieceType, GAME_MAX_QUEUE_LEN>,
    #[serde(rename = "canHold")]
    pub can_hold: bool,
}

impl Game {
    pub fn from_parts(
        board: Board,
        active: Piece,
        hold: Option<PieceType>,
        queue: &[PieceType],
        can_hold: bool,
    ) -> Self {
        let queue = queue.iter().copied().collect();

        Game {
            board,
            active,
            hold,
            queue,
            can_hold,
        }
    }

    pub fn from_bag(bag: &mut Bag) -> Self {
        let active = Piece::from_piece_type(bag.next());
        let mut queue = ArrDeque::new();
        while queue.len() < GAME_MAX_QUEUE_LEN {
            queue.push_back(bag.next()).unwrap();
        }
        Game {
            board: Board::new(),
            active,
            hold: None,
            queue,
            can_hold: true,
        }
    }

    pub fn from_pieces(active: PieceType, hold: Option<PieceType>, queue: &[PieceType]) -> Self {
        Game {
            board: Board::new(),
            active: Piece::from_piece_type(active),
            hold,
            queue: queue.iter().copied().collect(),
            can_hold: true,
        }
    }

    pub fn set_active(&mut self, piece: PieceType) {
        self.active.piece_type = piece;
        self.active.reset();
        self.can_hold = true;
    }

    pub fn set_hold(&mut self, piece: Option<PieceType>) {
        self.hold = piece;
        self.can_hold = true;
    }

    pub fn set_queue(&mut self, pieces: &[PieceType]) {
        self.clear_queue();
        self.extend_queue(pieces);
    }

    pub fn append_queue(&mut self, piece: PieceType) {
        self.queue.push_back(piece).unwrap();
    }

    pub fn extend_queue(&mut self, pieces: &[PieceType]) {
        for &piece in pieces {
            if self.queue.len() < GAME_MAX_QUEUE_LEN {
                self.queue.push_back(piece).unwrap();
            }
        }
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    pub fn refill_queue(&mut self, bag: &mut Bag) {
        while self.queue.len() < GAME_MAX_QUEUE_LEN {
            self.append_queue(bag.next());
        }
    }

    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }

    pub fn swap_hold(&mut self) -> bool {
        if !self.can_hold {
            return false;
        }
        let hold = match self.hold {
            Some(hold) => hold,
            None => match self.queue.pop_front() {
                Some(piece) => piece,
                None => return false,
            },
        };
        self.hold = Some(self.active.piece_type);
        self.active.piece_type = hold;
        self.active.reset();
        self.can_hold = true;
        true
    }

    pub fn lock(&mut self) -> Option<LockInfo> {
        if self.queue.is_empty() {
            return None;
        }

        let info = self.board.lock(&self.active);
        self.active.piece_type = self.queue.pop_front().unwrap();
        self.active.reset();
        self.can_hold = true;

        Some(info)
    }

    pub fn hard_drop(&mut self) -> Option<LockInfo> {
        if self.queue.is_empty() {
            return None;
        }

        self.active.soft_drop(&self.board);
        self.lock()
    }

    pub fn apply_action(&mut self, game_action: GameAction) -> bool {
        match game_action {
            GameAction::ShiftLeft => self.active.shift_left(&self.board),
            GameAction::ShiftRight => self.active.shift_right(&self.board),
            GameAction::ShiftDown => self.active.shift_down(&self.board),
            GameAction::RotateCw => self.active.rotate_cw(&self.board),
            GameAction::Rotate180 => self.active.rotate_180(&self.board),
            GameAction::RotateCcw => self.active.rotate_ccw(&self.board),
            GameAction::SoftDrop => self.active.soft_drop(&self.board),
            GameAction::HardDrop => self.hard_drop().is_some(),
            GameAction::Hold => self.swap_hold(),
            GameAction::Lock => self.lock().is_some(),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Board + Active Piece
        write!(f, "{}", self.board.to_string(Some(&self.active)))?;

        // Current, Hold, and Queue pieces
        let curr = format!("{}", &self.active);
        let hold = match &self.hold {
            Some(piece) => {
                let can_hold = if self.can_hold { "✓" } else { "✗" };
                format!("{piece} {can_hold}")
            }
            None => String::new(),
        };
        let queue_text = self
            .queue
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "[{hold}] {curr}\n{queue_text}")?;

        Ok(())
    }
}
