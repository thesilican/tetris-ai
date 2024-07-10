use super::board::Board;
use crate::PieceInfo;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PieceType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl PieceType {
    pub const ALL: [PieceType; 7] = [
        PieceType::O,
        PieceType::I,
        PieceType::T,
        PieceType::L,
        PieceType::J,
        PieceType::S,
        PieceType::Z,
    ];

    pub fn to_u8(self) -> u8 {
        match self {
            PieceType::O => 0,
            PieceType::I => 1,
            PieceType::T => 2,
            PieceType::L => 3,
            PieceType::J => 4,
            PieceType::S => 5,
            PieceType::Z => 6,
        }
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(PieceType::O),
            1 => Ok(PieceType::I),
            2 => Ok(PieceType::T),
            3 => Ok(PieceType::L),
            4 => Ok(PieceType::J),
            5 => Ok(PieceType::S),
            6 => Ok(PieceType::Z),
            x => Err(anyhow!("unknown piece type {x}")),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char = match self {
            PieceType::O => 'O',
            PieceType::I => 'I',
            PieceType::T => 'T',
            PieceType::L => 'L',
            PieceType::J => 'J',
            PieceType::S => 'S',
            PieceType::Z => 'Z',
        };
        write!(f, "{char}")
    }
}

impl Default for PieceType {
    fn default() -> Self {
        PieceType::O
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: i8,
    pub position_x: i8,
    pub position_y: i8,
}

impl Piece {
    pub fn from_piece_type(piece_type: PieceType) -> Self {
        let (position_x, position_y) = PieceInfo::spawn_location(piece_type);
        Piece {
            piece_type,
            rotation: 0,
            position_x,
            position_y,
        }
    }

    pub fn reset(&mut self) {
        self.rotation = 0;
        (self.position_x, self.position_y) = PieceInfo::spawn_location(self.piece_type);
    }

    pub fn rotate(&mut self, amount: i8, board: &Board) -> bool {
        let old_x = self.position_x;
        let old_y = self.position_y;
        let old_rot = self.rotation;
        let new_rot = (self.rotation + amount) % 4;
        self.rotation = new_rot;

        let kick_table = PieceInfo::kick_table(self.piece_type, old_rot, new_rot);
        let (b_left, b_right, b_bottom, b_top) =
            PieceInfo::location_bound(self.piece_type, new_rot);
        for (d_x, d_y) in kick_table {
            let new_x = old_x + d_x;
            let new_y = old_y + d_y;
            self.position_x = new_x;
            self.position_y = new_y;

            if !(new_x < b_left || new_x > b_right || new_y < b_bottom || new_y > b_top)
                && !board.intersects_with(self)
            {
                return true;
            }
        }
        self.rotation = old_rot;
        self.position_x = old_x;
        self.position_y = old_y;
        false
    }

    pub fn rotate_cw(&mut self, board: &Board) -> bool {
        self.rotate(1, board)
    }

    pub fn rotate_180(&mut self, board: &Board) -> bool {
        self.rotate(2, board)
    }

    pub fn rotate_ccw(&mut self, board: &Board) -> bool {
        self.rotate(3, board)
    }

    pub fn shift(&mut self, d_x: i8, d_y: i8, board: &Board) -> bool {
        let old_x = self.position_x;
        let old_y = self.position_y;
        let new_x = old_x + d_x;
        let new_y = old_y + d_y;
        self.position_x = new_x;
        self.position_y = new_y;

        let (b_left, b_right, b_bottom, b_top) =
            PieceInfo::location_bound(self.piece_type, self.rotation);
        if new_x < b_left
            || new_x > b_right
            || new_y < b_bottom
            || new_y > b_top
            || board.intersects_with(self)
        {
            self.position_x = old_x;
            self.position_y = old_y;
            return false;
        }

        true
    }

    pub fn shift_left(&mut self, board: &Board) -> bool {
        self.shift(-1, 0, board)
    }

    pub fn shift_right(&mut self, board: &Board) -> bool {
        self.shift(1, 0, board)
    }

    pub fn shift_down(&mut self, board: &Board) -> bool {
        self.shift(0, -1, board)
    }

    pub fn soft_drop(&mut self, board: &Board) -> bool {
        let old_y = self.position_y;

        // Optimization with board height
        let min_drop_amount = old_y - board.max_height() as i8;
        if min_drop_amount > 0 {
            self.position_y -= min_drop_amount;
        } else {
            // Try to shift down once
            if !self.shift_down(board) {
                return false;
            }
        }
        // Keep shifting down while possible
        while self.shift_down(board) {}
        true
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}} {} ({},{})",
            self.piece_type, self.rotation, self.position_x, self.position_y
        )
    }
}

impl Default for Piece {
    fn default() -> Self {
        let piece_type = PieceType::default();
        Piece::from_piece_type(piece_type)
    }
}
