use super::board::Board;
use super::GameMove;
use crate::misc::GenericErr;
use crate::model::consts::*;
use crate::model::piece_computed::PIECE_INFO;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::lazy::SyncLazy;
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceMoveRes {
    Success,
    Failed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "char")]
#[serde(into = "char")]
pub enum PieceType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

static ALL_PIECE_TYPES: SyncLazy<Vec<PieceType>> = SyncLazy::new(|| {
    vec![
        PieceType::O,
        PieceType::I,
        PieceType::T,
        PieceType::L,
        PieceType::J,
        PieceType::S,
        PieceType::Z,
    ]
});

impl PieceType {
    pub fn all() -> &'static [PieceType] {
        &ALL_PIECE_TYPES
    }
}
impl TryFrom<i32> for PieceType {
    type Error = GenericErr;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PieceType::O),
            1 => Ok(PieceType::I),
            2 => Ok(PieceType::T),
            3 => Ok(PieceType::L),
            4 => Ok(PieceType::J),
            5 => Ok(PieceType::S),
            6 => Ok(PieceType::Z),
            _ => Err("Unknown piece type".into()),
        }
    }
}
impl From<PieceType> for i32 {
    fn from(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::O => 0,
            PieceType::I => 1,
            PieceType::T => 2,
            PieceType::L => 3,
            PieceType::J => 4,
            PieceType::S => 5,
            PieceType::Z => 6,
        }
    }
}
impl FromStr for PieceType {
    type Err = GenericErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: char = s
            .to_uppercase()
            .chars()
            .next()
            .ok_or("Unable to parse piece type")?;
        c.try_into()
    }
}
impl TryFrom<char> for PieceType {
    type Error = GenericErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(PieceType::O),
            'I' => Ok(PieceType::I),
            'T' => Ok(PieceType::T),
            'L' => Ok(PieceType::L),
            'J' => Ok(PieceType::J),
            'S' => Ok(PieceType::S),
            'Z' => Ok(PieceType::Z),
            _ => Err("Unknown piece type".into()),
        }
    }
}
impl From<PieceType> for char {
    fn from(value: PieceType) -> Self {
        match value {
            PieceType::O => 'O',
            PieceType::I => 'I',
            PieceType::T => 'T',
            PieceType::L => 'L',
            PieceType::J => 'J',
            PieceType::S => 'S',
            PieceType::Z => 'Z',
        }
    }
}
impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl Default for PieceType {
    fn default() -> Self {
        PieceType::O
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceMove {
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    Rotate180,
    RotateRight,
    SoftDrop,
}
impl TryFrom<GameMove> for PieceMove {
    type Error = ();

    fn try_from(value: GameMove) -> Result<Self, Self::Error> {
        match value {
            GameMove::ShiftLeft => Ok(PieceMove::ShiftLeft),
            GameMove::ShiftRight => Ok(PieceMove::ShiftRight),
            GameMove::RotateLeft => Ok(PieceMove::RotateLeft),
            GameMove::RotateRight => Ok(PieceMove::RotateRight),
            GameMove::Rotate180 => Ok(PieceMove::Rotate180),
            GameMove::SoftDrop => Ok(PieceMove::SoftDrop),
            GameMove::Hold => Err(()),
            GameMove::HardDrop => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
    #[serde(rename = "type")]
    pub piece_type: PieceType,
    #[serde(rename = "rot")]
    pub rotation: i8,
    #[serde(rename = "loc")]
    pub location: (i8, i8),
}
// Piece info stuff
impl Piece {
    pub fn info_spawn_location(piece_type: PieceType) -> &'static (i8, i8) {
        &PIECE_INFO.spawn_locations[i32::from(piece_type) as usize]
    }
    pub fn info_shape(
        piece_type: PieceType,
        rotation: i8,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.shapes[i32::from(piece_type) as usize][rotation as usize]
    }
    pub fn info_bit_shape(
        piece_type: PieceType,
        rotation: i8,
        x_pos: i8,
    ) -> &'static [u16; PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.bit_shapes[i32::from(piece_type) as usize][rotation as usize][(x_pos
            + (PIECE_MAX_X_SHIFT as i8)
            - Piece::info_spawn_location(piece_type).0)
            as usize]
    }
    pub fn info_height_map(
        piece_type: PieceType,
        rotation: i8,
    ) -> &'static [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.height_maps[i32::from(piece_type) as usize][rotation as usize]
    }
    pub fn info_shift_bounds(piece_type: PieceType, rotation: i8) -> &'static (i8, i8) {
        &PIECE_INFO.shift_bounds[i32::from(piece_type) as usize][rotation as usize]
    }
    pub fn info_location_bounds(piece_type: PieceType, rotation: i8) -> &'static (i8, i8, i8, i8) {
        &PIECE_INFO.location_bounds[i32::from(piece_type) as usize][rotation as usize]
    }
    pub fn info_kick_table(piece_type: PieceType, from: i8, to: i8) -> &'static [(i8, i8)] {
        &PIECE_INFO.kick_table[i32::from(piece_type) as usize][from as usize][to as usize]
    }

    pub fn get_spawn_location(&self) -> &'static (i8, i8) {
        Piece::info_spawn_location(self.piece_type)
    }
    pub fn get_shape(
        &self,
        rotation: Option<i8>,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        Piece::info_shape(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_bit_shape(
        &self,
        rotation: Option<i8>,
        x_pos: Option<i8>,
    ) -> &'static [u16; PIECE_SHAPE_SIZE as usize] {
        Piece::info_bit_shape(
            self.piece_type,
            rotation.unwrap_or(self.rotation),
            x_pos.unwrap_or(self.location.0),
        )
    }
    pub fn get_height_map(
        &self,
        rotation: Option<i8>,
    ) -> &'static [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        Piece::info_height_map(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_shift_bounds(&self, rotation: Option<i8>) -> &'static (i8, i8) {
        Piece::info_shift_bounds(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_location_bounds(&self, rotation: Option<i8>) -> &'static (i8, i8, i8, i8) {
        Piece::info_location_bounds(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_kick_table(&self, from: Option<i8>, to: i8) -> &'static [(i8, i8)] {
        Piece::info_kick_table(self.piece_type, from.unwrap_or(self.rotation), to)
    }
}
impl Piece {
    pub fn reset(&mut self, board: &Board) {
        self.rotation = 0;
        self.location = *self.get_spawn_location();
        self.shift_down(&board);
    }
    pub fn rotate(&mut self, amount: i8, board: &Board) -> PieceMoveRes {
        let (old_x, old_y) = self.location;
        let old_rot = self.rotation;
        let new_rot = (self.rotation + amount) % 4;
        self.rotation = new_rot;

        let kick_table = self.get_kick_table(Some(old_rot), new_rot);
        let (b_left, b_right, b_bottom, b_top) = *self.get_location_bounds(None);
        for (d_x, d_y) in kick_table {
            let new_x = old_x + *d_x;
            let new_y = old_y + *d_y;
            self.location = (new_x, new_y);

            if !(new_x < b_left || new_x > b_right || new_y < b_bottom || new_y > b_top)
                && !board.intersects_with(&self)
            {
                return PieceMoveRes::Success;
            }
        }
        self.rotation = old_rot;
        self.location = (old_x, old_y);
        PieceMoveRes::Failed
    }
    pub fn rotate_right(&mut self, board: &Board) -> PieceMoveRes {
        self.rotate(1, &board)
    }
    pub fn rotate_180(&mut self, board: &Board) -> PieceMoveRes {
        self.rotate(2, &board)
    }
    pub fn rotate_left(&mut self, board: &Board) -> PieceMoveRes {
        self.rotate(3, &board)
    }
    pub fn shift(&mut self, (d_x, d_y): (i8, i8), board: &Board) -> PieceMoveRes {
        let (old_x, old_y) = self.location;
        let new_x = old_x + d_x;
        let new_y = old_y + d_y;
        self.location = (new_x, new_y);

        let (b_left, b_right, b_bottom, b_top) = *self.get_location_bounds(None);
        if new_x < b_left
            || new_x > b_right
            || new_y < b_bottom
            || new_y > b_top
            || board.intersects_with(&self)
        {
            self.location = (old_x, old_y);
            return PieceMoveRes::Failed;
        }

        PieceMoveRes::Success
    }
    pub fn shift_left(&mut self, board: &Board) -> PieceMoveRes {
        self.shift((-1, 0), board)
    }
    pub fn shift_right(&mut self, board: &Board) -> PieceMoveRes {
        self.shift((1, 0), board)
    }
    pub fn shift_down(&mut self, board: &Board) -> PieceMoveRes {
        self.shift((0, -1), board)
    }
    pub fn soft_drop(&mut self, board: &Board) -> PieceMoveRes {
        let (p_x, old_y) = self.location;
        let height_map = self.get_height_map(None);
        let mut min_drop_amount = i8::MAX;
        // Slightly optimized soft-drop algorithm
        // Effective if the piece is above the height of the board
        // Used in probably 99% of scenarios
        for i in 0..PIECE_SHAPE_SIZE {
            let (low, _) = height_map[i as usize];
            if low == -1 {
                continue;
            }
            let x = p_x + (i as i8);
            let matrix_height = board.height_map[x as usize];
            let drop_amount = old_y + low - matrix_height;
            if drop_amount < min_drop_amount {
                min_drop_amount = drop_amount;
            }
            if drop_amount < 0 {
                break;
            }
        }

        // Return if dropped any amount
        if min_drop_amount >= 0 {
            self.location.1 -= min_drop_amount;
            return if min_drop_amount != 0 {
                PieceMoveRes::Success
            } else {
                PieceMoveRes::Failed
            };
        }

        // Try to shift down once
        if let PieceMoveRes::Failed = self.shift_down(&board) {
            return PieceMoveRes::Failed;
        }
        // Keep shifting down while possible
        while let PieceMoveRes::Success = self.shift_down(&board) {}
        PieceMoveRes::Success
    }
    pub fn make_move(&mut self, piece_move: PieceMove, board: &Board) -> PieceMoveRes {
        match piece_move {
            PieceMove::ShiftLeft => self.shift_left(board),
            PieceMove::ShiftRight => self.shift_right(board),
            PieceMove::RotateLeft => self.rotate_left(board),
            PieceMove::Rotate180 => self.rotate_180(board),
            PieceMove::RotateRight => self.rotate_right(board),
            PieceMove::SoftDrop => self.soft_drop(board),
        }
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.piece_type)
    }
}
impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Self {
        Piece {
            piece_type,
            location: *Piece::info_spawn_location(piece_type),
            rotation: 0,
        }
    }
}
impl Default for Piece {
    fn default() -> Self {
        let piece_type = PieceType::default();
        Piece {
            piece_type,
            location: *Piece::info_spawn_location(piece_type),
            rotation: 0,
        }
    }
}
