use super::piece::Piece;
use crate::model::consts::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardLockRes {
    pub top_out: bool,
    pub lines_cleared: i32,
}

#[derive(Debug, Clone, Copy)]
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
        if state {
            self.matrix[y as usize] |= 1 << x;
        } else {
            self.matrix[y as usize] &= !(1 << x);
        }
        let max_height = std::cmp::max(y, self.height_map[x as usize] as i32);
        self.recalculate_metadata(x, max_height);
    }
    pub fn set_col(&mut self, x: i32, height: i32) {
        for i in 0..BOARD_HEIGHT {
            if i < height {
                self.matrix[i as usize] |= 1 << x;
            } else {
                self.matrix[i as usize] &= !(1 << x);
            }
        }
        self.height_map[x as usize] = height as i8;
        self.holes[x as usize] = 0;
    }
    pub fn set_cols(&mut self, heights: [i32; BOARD_WIDTH as usize]) {
        for i in 0..BOARD_WIDTH {
            self.set_col(i, heights[i as usize]);
        }
    }
    pub fn set_row(&mut self, y: i32, row: u16) {
        assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        self.matrix[y as usize] = row;
        for i in 0..BOARD_WIDTH {
            let max_height = std::cmp::max(y + 1, self.height_map[i as usize] as i32);
            self.recalculate_metadata(i, max_height)
        }
    }
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT as usize]) {
        for row in matrix {
            assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        }
        self.matrix = matrix;
        for i in 0..BOARD_WIDTH {
            self.recalculate_metadata(i, BOARD_HEIGHT);
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
    pub fn lock(&mut self, piece: &Piece) -> BoardLockRes {
        let (p_x, p_y) = piece.location;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            let y = (p_y as i32) + j;
            if y < 0 || y >= BOARD_HEIGHT {
                continue;
            }
            self.matrix[y as usize] |= shape[j as usize];
        }

        let mut lines_cleared = 0;
        for j in 0..BOARD_HEIGHT {
            if self.matrix[j as usize] == (1 << BOARD_WIDTH) - 1 {
                lines_cleared += 1;
            } else {
                self.matrix[(j - lines_cleared) as usize] = self.matrix[j as usize];
            }
        }
        for j in 0..lines_cleared {
            self.matrix[(BOARD_HEIGHT - lines_cleared + j) as usize] = 0;
        }

        // Check for top-out
        let top_out = self.matrix[BOARD_VISIBLE_HEIGHT as usize] != 0;

        // Recalcluate metadatas
        // TODO: Figure out an efficient method for this
        let mut height_map_max = i8::MIN;
        for height in self.height_map {
            if height > height_map_max {
                height_map_max = height;
            }
        }
        let max_height = std::cmp::min(
            height_map_max as i32 + PIECE_SHAPE_SIZE - lines_cleared,
            BOARD_HEIGHT,
        );
        if lines_cleared == 0 {
            for i in 0..PIECE_SHAPE_SIZE {
                let x = i + (p_x as i32);
                if x < 0 || x >= BOARD_WIDTH {
                    continue;
                }
                self.recalculate_metadata(x, max_height);
            }
        } else {
            for x in 0..BOARD_WIDTH {
                self.recalculate_metadata(x, max_height);
            }
        }

        BoardLockRes {
            lines_cleared,
            top_out,
        }
    }
    pub fn topped_out(&self) -> bool {
        self.matrix[BOARD_VISIBLE_HEIGHT as usize] != 0
    }

    fn recalculate_metadata(&mut self, x: i32, max_height: i32) {
        // max_height - assert that all cells above (x, max_height)
        // is empty
        let mut encountered = false;
        let mut height = 0;
        let mut holes = 0;
        for j in (0..max_height).rev() {
            if self.matrix[j as usize] & (1 << x) != 0 {
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
        self.height_map[x as usize] = height;
        self.holes[x as usize] = holes;
    }
}
// Only compare matrix, other fields are only metadata
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}
impl Eq for Board {}
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.matrix.hash(state);
    }
}
