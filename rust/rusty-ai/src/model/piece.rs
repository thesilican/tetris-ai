use crate::model::consts::PIECE_COLUMN;
use crate::model::consts::{BOARD_WIDTH, PIECE_MAX_ROTATION, PIECE_MAX_SIZE, PIECE_NUM_TYPES};
use lazy_static::lazy_static;

// Utility function to rotate an array
// Used by PieceInfo::new()
fn rotate_array(
    arr: &mut [[bool; PIECE_MAX_SIZE as usize]; PIECE_MAX_SIZE as usize],
    size: i32,
    offset: i32,
) {
    for layer in 0..(size / 2) {
        let first = layer + offset;
        let last = size - layer - 1 + offset;
        for element in first..last {
            let element_last = size - element - 1 + offset;
            let top = arr[first as usize][element as usize];
            let right = arr[element as usize][last as usize];
            let bottom = arr[last as usize][element_last as usize];
            let left = arr[element_last as usize][first as usize];
            arr[first as usize][element as usize] = left;
            arr[element as usize][last as usize] = top;
            arr[last as usize][element_last as usize] = right;
            arr[element_last as usize][first as usize] = bottom;
        }
    }
}
// Utility function to turn an array into a bit shape
// Used by PieceInfo::new()
fn calculate_bit_shape(
    arr: &[[bool; PIECE_MAX_SIZE as usize]; PIECE_MAX_SIZE as usize],
) -> [u16; PIECE_MAX_SIZE as usize] {
    let mut res = [0u16; PIECE_MAX_SIZE as usize];
    for i in 0..PIECE_MAX_SIZE {
        for j in 0..PIECE_MAX_SIZE {
            if arr[i as usize][j as usize] {
                let bit = 1 << (BOARD_WIDTH - 1 - PIECE_COLUMN - i);
                res[j as usize] |= bit;
            }
        }
    }
    res
}

lazy_static! {
    static ref PIECE_INFO: PieceInfo = PieceInfo::new();
}

#[derive(Debug)]
struct PieceInfo {
    piece_shapes: [[[[bool; PIECE_MAX_SIZE as usize]; PIECE_MAX_SIZE as usize];
        PIECE_MAX_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    piece_bit_shapes:
        [[[u16; PIECE_MAX_SIZE as usize]; PIECE_MAX_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    piece_sizes: [i32; PIECE_NUM_TYPES as usize],
    piece_rotation_bounds: [i32; PIECE_NUM_TYPES as usize],
    piece_shift_bounds: [[(i32, i32); PIECE_MAX_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    piece_height_maps: [[(
        [i32; PIECE_MAX_SIZE as usize],
        [i32; PIECE_MAX_SIZE as usize],
    ); PIECE_MAX_ROTATION as usize]; PIECE_NUM_TYPES as usize],
}
impl PieceInfo {
    fn new() -> Self {
        // Constants
        let piece_sizes: [i32; 7] = [2, 4, 3, 3, 3, 3, 3];
        let piece_rotation_bounds: [i32; 7] = [1i32, 2, 4, 4, 4, 2, 2];
        let piece_base_shapes = [
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

        // Calculate shapes and bit shapes
        let mut piece_shapes = [[[[false; 4]; 4]; 4]; 7];
        let mut piece_bit_shapes = [[[0u16; 4]; 4]; 7];
        for piece in 0..PIECE_NUM_TYPES {
            let size = piece_sizes[piece as usize];
            let offset = match size {
                2 => 1,
                3 => 0,
                4 => 0,
                _ => panic!("Invalid size {}", size),
            };
            let mut shape = piece_base_shapes[piece as usize];
            for rotation in 0..PIECE_MAX_ROTATION {
                piece_shapes[piece as usize][rotation as usize] = shape;
                piece_bit_shapes[piece as usize][rotation as usize] = calculate_bit_shape(&shape);
                rotate_array(&mut shape, size, offset);
            }
        }
        // Calculate height maps
        let mut piece_height_maps = [[([0; PIECE_MAX_SIZE as usize], [0; PIECE_MAX_SIZE as usize]);
            PIECE_MAX_ROTATION as usize];
            PIECE_NUM_TYPES as usize];
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_MAX_ROTATION {
                let shape = piece_shapes[piece as usize][rotation as usize];
                let mut low = [-1; PIECE_MAX_SIZE as usize];
                let mut height = [-1; PIECE_MAX_SIZE as usize];
                // Get lows and heights
                for x in 0..PIECE_MAX_SIZE {
                    let mut found = false;
                    for y in 0..PIECE_MAX_SIZE {
                        if shape[x as usize][y as usize] {
                            if !found {
                                low[x as usize] = y;
                            }
                            found = true;
                            if height[x as usize] == -1 {
                                height[x as usize] += 1;
                            }
                            height[x as usize] += 1;
                        }
                    }
                }
                // Norm lows
                // let mut min = PIECE_MAX_SIZE;
                // for x in 0..PIECE_MAX_SIZE {
                //     if low[x as usize] < min && low[x as usize] != -1 {
                //         min = low[x as usize];
                //     }
                // }
                // for x in 0..PIECE_MAX_SIZE {
                //     if low[x as usize] != -1 {
                //         low[x as usize] -= min;
                //     }
                // }
                piece_height_maps[piece as usize][rotation as usize] = (low, height);
            }
        }

        // Calculate shift bounds
        let mut piece_shift_bounds =
            [[(0i32, 0i32); PIECE_MAX_ROTATION as usize]; PIECE_NUM_TYPES as usize];
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_MAX_ROTATION {
                let shape = piece_shapes[piece as usize][rotation as usize];
                let mut left = PIECE_COLUMN;
                'l_outer: for x in 0..PIECE_MAX_SIZE {
                    for y in 0..PIECE_MAX_SIZE {
                        if shape[x as usize][y as usize] {
                            break 'l_outer;
                        }
                    }
                    left += 1;
                }
                let mut right = BOARD_WIDTH - PIECE_MAX_SIZE - PIECE_COLUMN;
                'r_outer: for x in (0..PIECE_MAX_SIZE).rev() {
                    for y in 0..PIECE_MAX_SIZE {
                        if shape[x as usize][y as usize] {
                            break 'r_outer;
                        }
                    }
                    right += 1;
                }
                piece_shift_bounds[piece as usize][rotation as usize] = (left, right);
            }
        }

        // Generate shape offsets
        PieceInfo {
            piece_shapes,
            piece_bit_shapes,
            piece_height_maps,
            piece_shift_bounds,
            piece_rotation_bounds,
            piece_sizes,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Piece {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}
impl Piece {
    pub fn get_all_pieces() -> Vec<Piece> {
        let mut ret = Vec::new();
        for piece in 0..PIECE_NUM_TYPES {
            ret.push(Piece::from_i32(piece).unwrap())
        }
        ret
    }
    pub fn info_shape(
        piece: &Self,
        rotation: i32,
    ) -> &'static [[bool; PIECE_MAX_SIZE as usize]; PIECE_MAX_SIZE as usize] {
        return &PIECE_INFO.piece_shapes[piece.to_i32() as usize][rotation as usize];
    }
    #[allow(dead_code)]
    pub fn info_bit_shape(piece: &Self, rotation: i32) -> &'static [u16; PIECE_MAX_SIZE as usize] {
        return &PIECE_INFO.piece_bit_shapes[piece.to_i32() as usize][rotation as usize];
    }
    #[allow(dead_code)]
    pub fn info_size(piece: &Self) -> &'static i32 {
        &PIECE_INFO.piece_sizes[piece.to_i32() as usize]
    }
    pub fn info_rotation_bounds(piece: &Self) -> &'static i32 {
        &PIECE_INFO.piece_rotation_bounds[piece.to_i32() as usize]
    }
    pub fn info_shift_bounds(piece: &Self, rotation: i32) -> &'static (i32, i32) {
        &PIECE_INFO.piece_shift_bounds[piece.to_i32() as usize][rotation as usize]
    }
    pub fn info_height_map(
        piece: &Self,
        rotation: i32,
    ) -> &'static (
        [i32; PIECE_MAX_SIZE as usize],
        [i32; PIECE_MAX_SIZE as usize],
    ) {
        &PIECE_INFO.piece_height_maps[piece.to_i32() as usize][rotation as usize]
    }

