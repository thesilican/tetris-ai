// use crate::model::consts::BOARD_HEIGHT;
// use crate::model::consts::BOARD_VISIBLE_HEIGHT;
use crate::model::board::Board;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_MAX_X_SHIFT;
use crate::model::consts::PIECE_NUM_ROTATION;
use crate::model::consts::PIECE_NUM_TYPES;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::consts::PIECE_STARTING_COLUMN;
use crate::model::game::GameMove;
use lazy_static::lazy_static;

// For convenience purposes
const PIECE_NUM_TYPES_USIZE: usize = PIECE_NUM_TYPES as usize;
const PIECE_NUM_ROTATION_USIZE: usize = PIECE_NUM_ROTATION as usize;
const PIECE_MAX_X_SHIFT_USIZE: usize = PIECE_MAX_X_SHIFT as usize;
const PIECE_SHAPE_SIZE_USIZE: usize = PIECE_SHAPE_SIZE as usize;
// const BOARD_WIDTH_USIZE: usize = BOARD_WIDTH as usize;
// const BOARD_HEIGHT_USIZE: usize = BOARD_HEIGHT as usize;
// const BOARD_VISIBLE_HEIGHT_USIZE: usize = BOARD_VISIBLE_HEIGHT as usize;

lazy_static! {
    pub static ref PIECE_INFO: PieceInfo = PieceInfo::new();
}

