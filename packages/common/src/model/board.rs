use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_VISIBLE_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::piece::Piece;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct BoardLockResult {
    pub top_out: bool,
    pub lines_cleared: i32,
}

#[derive(Clone, Debug, Eq)]
pub struct Board {
    pub matrix: [u16; BOARD_HEIGHT as usize],
    pub height_map: [i8; BOARD_WIDTH as usize],
    pub holes: [i8; BOARD_WIDTH as usize],
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
        let x = x as i8;
        let y = y as i8;
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
                        || self.get(x as i32, (self.height_map[x as usize] - 1) as i32)
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
        self.height_map[col as usize] = height as i8;
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
        let p_y = piece.location.1 as i32;
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
    pub fn lock(&mut self, piece: &Piece) -> BoardLockResult {
        let (p_x, p_y) = piece.location;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            let y = (p_y as i32) + j;
            if y < 0 || y >= BOARD_HEIGHT {
                continue;
            }
            self.matrix[y as usize] |= shape[j as usize];
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

        // Check for top-out
        let mut top_out = false;
        for j in BOARD_VISIBLE_HEIGHT..BOARD_HEIGHT {
            if self.matrix[j as usize] != 0 {
                top_out = true;
                break;
            }
        }

        // Recalcluate metadatas
        if lines_cleared.len() == 0 {
            for i in 0..PIECE_SHAPE_SIZE {
                let x = i + (p_x as i32);
                if x >= 0 && x < BOARD_WIDTH {
                    self.recalculate_metadata(x);
                }
            }
        } else {
            for i in 0..BOARD_WIDTH {
                self.recalculate_metadata(i);
            }
        }

        BoardLockResult {
            lines_cleared: lines_cleared.len() as i32,
            top_out,
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
                    height = (j + 1) as i8;
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
// Only compare matrix, other fields are only metadata
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.matrix.hash(state);
    }
}
