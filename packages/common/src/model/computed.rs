use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::PIECE_NUM_ROTATION;
use crate::model::consts::PIECE_NUM_TYPES;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::consts::PIECE_SPAWN_COLUMN;
use crate::model::consts::{BOARD_WIDTH, PIECE_MAX_X_SHIFT};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref PIECE_INFO: PieceInfo = PieceInfo::new();
}

/// Precomputed constants for a piece
pub(crate) struct PieceInfo {
    /// The spawn location for each piece
    pub spawn_locations: [(i32, i32); PIECE_NUM_TYPES as usize],
    /// The shape of each piece, as a 2d array of bools
    pub shapes: [[[[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize];
        PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    /// u16 bit maps of each shape
    /// If shifting out of bounds, the shape will be cut-off
    pub bit_shapes: [[[[u16; PIECE_SHAPE_SIZE as usize]; (PIECE_MAX_X_SHIFT as usize * 2) + 1];
        PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    /// Lows and Heights (Height from bottom to first block, then height of blocks)
    /// Both fields are -1 if that column is empty
    pub height_maps: [[[(i32, i32); PIECE_SHAPE_SIZE as usize]; PIECE_NUM_ROTATION as usize];
        PIECE_NUM_TYPES as usize],
    /// Min/Max x/y positions for a piece (min x, max x, min y, max y)
    pub location_bounds:
        [[(i32, i32, i32, i32); PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    /// How much a piece can shift from its spawn position
    pub shift_bounds: [[(i32, i32); PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize],
    pub kick_table: [[[Vec<(i32, i32)>; PIECE_NUM_ROTATION as usize]; PIECE_NUM_ROTATION as usize];
        PIECE_NUM_TYPES as usize],
}

impl PieceInfo {
    fn new() -> Self {
        fn rotate_shape(
            arr: [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize],
            size: i32,
        ) -> [[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize] {
            let mut new_arr = [[false; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize];
            for i in 0..size {
                for j in 0..size {
                    new_arr[j as usize][(size - 1 - i) as usize] = arr[i as usize][j as usize];
                }
            }
            new_arr
        }

        let sizes = [2, 4, 3, 3, 3, 3, 3];
        let spawn_locations = [
            (PIECE_SPAWN_COLUMN, 19),
            (PIECE_SPAWN_COLUMN, 18),
            (PIECE_SPAWN_COLUMN, 19),
            (PIECE_SPAWN_COLUMN, 19),
            (PIECE_SPAWN_COLUMN, 19),
            (PIECE_SPAWN_COLUMN, 19),
            (PIECE_SPAWN_COLUMN, 19),
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
        let mut shapes: [[[[bool; PIECE_SHAPE_SIZE as usize]; PIECE_SHAPE_SIZE as usize];
            PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize] = Default::default();
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
        let mut bit_shapes: [[[[u16; PIECE_SHAPE_SIZE as usize];
            (PIECE_MAX_X_SHIFT as usize * 2) + 1];
            PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize] = Default::default();
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = shapes[piece as usize][rotation as usize];
                for s in 0..(PIECE_MAX_X_SHIFT * 2) + 1 {
                    let mut bit_shape = [0u16; PIECE_SHAPE_SIZE as usize];
                    for i in 0..PIECE_SHAPE_SIZE {
                        for j in 0..PIECE_SHAPE_SIZE {
                            if !shape[i as usize][j as usize] {
                                continue;
                            }
                            let x = PIECE_SPAWN_COLUMN - PIECE_MAX_X_SHIFT + s + i;
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
        let mut height_maps: [[[(i32, i32); PIECE_SHAPE_SIZE as usize];
            PIECE_NUM_ROTATION as usize]; PIECE_NUM_TYPES as usize] = Default::default();
        let mut location_bounds: [[(i32, i32, i32, i32); PIECE_NUM_ROTATION as usize];
            PIECE_NUM_TYPES as usize] = Default::default();
        let mut shift_bounds: [[(i32, i32); PIECE_NUM_ROTATION as usize];
            PIECE_NUM_TYPES as usize] = Default::default();
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = shapes[piece as usize][rotation as usize];
                let bit_shape =
                    bit_shapes[piece as usize][rotation as usize][PIECE_MAX_X_SHIFT as usize];
                let mut height_map = [(-1, -1); PIECE_SHAPE_SIZE as usize];
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
                location_bounds[piece as usize][rotation as usize] = (left, right, bottom, top);
                shift_bounds[piece as usize][rotation as usize] =
                    (PIECE_SPAWN_COLUMN - left, right - PIECE_SPAWN_COLUMN);
            }
        }
        // Pain
        let o_kick_table: [[Vec<(i32, i32)>; PIECE_NUM_ROTATION as usize];
            PIECE_NUM_ROTATION as usize] = [
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
            spawn_locations,
            shapes,
            bit_shapes,
            height_maps,
            location_bounds,
            shift_bounds,
            kick_table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PIECE_INFO;
    use crate::model::consts::BOARD_WIDTH;
    use crate::model::consts::PIECE_MAX_X_SHIFT;
    use crate::model::consts::PIECE_NUM_ROTATION;
    use crate::model::consts::PIECE_NUM_TYPES;
    use crate::model::consts::PIECE_SHAPE_SIZE;
    use crate::model::consts::PIECE_SPAWN_COLUMN;

    #[test]
    fn bit_shapes_match_shapes() {
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = PIECE_INFO.shapes[piece as usize][rotation as usize];
                // Get the center shape
                let bit_shape = PIECE_INFO.bit_shapes[piece as usize][rotation as usize]
                    [PIECE_MAX_X_SHIFT as usize];
                // Check all 16 bits
                for j in 0..PIECE_SHAPE_SIZE {
                    for i in 0..16 {
                        let bit = (bit_shape[j as usize] >> i) & 1;
                        let x = i - PIECE_SPAWN_COLUMN;
                        if x < 0 || x >= PIECE_SHAPE_SIZE {
                            assert_eq!(bit, 0);
                        } else {
                            if shape[x as usize][j as usize] {
                                assert_eq!(bit, 1);
                            } else {
                                assert_eq!(bit, 0);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bit_shapes_match() {
        let bit_mask = (1 << BOARD_WIDTH) - 1;
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let bit_shape_arr = PIECE_INFO.bit_shapes[piece as usize][rotation as usize];
                let center_shape = bit_shape_arr[PIECE_MAX_X_SHIFT as usize];
                for shift in 1..PIECE_MAX_X_SHIFT {
                    let left_shape = bit_shape_arr[(PIECE_MAX_X_SHIFT - shift) as usize];
                    let right_shape = bit_shape_arr[(PIECE_MAX_X_SHIFT + shift) as usize];
                    for j in 0..PIECE_SHAPE_SIZE {
                        let center = center_shape[j as usize];
                        let left = left_shape[j as usize];
                        let right = right_shape[j as usize];
                        assert_eq!(left, center >> shift);
                        assert_eq!(right, (center << shift) & bit_mask);
                    }
                }
            }
        }
    }

    // TODO: Maybe add other tests
}
