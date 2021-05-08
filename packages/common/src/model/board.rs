use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::piece::Piece;

#[derive(Debug)]
pub struct BoardLockResult {
    pub lines_cleared: i32,
    pub block_out: bool,
}

#[derive(Debug)]
pub struct BoardUndoInfo {
    pub matrix: [u16; PIECE_SHAPE_SIZE as usize],
    pub shape_y: i32,
    pub lines_cleared: Vec<i32>,
    pub height_map: [i32; BOARD_WIDTH as usize],
    pub holes: [i32; BOARD_WIDTH as usize],
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    pub matrix: [u16; BOARD_HEIGHT as usize],
    pub height_map: [i32; BOARD_WIDTH as usize],
    pub holes: [i32; BOARD_WIDTH as usize],
}
impl Board {
    pub fn new() -> Self {
        Board {
            matrix: [0; BOARD_HEIGHT as usize],
            height_map: [0; BOARD_WIDTH as usize],
            holes: [0; BOARD_WIDTH as usize],
        }
    }
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.matrix[y as usize] & (1 << x) != 0
    }
    pub fn set(&mut self, x: i32, y: i32, state: bool) {
        if self.get(x, y) == state {
            return;
        }
        if state {
            // Turn on bit
            self.matrix[y as usize] |= 1 << x;
            if y >= self.height_map[x as usize] {
                // Above height - add holes if necessary
                self.holes[x as usize] += y - self.height_map[x as usize];
                self.height_map[x as usize] = y + 1;
            } else {
                // Below height (must be a hole that was removed)
                self.holes[x as usize] -= 1;
            }
        } else {
            // Turn off bit
            self.matrix[y as usize] &= !(1 << x);
            if self.height_map[x as usize] == y + 1 {
                // Top hole was turned off
                // Keep looping down until hole is found
                loop {
                    self.height_map[x as usize] -= 1;
                    if self.height_map[x as usize] == 0
                        || self.get(x, self.height_map[x as usize] - 1)
                    {
                        break;
                    }
                    self.holes[x as usize] -= 1;
                }
            } else {
                // Bit must be a hole
                self.holes[x as usize] += 1
            }
        }
    }
    pub fn set_col(&mut self, col: i32, height: i32) {
        for i in 0..BOARD_HEIGHT {
            if i < height {
                self.matrix[i as usize] |= 1 << col;
            } else {
                self.matrix[i as usize] &= !(1 << col);
            }
        }
        self.height_map[col as usize] = height;
        self.holes[col as usize] = 0;
    }
    pub fn set_cols(&mut self, heights: [i32; BOARD_WIDTH as usize]) {
        for i in 0..BOARD_WIDTH {
            self.set_col(i, heights[i as usize]);
        }
    }
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT as usize]) {
        self.matrix = matrix;
        for i in 0..BOARD_WIDTH {
            self.recalculate_metadata(i);
        }
    }
    pub fn intersects_with(&self, piece: &Piece) -> bool {
        let p_y = piece.location.1;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            if p_y + j < 0 || p_y + j >= BOARD_HEIGHT {
                continue;
            }
            let row = self.matrix[(p_y + j) as usize];
            if shape[j as usize] & row != 0 {
                return true;
            }
        }
        false
    }
    pub fn lock(&mut self, piece: &Piece) -> (BoardLockResult, BoardUndoInfo) {
        let height_map_backup = self.height_map.clone();
        let holes_backup = self.holes.clone();
        let mut matrix_backup = [0; PIECE_SHAPE_SIZE as usize];
        let mut block_out = false;

        let (p_x, p_y) = piece.location;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            let y = p_y + j;
            if y < 0 || y >= BOARD_HEIGHT {
                continue;
            }
            let matrix_row = self.matrix[y as usize];
            let piece_row = shape[j as usize];
            if matrix_row & piece_row != 0 {
                block_out = true;
            }
            matrix_backup[j as usize] = matrix_row;
            self.matrix[y as usize] |= piece_row;
        }

        let mut lines_cleared = Vec::new();
        for j in (0..BOARD_HEIGHT).rev() {
            if self.matrix[j as usize] == (1 << BOARD_WIDTH) - 1 {
                lines_cleared.push(j);
                for y in j..BOARD_HEIGHT {
                    if y == BOARD_HEIGHT - 1 {
                        self.matrix[y as usize] = 0;
                    } else {
                        self.matrix[y as usize] = self.matrix[(y + 1) as usize];
                    }
                }
            }
        }

        // TODO: maybe find a faster algorithm for recalculating metadata?
        // Although this is probably fast enough anyways
        if lines_cleared.len() == 0 {
            for i in 0..PIECE_SHAPE_SIZE {
                let x = i + p_x;
                if x >= 0 && x < BOARD_WIDTH {
                    self.recalculate_metadata(x);
                }
            }
        } else {
            for i in 0..BOARD_WIDTH {
                self.recalculate_metadata(i);
            }
        }

        (
            BoardLockResult {
                lines_cleared: lines_cleared.len() as i32,
                block_out,
            },
            BoardUndoInfo {
                matrix: matrix_backup,
                shape_y: p_y,
                lines_cleared,
                height_map: height_map_backup,
                holes: holes_backup,
            },
        )
    }
    pub fn undo_lock(&mut self, undo: BoardUndoInfo) {
        self.height_map = undo.height_map;
        self.holes = undo.holes;

        // Undo lines cleared
        for j in undo.lines_cleared.iter().rev() {
            for y in ((*j + 1)..BOARD_HEIGHT).rev() {
                self.matrix[y as usize] = self.matrix[(y - 1) as usize];
            }
            self.matrix[*j as usize] = (1 << BOARD_WIDTH) - 1;
        }

        // Undo shape
        for j in 0..PIECE_SHAPE_SIZE {
            let y = j + undo.shape_y;
            if y < 0 || y >= BOARD_HEIGHT {
                continue;
            }
            self.matrix[y as usize] = undo.matrix[j as usize];
        }
    }

    fn recalculate_metadata(&mut self, col: i32) {
        let mut encountered = false;
        let mut height = 0;
        let mut holes = 0;
        for j in (0..BOARD_HEIGHT).rev() {
            if self.get(col, j) {
                if !encountered {
                    encountered = true;
                    height = j + 1;
                }
            } else {
                if encountered {
                    holes += 1
                }
            }
        }
        self.height_map[col as usize] = height;
        self.holes[col as usize] = holes;
    }
}
