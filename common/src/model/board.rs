use super::piece::Piece;
use crate::model::consts::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardLockRes {
    pub top_out: bool,
    pub lines_cleared: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "BoardSer")]
#[serde(into = "BoardSer")]
pub struct Board {
    pub matrix: [u16; BOARD_HEIGHT as usize],
    pub height_map: [i8; BOARD_WIDTH as usize],
}
impl Board {
    pub fn new() -> Self {
        Board {
            matrix: [0; BOARD_HEIGHT as usize],
            height_map: [0; BOARD_WIDTH as usize],
        }
    }
    pub fn from_matrix(matrix: [u16; BOARD_HEIGHT as usize]) -> Self {
        let mut board = Board::new();
        board.set_matrix(matrix);
        board
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
        let max_height = std::cmp::max(y + 1, self.height_map[x as usize] as i32);
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
    pub fn add_garbage(&mut self, col: i32, height: i32) {
        assert!(col >= 0 && col < BOARD_WIDTH);
        assert!(height >= 0 && height < BOARD_HEIGHT);
        // Copy rows up
        for j in (height..BOARD_HEIGHT).rev() {
            self.matrix[j as usize] = self.matrix[(j - height) as usize];
        }
        // Set garbage rows
        let garbage_row: u16 = ((1 << BOARD_WIDTH) - 1) & !(1 << col);
        for j in 0..height {
            self.matrix[j as usize] = garbage_row;
        }
        // Update Metadata
        for i in 0..BOARD_WIDTH {
            let max_height =
                std::cmp::min(self.height_map[i as usize] as i32 + height, BOARD_HEIGHT);
            self.recalculate_metadata(i, max_height);
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

        // Recalculate height map
        let mut height_map_max = i8::MIN;
        for height in self.height_map {
            if height > height_map_max {
                height_map_max = height;
            }
        }
        // Current maximum possible height of the board
        let max_height = std::cmp::min(
            height_map_max as i32 + PIECE_SHAPE_SIZE - lines_cleared,
            BOARD_HEIGHT,
        );
        if lines_cleared == 0 {
            // Only check metadata for the columns that the piece dropped in
            for i in 0..PIECE_SHAPE_SIZE {
                let x = i + (p_x as i32);
                if x < 0 || x >= BOARD_WIDTH {
                    continue;
                }
                self.recalculate_metadata(x, max_height);
            }
        } else {
            // Check all columns
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
    pub fn calculate_holes(&self) -> [i32; BOARD_WIDTH as usize] {
        let mut holes = [0; BOARD_WIDTH as usize];
        for j in 0..BOARD_WIDTH as usize {
            for i in 0..self.height_map[j] as usize {
                if self.get(i as i32, j as i32) {
                    holes[j] += 1;
                }
            }
        }
        holes
    }

    fn recalculate_metadata(&mut self, x: i32, max_height: i32) {
        // max_height - assert that the cell (x, max_height)
        // and all the cells above it are empty
        for j in (0..max_height).rev() {
            if self.get(x, j) {
                self.height_map[x as usize] = (j + 1) as i8;
                return;
            }
        }
        self.height_map[x as usize] = 0;
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

// Small type used for ser/de
type BoardSer = [[u8; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];
impl From<BoardSer> for Board {
    fn from(ser: BoardSer) -> Self {
        let mut board = Board::new();
        for (i, col) in ser.iter().enumerate() {
            for (j, cell) in col.iter().enumerate() {
                let val = match cell {
                    0 => false,
                    _ => true,
                };
                board.set(i as i32, j as i32, val);
            }
        }
        board
    }
}
impl From<Board> for BoardSer {
    fn from(board: Board) -> Self {
        let mut ser: BoardSer = Default::default();
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let val = match board.get(i, j) {
                    false => 0,
                    true => 1,
                };
                ser[i as usize][j as usize] = val;
            }
        }
        ser
    }
}