    pub fn from_i32(val: i32) -> Result<Self, ()> {
        match val {
            0 => Ok(Piece::O),
            1 => Ok(Piece::I),
            2 => Ok(Piece::T),
            3 => Ok(Piece::L),
            4 => Ok(Piece::J),
            5 => Ok(Piece::S),
            6 => Ok(Piece::Z),
            _ => Err(()),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match self {
            Piece::O => 0,
            Piece::I => 1,
            Piece::T => 2,
            Piece::L => 3,
            Piece::J => 4,
            Piece::S => 5,
            Piece::Z => 6,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Piece::O => 'O',
            Piece::I => 'I',
            Piece::T => 'T',
            Piece::L => 'L',
            Piece::J => 'J',
            Piece::S => 'S',
            Piece::Z => 'Z',
        }
    }
}

#[test]
fn test_piece_info_is_correct() {
    for i in 0..PIECE_NUM_TYPES {
        let piece = Piece::from_i32(i).unwrap();
        for rotation in 0..PIECE_MAX_ROTATION {
            let size = Piece::info_size(&piece);
            let shape = Piece::info_shape(&piece, rotation);
            let bit_shape = Piece::info_bit_shape(&piece, rotation);
            let height_map = Piece::info_height_map(&piece, rotation);
            let shift_bounds = Piece::info_shift_bounds(&piece, rotation);
            let rotation_bounds = Piece::info_rotation_bounds(&piece);
            for y in (0..4).rev() {
                for x in 0..4 {
                    print!("{}", if shape[x][y] { "██" } else { "░░" });
                }
                print!("{:0>10b}\n", bit_shape[y as usize]);
            }
            for x in 0..4 {
                print!("{: >2}", height_map.0[x])
            }
            println!();
            for x in 0..4 {
                print!("{: >2}", height_map.1[x])
            }
            println!();
            println!(
                "Piece {} Rotation {}\nSize {} Bounds: Shift {:?} Rotation {}",
                piece.to_char(),
                rotation,
                size,
                shift_bounds,
                rotation_bounds
            );
            println!();
        }
        println!();
    }
}
