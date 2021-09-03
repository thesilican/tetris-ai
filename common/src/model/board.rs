use super::piece::Piece;
use crate::model::consts::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardLockRes {
    pub top_out: bool,
    pub lines_cleared: i8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "BoardSer")]
#[serde(into = "BoardSer")]
pub struct Board {
    pub matrix: [u16; BOARD_HEIGHT],
    pub height_map: [i8; BOARD_WIDTH],
}
impl Board {
    pub fn new() -> Self {
        Board {
            matrix: [0; BOARD_HEIGHT],
            height_map: [0; BOARD_WIDTH],
        }
    }
    pub fn from_matrix(matrix: [u16; BOARD_HEIGHT]) -> Self {
        let mut board = Board::new();
        board.set_matrix(matrix);
        board
    }
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.matrix[y] & (1 << x) != 0
    }
    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        if self.get(x, y) == state {
            return;
        }
        if state {
            self.matrix[y] |= 1 << x;
        } else {
            self.matrix[y] &= !(1 << x);
        }
        let max_height = std::cmp::max((y + 1) as i8, self.height_map[x]);
        self.recalculate_metadata(x, max_height);
    }
    pub fn set_col(&mut self, x: usize, height: i8) {
        assert!(height >= 0);
        for i in 0..BOARD_HEIGHT {
            if i < height as usize {
                self.matrix[i] |= 1 << x;
            } else {
                self.matrix[i] &= !(1 << x);
            }
        }
        self.height_map[x] = height;
    }
    pub fn set_cols(&mut self, heights: [i8; BOARD_WIDTH]) {
        for i in 0..BOARD_WIDTH {
            self.set_col(i, heights[i]);
        }
    }
    pub fn set_row(&mut self, y: usize, row: u16) {
        assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        self.matrix[y] = row;
        for i in 0..BOARD_WIDTH {
            let max_height = std::cmp::max((y + 1) as i8, self.height_map[i]);
            self.recalculate_metadata(i, max_height)
        }
    }
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT]) {
        for row in matrix {
            assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        }
        self.matrix = matrix;
        for i in 0..BOARD_WIDTH {
            self.recalculate_metadata(i, BOARD_HEIGHT as i8);
        }
    }
    pub fn add_garbage(&mut self, col: usize, height: i8) {
        let height = height as usize;
        assert!(col < BOARD_WIDTH);
        assert!(height < BOARD_HEIGHT);
        // Copy rows up
        for j in (height..BOARD_HEIGHT).rev() {
            self.matrix[j] = self.matrix[j - height];
        }
        // Set garbage rows
        let garbage_row: u16 = ((1 << BOARD_WIDTH) - 1) & !(1 << col);
        for j in 0..height {
            self.matrix[j] = garbage_row;
        }
        // Update Metadata
        for i in 0..BOARD_WIDTH {
            let max_height = std::cmp::min(self.height_map[i] + (height as i8), BOARD_HEIGHT as i8);
            self.recalculate_metadata(i, max_height);
        }
    }
    pub fn intersects_with(&self, piece: &Piece) -> bool {
        let p_y = piece.location.1;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            let y = p_y + j as i8;
            if y < 0 || y >= BOARD_HEIGHT as i8 {
                continue;
            }
            let row = self.matrix[y as usize];
            if shape[j] & row != 0 {
                return true;
            }
        }
        false
    }
    pub fn lock(&mut self, piece: &Piece) -> BoardLockRes {
        let p_x = piece.location.0;
        let p_y = piece.location.1;
        let shape = piece.get_bit_shape(None, None);
        for j in 0..PIECE_SHAPE_SIZE {
            let y = p_y + j as i8;
            if y < 0 || y >= BOARD_HEIGHT as i8 {
                continue;
            }
            self.matrix[y as usize] |= shape[j];
        }

        let mut lines_cleared = 0i8;
        for j in 0..BOARD_HEIGHT {
            if self.matrix[j] == (1 << BOARD_WIDTH) - 1 {
                lines_cleared += 1;
            } else {
                self.matrix[j - lines_cleared as usize] = self.matrix[j];
            }
        }
        for j in 0..lines_cleared {
            self.matrix[BOARD_HEIGHT - lines_cleared as usize + j as usize] = 0;
        }

        // Check for top-out
        let top_out = self.matrix[BOARD_VISIBLE_HEIGHT] != 0;

        // Recalculate height map
        let mut height_map_max = i8::MIN;
        for height in self.height_map {
            if height > height_map_max {
                height_map_max = height;
            }
        }
        // Current maximum possible height of the board
        let max_height = std::cmp::min(
            height_map_max + PIECE_SHAPE_SIZE as i8 - lines_cleared,
            BOARD_HEIGHT as i8,
        );
        if lines_cleared == 0 {
            // Only check metadata for the columns that the piece dropped in
            for i in 0..PIECE_SHAPE_SIZE {
                let x = i as i8 + p_x;
                if x < 0 || x >= BOARD_WIDTH as i8 {
                    continue;
                }
                self.recalculate_metadata(x as usize, max_height);
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
        self.matrix[BOARD_VISIBLE_HEIGHT] != 0
    }
    pub fn calculate_holes(&self) -> [i32; BOARD_WIDTH] {
        let mut holes = [0; BOARD_WIDTH];
        for j in 0..BOARD_WIDTH {
            for i in 0..self.height_map[j] as usize {
                if self.get(i, j) {
                    holes[j] += 1;
                }
            }
        }
        holes
    }

    fn recalculate_metadata(&mut self, x: usize, max_height: i8) {
        // max_height - assert that the cell (x, max_height)
        // and all the cells above it are empty
        for j in (0..max_height).rev() {
            if self.get(x, j as usize) {
                self.height_map[x] = j + 1;
                return;
            }
        }
        self.height_map[x] = 0;
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct BoardSer([[u8; BOARD_HEIGHT]; BOARD_WIDTH]);
impl From<BoardSer> for Board {
    fn from(ser: BoardSer) -> Self {
        let mut board = Board::new();
        for (i, col) in ser.0.iter().enumerate() {
            for (j, cell) in col.iter().enumerate() {
                let val = match cell {
                    0 => false,
                    _ => true,
                };
                board.set(i, j, val);
            }
        }
        board
    }
}
impl From<Board> for BoardSer {
    fn from(board: Board) -> Self {
        let mut arr = [[0u8; BOARD_HEIGHT]; BOARD_WIDTH];
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                arr[i][j] = match board.get(i, j) {
                    false => 0,
                    true => 1,
                };
            }
        }
        BoardSer(arr)
    }
}