pub struct PieceInfo {
    pub sizes: [i32; PIECE_NUM_TYPES_USIZE],
    pub starting_positions: [(i32, i32); PIECE_NUM_TYPES_USIZE],
    pub shapes: [[[[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE];
        PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE],
    pub bit_shapes: [[[[u16; PIECE_SHAPE_SIZE_USIZE]; (PIECE_MAX_X_SHIFT_USIZE * 2) + 1];
        PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE],
    /// Lows and Heights (Height from bottom to first block, then height of blocks)
    /// Both fields are -1 if that column is empty
    pub height_maps:
        [[[(i32, i32); PIECE_SHAPE_SIZE_USIZE]; PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE],
    /// Maximum shifts for a piece (min x, max x, min y, max y)
    pub shift_bounds: [[(i32, i32, i32, i32); PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE],
    pub kick_table: [[[Vec<(i32, i32)>; PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_ROTATION_USIZE];
        PIECE_NUM_TYPES_USIZE],
}
impl PieceInfo {
    fn new() -> Self {
        fn rotate_shape(
            arr: [[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE],
            size: i32,
        ) -> [[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE] {
            let mut new_arr = [[false; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE];
            for i in 0..size {
                for j in 0..size {
                    new_arr[j as usize][(size - 1 - i) as usize] = arr[i as usize][j as usize];
                }
            }
            new_arr
        }

        let sizes = [2, 4, 3, 3, 3, 3, 3];
        let starting_positions = [
            (PIECE_STARTING_COLUMN, 19),
            (PIECE_STARTING_COLUMN, 18),
            (PIECE_STARTING_COLUMN, 19),
            (PIECE_STARTING_COLUMN, 19),
            (PIECE_STARTING_COLUMN, 19),
            (PIECE_STARTING_COLUMN, 19),
            (PIECE_STARTING_COLUMN, 19),
        ];
        let base_shapes = [
            // O
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            // I
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            // T
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            // L
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            // J
            [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            // S
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            // Z
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ];
        let mut shapes: [[[[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE];
            PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE] = Default::default();
        for piece in 0..PIECE_NUM_TYPES {
            let size = sizes[piece as usize];
            let mut shape = base_shapes[piece as usize];
            for rotation in 0..PIECE_NUM_ROTATION {
                shapes[piece as usize][rotation as usize] = shape.clone();
                // Don't rotate O shape
                if piece != 0 {
                    shape = rotate_shape(shape, size);
                }
            }
        }

        // You have to be careful when doing bit shapes, as they're kinda backwards
        // Typically the LSB is written on the right side, but in this case
        // Bit 0 represents the left-most bit of the matrix
        let mut bit_shapes: [[[[u16; PIECE_SHAPE_SIZE_USIZE]; (PIECE_MAX_X_SHIFT_USIZE * 2) + 1];
            PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_TYPES_USIZE] = Default::default();
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = shapes[piece as usize][rotation as usize];
                for s in 0..(PIECE_MAX_X_SHIFT * 2) + 1 {
                    let mut bit_shape = [0u16; PIECE_SHAPE_SIZE_USIZE];
                    for i in 0..PIECE_SHAPE_SIZE {
                        for j in 0..PIECE_SHAPE_SIZE {
                            if !shape[i as usize][j as usize] {
                                continue;
                            }
                            let x = PIECE_STARTING_COLUMN - PIECE_MAX_X_SHIFT + s + i;
                            if x < 0 || x >= BOARD_WIDTH {
                                continue;
                            }
                            bit_shape[j as usize] |= 1 << x;
                        }
                    }
                    bit_shapes[piece as usize][rotation as usize][s as usize] = bit_shape;
                }
            }
        }

        // Calculate height maps and shift bounds
        let mut height_maps: [[[(i32, i32); PIECE_SHAPE_SIZE_USIZE]; PIECE_NUM_ROTATION_USIZE];
            PIECE_NUM_TYPES_USIZE] = Default::default();
        let mut shift_bounds: [[(i32, i32, i32, i32); PIECE_NUM_ROTATION_USIZE];
            PIECE_NUM_TYPES_USIZE] = Default::default();
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = shapes[piece as usize][rotation as usize];
                let bit_shape =
                    bit_shapes[piece as usize][rotation as usize][PIECE_MAX_X_SHIFT_USIZE];
                let mut height_map = [(-1, -1); PIECE_SHAPE_SIZE_USIZE];
                for i in 0..PIECE_SHAPE_SIZE {
                    for j in 0..PIECE_SHAPE_SIZE {
                        if shape[i as usize][j as usize] {
                            if height_map[i as usize].0 == -1 {
                                height_map[i as usize] = (j, 1);
                            } else {
                                height_map[i as usize].1 += 1;
                            }
                        }
                    }
                }
                height_maps[piece as usize][rotation as usize] = height_map;

                let mut left = 0;
                for i in 0..PIECE_SHAPE_SIZE {
                    if height_map[i as usize].0 == -1 {
                        left -= 1
                    } else {
                        break;
                    }
                }
                let mut right = BOARD_WIDTH - PIECE_SHAPE_SIZE;
                for i in (0..PIECE_SHAPE_SIZE).rev() {
                    if height_map[i as usize].0 == -1 {
                        right += 1;
                    } else {
                        break;
                    }
                }
                let mut bottom = 0;
                for j in 0..PIECE_SHAPE_SIZE {
                    if bit_shape[j as usize] == 0 {
                        bottom -= 1;
                    } else {
                        break;
                    }
                }
                let mut top = BOARD_HEIGHT - PIECE_SHAPE_SIZE;
                for j in (0..PIECE_SHAPE_SIZE).rev() {
                    if bit_shape[j as usize] == 0 {
                        top += 1;
                    } else {
                        break;
                    }
                }
                shift_bounds[piece as usize][rotation as usize] = (left, right, bottom, top);
            }
        }
        // Pain
        let o_kick_table: [[Vec<(i32, i32)>; PIECE_NUM_ROTATION_USIZE]; PIECE_NUM_ROTATION_USIZE] = [
            [vec![], vec![(0, 0)], vec![(0, 0)], vec![(0, 0)]],
            [vec![(0, 0)], vec![], vec![(0, 0)], vec![(0, 0)]],
            [vec![(0, 0)], vec![(0, 0)], vec![], vec![(0, 0)]],
            [vec![(0, 0)], vec![(0, 0)], vec![(0, 0)], vec![]],
        ];
        let i_kick_table = [
            [
                // 0 >> 0
                vec![],
                // 0 >> 1
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                // 0 >> 2
                vec![(0, 0)],
                // 0 >> 3
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
            ],
            [
                // 1 >> 0
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                // 1 >> 1
                vec![],
                // 1 >> 2
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                // 1 >> 3
                vec![(0, 0)],
            ],
            [
                // 2 >> 0
                vec![(0, 0)],
                // 2 >> 1
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                // 2 >> 2
                vec![],
                // 2 >> 3
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
            ],
            [
                // 3 >> 0
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                // 3 >> 1
                vec![(0, 0)],
                // 3 >> 2
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                // 3 >> 3
                vec![],
            ],
        ];
        let tljsz_kick_table = [
            [
                // 0 >> 0
                vec![],
                // 0 >> 1
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                // 0 >> 2
                vec![(0, 0)],
                // 0 >> 3
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            [
                // 1 >> 0
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                // 1 >> 1
                vec![],
                // 1 >> 2
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                // 1 >> 3
                vec![(0, 0)],
            ],
            [
                // 2 >> 0
                vec![(0, 0)],
                // 2 >> 1
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                // 2 >> 2
                vec![],
                // 2 >> 3
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            [
                // 3 >> 0
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                // 3 >> 1
                vec![(0, 0)],
                // 3 >> 2
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                // 3 >> 3
                vec![],
            ],
        ];
        let kick_table = [
            o_kick_table,
            i_kick_table,
            tljsz_kick_table.clone(),
            tljsz_kick_table.clone(),
            tljsz_kick_table.clone(),
            tljsz_kick_table.clone(),
            tljsz_kick_table,
        ];
        PieceInfo {
            sizes,
            starting_positions,
            shapes,
            bit_shapes,
            height_maps,
            shift_bounds,
            kick_table,
        }
    }
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
        // https://stackoverflow.com/a/21376984/7937009
        let mut pieces = Vec::new();
        for i in 0..PIECE_NUM_TYPES {
            pieces.push(PieceType::from_i32(i).unwrap());
        }
        pieces.into_iter()
    }
    pub fn from_i32(val: i32) -> Result<Self, ()> {
        match val {
            0 => Ok(PieceType::O),
            1 => Ok(PieceType::I),
            2 => Ok(PieceType::T),
            3 => Ok(PieceType::L),
            4 => Ok(PieceType::J),
            5 => Ok(PieceType::S),
            6 => Ok(PieceType::Z),
            _ => Err(()),
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
impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: i32,
    pub location: (i32, i32),
}
impl Piece {
    pub fn info_size(piece_type: &PieceType) -> &'static i32 {
        &PIECE_INFO.sizes[piece_type.to_i32() as usize]
    }
    pub fn info_starting_position(piece_type: &PieceType) -> &'static (i32, i32) {
        &PIECE_INFO.starting_positions[piece_type.to_i32() as usize]
    }
    pub fn info_shape(
        piece_type: &PieceType,
        rotation: i32,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE] {
        &PIECE_INFO.shapes[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_bit_shape(
        piece_type: &PieceType,
        rotation: i32,
        x_pos: i32,
    ) -> &'static [u16; PIECE_SHAPE_SIZE_USIZE] {
        &PIECE_INFO.bit_shapes[piece_type.to_i32() as usize][rotation as usize]
            [(x_pos + PIECE_MAX_X_SHIFT - Piece::info_starting_position(piece_type).0) as usize]
    }
    pub fn info_height_map(
        piece_type: &PieceType,
        rotation: i32,
    ) -> &'static [(i32, i32); PIECE_SHAPE_SIZE_USIZE] {
        &PIECE_INFO.height_maps[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_shift_bounds(
        piece_type: &PieceType,
        rotation: i32,
    ) -> &'static (i32, i32, i32, i32) {
        &PIECE_INFO.shift_bounds[piece_type.to_i32() as usize][rotation as usize]
    }
    pub fn info_kick_table(piece_type: &PieceType, from: i32, to: i32) -> &'static Vec<(i32, i32)> {
        &PIECE_INFO.kick_table[piece_type.to_i32() as usize][from as usize][to as usize]
    }

    pub fn get_size(&self) -> &'static i32 {
        Piece::info_size(&self.piece_type)
    }
    pub fn get_starting_position(&self) -> &'static (i32, i32) {
        Piece::info_starting_position(&self.piece_type)
    }
    pub fn get_shape(
        &self,
        rotation: Option<i32>,
    ) -> &'static [[bool; PIECE_SHAPE_SIZE_USIZE]; PIECE_SHAPE_SIZE_USIZE] {
        Piece::info_shape(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_bit_shape(
        &self,
        rotation: Option<i32>,
        x_pos: Option<i32>,
    ) -> &'static [u16; PIECE_SHAPE_SIZE_USIZE] {
        Piece::info_bit_shape(
            &self.piece_type,
            rotation.unwrap_or(self.rotation),
            x_pos.unwrap_or(self.location.0),
        )
    }
    pub fn get_height_map(
        &self,
        rotation: Option<i32>,
    ) -> &'static [(i32, i32); PIECE_SHAPE_SIZE_USIZE] {
        Piece::info_height_map(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_shift_bounds(&self, rotation: Option<i32>) -> &'static (i32, i32, i32, i32) {
        Piece::info_shift_bounds(&self.piece_type, rotation.unwrap_or(self.rotation))
    }
    pub fn get_kick_table(&self, from: Option<i32>, to: i32) -> &'static Vec<(i32, i32)> {
        Piece::info_kick_table(&self.piece_type, from.unwrap_or(self.rotation), to)
    }
    pub fn to_string(&self) -> String {
        String::from(self.piece_type.to_char())
    }
}
impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        let location = Piece::info_starting_position(&piece_type).clone();
        Piece {
            piece_type,
            rotation: 0,
            location,
        }
    }
    pub fn reset(&mut self) {
        self.rotation = 0;
        self.location = *self.get_starting_position();
    }
    pub fn rotate(&mut self, amount: i32, board: &Board) -> bool {
        let (old_x, old_y) = self.location;
        let old_rot = self.rotation;
        let new_rot = (self.rotation + amount) % 4;
        self.rotation = new_rot;

        let kick_table = self.get_kick_table(Some(old_rot), new_rot);
        let (b_left, b_right, b_bottom, b_top) = *self.get_shift_bounds(None);
        for (d_x, d_y) in kick_table {
            let new_x = old_x + *d_x;
            let new_y = old_y + *d_y;
            self.location = (new_x, new_y);

            if !(new_x < b_left || new_x > b_right || new_y < b_bottom || new_y > b_top)
                && !board.intersects_with(&self)
            {
                return true;
            }
        }
        self.rotation = old_rot;
        self.location = (old_x, old_y);
        false
    }
    pub fn rotate_right(&mut self, board: &Board) -> bool {
        self.rotate(1, &board)
    }
    pub fn rotate_180(&mut self, board: &Board) -> bool {
        self.rotate(2, &board)
    }
    pub fn rotate_left(&mut self, board: &Board) -> bool {
        self.rotate(3, &board)
    }
    pub fn shift(&mut self, (d_x, d_y): (i32, i32), board: &Board) -> bool {
        let (old_x, old_y) = self.location;
        let new_x = old_x + d_x;
        let new_y = old_y + d_y;
        self.location = (new_x, new_y);

        let (b_left, b_right, b_bottom, b_top) = *self.get_shift_bounds(None);
        if new_x < b_left
            || new_x > b_right
            || new_y < b_bottom
            || new_y > b_top
            || board.intersects_with(&self)
        {
            self.location = (old_x, old_y);
            return false;
        }

        true
    }
    pub fn shift_left(&mut self, board: &Board) -> bool {
        self.shift((-1, 0), board)
    }
    pub fn shift_right(&mut self, board: &Board) -> bool {
        self.shift((1, 0), board)
    }
    pub fn shift_down(&mut self, board: &Board) -> bool {
        self.shift((0, -1), board)
    }
    pub fn soft_drop(&mut self, board: &Board) -> bool {
        let (p_x, old_y) = self.location;
        let height_map = self.get_height_map(None);
        let mut min_drop_amount = i32::MAX;
        // Slightly optimized soft-drop algorithm
        // Effective if the piece is above the height of the board
        // Used in probably 99% of scenarios
        for i in 0..PIECE_SHAPE_SIZE {
            let (low, _) = height_map[i as usize];
            if low == -1 {
                continue;
            }
            let x = p_x + i;
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
            return min_drop_amount != 0;
        }

        // Try to shift down once
        if self.shift_down(&board) == false {
            return false;
        }
        // Keep shifting down while possible
        while self.shift_down(&board) {}
        true
    }
    pub fn make_move(&mut self, board: &Board, piece_move: &PieceMove) -> bool {
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
    pub fn to_game_move(&self) -> GameMove {
        match self {
            PieceMove::ShiftLeft => GameMove::ShiftLeft,
            PieceMove::ShiftRight => GameMove::ShiftRight,
            PieceMove::RotateLeft => GameMove::RotateLeft,
            PieceMove::RotateRight => GameMove::RotateRight,
            PieceMove::Rotate180 => GameMove::Rotate180,
            PieceMove::SoftDrop => GameMove::SoftDrop,
        }
    }
}
impl std::fmt::Display for PieceMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
