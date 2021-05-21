use crate::model::board::Board;
use crate::model::computed::PIECE_INFO;
use crate::model::consts::{PIECE_MAX_X_SHIFT, PIECE_SHAPE_SIZE};
use crate::model::game::GameMove;
use crate::{misc::GenericErr, model::consts::PIECE_NUM_TYPES};
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::hash::Hasher;

pub enum PieceMoveRes {
    Success,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub fn iter_types() -> impl Iterator<Item = PieceType> {
        let mut pieces = Vec::new();
        for i in 0..PIECE_NUM_TYPES {
            pieces.push(PieceType::from_i32(i).unwrap());
        }
        pieces.into_iter()
    }
    pub fn from_i32(val: i32) -> Result<Self, GenericErr> {
        match val {
            0 => Ok(PieceType::O),
            1 => Ok(PieceType::I),
            2 => Ok(PieceType::T),
            3 => Ok(PieceType::L),
            4 => Ok(PieceType::J),
            5 => Ok(PieceType::S),
            6 => Ok(PieceType::Z),
            _ => Err(format!("Invalid PieceType #{}", val).into()),
        }
    }
    pub fn to_i32(&self) -> i32 {
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
    pub fn to_char(&self) -> char {
        match self {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: i8,
    pub location: (i8, i8),
}
impl Piece {
    pub fn info_spawn_location(piece_type: &PieceType) -> &'static (i8, i8) {
        &PIECE_INFO.spawn_locations[piece_type.to_i32() as usize]
    }
    pub fn info_shape(
        piece_type: &PieceType,
        rotation: i8,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.shapes[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_bit_shape(
        piece_type: &PieceType,
        rotation: i8,
        x_pos: i8,
    ) -> &'static [u16; PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.bit_shapes[piece_type.to_i32() as usize][rotation as usize][(x_pos
            + (PIECE_MAX_X_SHIFT as i8)
            - Piece::info_spawn_location(piece_type).0)
            as usize]
    }
    pub fn info_height_map(
        piece_type: &PieceType,
        rotation: i8,
    ) -> &'static [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        &PIECE_INFO.height_maps[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_shift_bounds(piece_type: &PieceType, rotation: i8) -> &'static (i8, i8) {
        &PIECE_INFO.shift_bounds[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_location_bounds(piece_type: &PieceType, rotation: i8) -> &'static (i8, i8, i8, i8) {
        &PIECE_INFO.location_bounds[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_kick_table(piece_type: &PieceType, from: i8, to: i8) -> &'static Vec<(i8, i8)> {
        &PIECE_INFO.kick_table[piece_type.to_i32() as usize][from as usize][to as usize]
    }

    pub fn get_spawn_location(&self) -> &'static (i8, i8) {
        Piece::info_spawn_location(&self.piece_type)
    }
    pub fn get_shape(
        &self,
        rotation: Option<i8>,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
        Piece::info_shape(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_bit_shape(
        &self,
        rotation: Option<i8>,
        x_pos: Option<i8>,
    ) -> &'static [u16; PIECE_SHAPE_SIZE as usize] {
        Piece::info_bit_shape(
            &self.piece_type,
            rotation.unwrap_or(self.rotation),
            x_pos.unwrap_or(self.location.0),
        )
    }
    pub fn get_height_map(
        &self,
        rotation: Option<i8>,
    ) -> &'static [(i8, i8); PIECE_SHAPE_SIZE as usize] {
        Piece::info_height_map(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_shift_bounds(&self, rotation: Option<i8>) -> &'static (i8, i8) {
        Piece::info_shift_bounds(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_location_bounds(&self, rotation: Option<i8>) -> &'static (i8, i8, i8, i8) {
        Piece::info_location_bounds(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_kick_table(&self, from: Option<i8>, to: i8) -> &'static Vec<(i8, i8)> {
        Piece::info_kick_table(&self.piece_type, from.unwrap_or(self.rotation), to)
    }
    pub fn to_string(&self) -> String {
        String::from(self.piece_type.to_char())
    }
}
impl Piece {
    pub fn new(piece_type: &PieceType) -> Self {
        let location = Piece::info_spawn_location(piece_type).clone();
        Piece {
            piece_type: piece_type.clone(),
            rotation: 0,
            location,
        }
    }
    pub fn reset(&mut self) {
        self.rotation = 0;
        self.location = *self.get_spawn_location();
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
        PieceMoveRes::Failed
    }
    pub fn make_move(&mut self, board: &Board, piece_move: &PieceMove) -> PieceMoveRes {
        match piece_move {
            PieceMove::ShiftLeft => self.shift_left(board),
            PieceMove::ShiftRight => self.shift_right(board),
            PieceMove::RotateLeft => self.rotate_left(board),
            PieceMove::RotateRight => self.rotate_right(board),
            PieceMove::Rotate180 => self.rotate_180(board),
            PieceMove::SoftDrop => self.soft_drop(board),
        }
    }
}
// Only compare piece type
impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.piece_type == other.piece_type
    }
}
impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.piece_type.hash(state);
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum PieceMove {
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Rotate180,
    SoftDrop,
}
impl PieceMove {
    fn to_string(&self) -> String {
        let slice = match self {
            PieceMove::ShiftLeft => "shiftLeft",
            PieceMove::ShiftRight => "shiftRight",
            PieceMove::RotateLeft => "rotateLeft",
            PieceMove::RotateRight => "rotateRight",
            PieceMove::Rotate180 => "rotate180",
            PieceMove::SoftDrop => "softDrop",
        };
        String::from(slice)
    }
}
impl From<PieceMove> for GameMove {
    fn from(piece_move: PieceMove) -> GameMove {
        match piece_move {
            PieceMove::ShiftLeft => GameMove::ShiftLeft,
            PieceMove::ShiftRight => GameMove::ShiftRight,
            PieceMove::RotateLeft => GameMove::RotateLeft,
            PieceMove::RotateRight => GameMove::RotateRight,
            PieceMove::Rotate180 => GameMove::Rotate180,
            PieceMove::SoftDrop => GameMove::SoftDrop,
        }
    }
}
impl Display for PieceMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
