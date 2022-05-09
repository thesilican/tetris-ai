use super::board::Board;
use super::game::GameAction;
use crate::misc::GenericErr;
use crate::model::consts::*;
use crate::model::piece_computed::PIECE_INFO;
use crate::KickSeq;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::lazy::SyncLazy;
use std::{convert::TryFrom, str::FromStr};

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
    pub fn all() -> impl Iterator<Item = PieceType> {
        ALL_PIECE_TYPES.iter().map(|x| *x)
    }
    #[inline]
    pub const fn to_i8(self) -> i8 {
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
}
impl TryFrom<i8> for PieceType {
    type Error = GenericErr;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
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
impl From<PieceType> for i8 {
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
pub enum PieceAction {
    ShiftLeft,
    ShiftRight,
    ShiftDown,
    RotateCW,
    Rotate180,
    RotateCCW,
    SoftDrop,
}
impl TryFrom<GameAction> for PieceAction {
    type Error = ();

    fn try_from(value: GameAction) -> Result<Self, Self::Error> {
        match value {
            GameAction::ShiftLeft => Ok(PieceAction::ShiftLeft),
            GameAction::ShiftDown => Ok(PieceAction::ShiftDown),
            GameAction::ShiftRight => Ok(PieceAction::ShiftRight),
            GameAction::RotateCW => Ok(PieceAction::RotateCW),
            GameAction::RotateCCW => Ok(PieceAction::RotateCCW),
            GameAction::Rotate180 => Ok(PieceAction::Rotate180),
            GameAction::SoftDrop => Ok(PieceAction::SoftDrop),
            GameAction::Hold => Err(()),
            GameAction::Lock => Err(()),
            GameAction::AddGarbage { .. } => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceActionRes {
    Success,
    Failed,
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
    #[inline]
    pub const fn info_spawn_location(piece_type: PieceType) -> (i8, i8) {
        PIECE_INFO.spawn_locations[piece_type.to_i8() as usize]
    }
    #[inline]
    pub const fn info_shape(
        piece_type: PieceType,
        rotation: i8,
    ) -> [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        PIECE_INFO.shapes[piece_type.to_i8() as usize][rotation as usize]
    }
    #[inline]
    pub const fn info_bit_shape(
        piece_type: PieceType,
        rotation: i8,
        x_pos: i8,
    ) -> [u16; PIECE_SHAPE_SIZE as usize] {
        PIECE_INFO.bit_shapes[piece_type.to_i8() as usize][rotation as usize]
            [(x_pos + (PIECE_MAX_X_SHIFT as i8) - (PIECE_SPAWN_COLUMN as i8)) as usize]
    }
    #[inline]
    pub const fn info_height_map(
        piece_type: PieceType,
        rotation: i8,
    ) -> [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        PIECE_INFO.height_maps[piece_type.to_i8() as usize][rotation as usize]
    }
    #[inline]
    pub const fn info_shift_bounds(piece_type: PieceType, rotation: i8) -> (i8, i8) {
        PIECE_INFO.shift_bounds[piece_type.to_i8() as usize][rotation as usize]
    }
    #[inline]
    pub const fn info_location_bounds(piece_type: PieceType, rotation: i8) -> (i8, i8, i8, i8) {
        PIECE_INFO.location_bounds[piece_type.to_i8() as usize][rotation as usize]
    }
    #[inline]
    pub const fn info_kick_table(piece_type: PieceType, from: i8, to: i8) -> KickSeq {
        PIECE_INFO.kick_table[piece_type.to_i8() as usize][from as usize][to as usize]
    }

    #[inline]
    pub const fn get_spawn_location(&self) -> (i8, i8) {
        Piece::info_spawn_location(self.piece_type)
    }
    #[inline]
    pub const fn get_shape(
        &self,
        rotation: Option<i8>,
    ) -> [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        Piece::info_shape(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    #[inline]
    pub const fn get_bit_shape(
        &self,
        rotation: Option<i8>,
        x_pos: Option<i8>,
    ) -> [u16; PIECE_SHAPE_SIZE as usize] {
        Piece::info_bit_shape(
            self.piece_type,
            rotation.unwrap_or(self.rotation),
            x_pos.unwrap_or(self.location.0),
        )
    }
    #[inline]
    pub const fn get_height_map(
        &self,
        rotation: Option<i8>,
    ) -> [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        Piece::info_height_map(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    #[inline]
    pub const fn get_shift_bounds(&self, rotation: Option<i8>) -> (i8, i8) {
        Piece::info_shift_bounds(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    #[inline]
    pub const fn get_location_bounds(&self, rotation: Option<i8>) -> (i8, i8, i8, i8) {
        Piece::info_location_bounds(self.piece_type, rotation.unwrap_or(self.rotation))
    }
    #[inline]
    pub const fn get_kick_table(&self, from: Option<i8>, to: i8) -> KickSeq {
        Piece::info_kick_table(self.piece_type, from.unwrap_or(self.rotation), to)
    }
}
impl Piece {
    #[inline]
    pub fn reset(&mut self) {
        self.rotation = 0;
        self.location = self.get_spawn_location();
    }
    #[inline]
    pub fn rotate(&mut self, amount: i8, board: &Board) -> PieceActionRes {
        let (old_x, old_y) = self.location;
        let old_rot = self.rotation;
        let new_rot = (self.rotation + amount) % 4;
        self.rotation = new_rot;

        let kick_table = self.get_kick_table(Some(old_rot), new_rot);
        let (b_left, b_right, b_bottom, b_top) = self.get_location_bounds(None);
        for (d_x, d_y) in kick_table {
            let new_x = old_x + d_x;
            let new_y = old_y + d_y;
            self.location = (new_x, new_y);

            if !(new_x < b_left || new_x > b_right || new_y < b_bottom || new_y > b_top)
                && !board.intersects_with(&self)
            {
                return PieceActionRes::Success;
            }
        }
        self.rotation = old_rot;
        self.location = (old_x, old_y);
        PieceActionRes::Failed
    }
    pub fn rotate_cw(&mut self, board: &Board) -> PieceActionRes {
        self.rotate(1, &board)
    }
    pub fn rotate_180(&mut self, board: &Board) -> PieceActionRes {
        self.rotate(2, &board)
    }
    pub fn rotate_ccw(&mut self, board: &Board) -> PieceActionRes {
        self.rotate(3, &board)
    }
    #[inline]
    pub fn shift(&mut self, (d_x, d_y): (i8, i8), board: &Board) -> PieceActionRes {
        let (old_x, old_y) = self.location;
        let new_x = old_x + d_x;
        let new_y = old_y + d_y;
        self.location = (new_x, new_y);

        let (b_left, b_right, b_bottom, b_top) = self.get_location_bounds(None);
        if new_x < b_left
            || new_x > b_right
            || new_y < b_bottom
            || new_y > b_top
            || board.intersects_with(&self)
        {
            self.location = (old_x, old_y);
            return PieceActionRes::Failed;
        }

        PieceActionRes::Success
    }
    pub fn shift_left(&mut self, board: &Board) -> PieceActionRes {
        self.shift((-1, 0), board)
    }
    pub fn shift_right(&mut self, board: &Board) -> PieceActionRes {
        self.shift((1, 0), board)
    }
    pub fn shift_down(&mut self, board: &Board) -> PieceActionRes {
        self.shift((0, -1), board)
    }
    pub fn soft_drop(&mut self, board: &Board) -> PieceActionRes {
        let (_, old_y) = self.location;

        // Optimization with board height
        let min_drop_amount = old_y - board.max_height();
        if min_drop_amount > 0 {
            self.location.1 -= min_drop_amount;
        } else {
            // Try to shift down once
            if let PieceActionRes::Failed = self.shift_down(&board) {
                return PieceActionRes::Failed;
            }
        }
        // Keep shifting down while possible
        while let PieceActionRes::Success = self.shift_down(&board) {}
        PieceActionRes::Success
    }
    pub fn apply_action(&mut self, piece_action: PieceAction, board: &Board) -> PieceActionRes {
        match piece_action {
            PieceAction::ShiftLeft => self.shift_left(board),
            PieceAction::ShiftRight => self.shift_right(board),
            PieceAction::ShiftDown => self.shift_down(board),
            PieceAction::RotateCW => self.rotate_cw(board),
            PieceAction::Rotate180 => self.rotate_180(board),
            PieceAction::RotateCCW => self.rotate_ccw(board),
            PieceAction::SoftDrop => self.soft_drop(board),
        }
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}} {} ({},{})",
            self.piece_type, self.rotation, self.location.0, self.location.1
        )
    }
}
impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Self {
        Piece {
            piece_type,
            location: Piece::info_spawn_location(piece_type),
            rotation: 0,
        }
    }
}
impl Default for Piece {
    fn default() -> Self {
        let piece_type = PieceType::default();
        Piece {
            piece_type,
            location: Piece::info_spawn_location(piece_type),
            rotation: 0,
        }
    }
}
