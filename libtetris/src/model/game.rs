use crate::model::board::Board;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use crate::model::Bag;
use crate::LockInfo;
use crate::PieceQueue;
use crate::PIECE_QUEUE_MAX_LEN;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

/// An action that can modify the game state
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum Action {
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

impl Action {
    /// Convert a u8 to game action
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Action::ShiftLeft),
            1 => Ok(Action::ShiftRight),
            2 => Ok(Action::ShiftDown),
            3 => Ok(Action::RotateCw),
            4 => Ok(Action::Rotate180),
            5 => Ok(Action::RotateCcw),
            6 => Ok(Action::SoftDrop),
            7 => Ok(Action::HardDrop),
            8 => Ok(Action::Hold),
            9 => Ok(Action::Lock),
            x => Err(anyhow!("unknown game action {x}")),
        }
    }

    /// Convert the game action to u8 representation
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::ShiftLeft
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Action::ShiftLeft => "shift-left",
            Action::ShiftRight => "shift-right",
            Action::ShiftDown => "shift-down",
            Action::RotateCw => "rotate-cw",
            Action::Rotate180 => "rotate-180",
            Action::RotateCcw => "rotate-ccw",
            Action::SoftDrop => "soft-drop",
            Action::HardDrop => "hard-drop",
            Action::Hold => "hold",
            Action::Lock => "lock",
        };
        write!(f, "{str}")
    }
}

/// Information after performing an action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionInfo {
    Success,
    Lock(LockInfo),
    Fail,
}

impl From<bool> for ActionInfo {
    fn from(value: bool) -> Self {
        match value {
            true => ActionInfo::Success,
            false => ActionInfo::Fail,
        }
    }
}

impl From<Option<LockInfo>> for ActionInfo {
    fn from(value: Option<LockInfo>) -> Self {
        match value {
            Some(lock_info) => ActionInfo::Lock(lock_info),
            None => ActionInfo::Success,
        }
    }
}

/// Represents a tetris game board
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<PieceType>,
    pub queue: PieceQueue,
    pub can_hold: bool,
}

impl Game {
    /// Construct a game from individual parts
    pub fn from_parts(
        board: Board,
        active: Piece,
        hold: Option<PieceType>,
        queue: &[PieceType],
        can_hold: bool,
    ) -> Self {
        assert!(queue.len() <= PIECE_QUEUE_MAX_LEN as usize);
        let mut piece_queue = PieceQueue::new();
        for &piece_type in queue {
            piece_queue.enqueue(piece_type);
        }

        Game {
            board,
            active,
            hold,
            queue: piece_queue,
            can_hold,
        }
    }

    /// Create a game from a given bag
    pub fn from_bag(bag: &mut Bag) -> Self {
        let active = Piece::from_piece_type(bag.next());
        let mut queue = PieceQueue::new();
        while queue.len() < PIECE_QUEUE_MAX_LEN {
            queue.enqueue(bag.next());
        }
        Game {
            board: Board::new(),
            active,
            hold: None,
            queue,
            can_hold: true,
        }
    }

    /// Create a game from a list of pieces
    pub fn from_pieces(active: PieceType, hold: Option<PieceType>, queue: &[PieceType]) -> Self {
        assert!(queue.len() <= PIECE_QUEUE_MAX_LEN as usize);
        let mut piece_queue = PieceQueue::new();
        for &piece_type in queue {
            piece_queue.enqueue(piece_type);
        }

        Game {
            board: Board::new(),
            active: Piece::from_piece_type(active),
            hold,
            queue: piece_queue,
            can_hold: true,
        }
    }

    /// Refill the game's queue with the given bag
    pub fn refill_queue(&mut self, bag: &mut Bag) {
        while self.queue.len() < PIECE_QUEUE_MAX_LEN {
            self.queue.enqueue(bag.next());
        }
    }

    pub fn swap_hold(&mut self) -> bool {
        if !self.can_hold {
            return false;
        }
        let hold = match self.hold {
            Some(hold) => hold,
            None => match self.queue.dequeue() {
                Some(piece) => piece,
                None => return false,
            },
        };
        self.hold = Some(self.active.piece_type);
        self.active.piece_type = hold;
        self.active.reset();
        self.can_hold = false;
        true
    }

    pub fn lock(&mut self) -> ActionInfo {
        if self.queue.is_empty() {
            return ActionInfo::Fail;
        }

        let info = self.board.lock(&self.active);
        self.active.piece_type = self.queue.dequeue().unwrap();
        self.active.reset();
        self.can_hold = true;

        ActionInfo::Lock(info)
    }

    pub fn hard_drop(&mut self) -> ActionInfo {
        if self.queue.is_empty() {
            return ActionInfo::Fail;
        }

        self.active.soft_drop(&self.board);
        self.lock()
    }

    pub fn apply(&mut self, action: Action) -> ActionInfo {
        match action {
            Action::ShiftLeft => self.active.shift_left(&self.board).into(),
            Action::ShiftRight => self.active.shift_right(&self.board).into(),
            Action::ShiftDown => self.active.shift_down(&self.board).into(),
            Action::RotateCw => self.active.rotate_cw(&self.board).into(),
            Action::Rotate180 => self.active.rotate_180(&self.board).into(),
            Action::RotateCcw => self.active.rotate_ccw(&self.board).into(),
            Action::SoftDrop => self.active.soft_drop(&self.board).into(),
            Action::HardDrop => self.hard_drop(),
            Action::Hold => self.swap_hold().into(),
            Action::Lock => self.lock(),
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
                let can_hold = if self.can_hold { "-" } else { "X" };
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
