use super::piece::Piece;
use crate::model::consts::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter, Write},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardLockRes {
    pub top_out: bool,
    pub lines_cleared: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "BoardSer")]
#[serde(into = "BoardSer")]
pub struct Board {
    pub matrix: [u16; BOARD_HEIGHT],
}
impl Board {
    pub fn new() -> Self {
        Board {
            matrix: [0; BOARD_HEIGHT],
        }
    }
    pub fn from_matrix(matrix: [u16; BOARD_HEIGHT]) -> Self {
        let mut board = Board::new();
        board.set_matrix(matrix);
        board
    }
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.matrix[y] & (1 << x) != 0
    }
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        if self.get(x, y) == state {
            return;
        }
        if state {
            self.matrix[y] |= 1 << x;
        } else {
            self.matrix[y] &= !(1 << x);
        }
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
    }
    pub fn set_cols(&mut self, heights: [i8; BOARD_WIDTH]) {
        for i in 0..BOARD_WIDTH {
            self.set_col(i, heights[i]);
        }
    }
    pub fn set_row(&mut self, y: usize, row: u16) {
        assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        self.matrix[y] = row;
    }
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT]) {
        for row in matrix {
            assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        }
        self.matrix = matrix;
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
        let top_out = self.topped_out();

        BoardLockRes {
            lines_cleared,
            top_out,
        }
    }
    #[inline]
    pub fn topped_out(&self) -> bool {
        self.matrix[BOARD_VISIBLE_HEIGHT] != 0
    }
    #[inline]
    pub fn max_height(&self) -> i8 {
        for i in 0..BOARD_HEIGHT {
            if self.matrix[i] == 0 {
                return i as i8;
            }
        }
        BOARD_HEIGHT as i8
    }
    pub fn holes(&self) -> [i32; BOARD_WIDTH] {
        let mut holes = [0; BOARD_WIDTH];
        let max_height = self.max_height() as usize;
        for i in 0..BOARD_WIDTH {
            for j in 0..max_height {
                if self.get(i, j) {
                    holes[i] += 1;
                }
            }
        }
        holes
    }
    pub fn height_map(&self) -> [i8; BOARD_WIDTH] {
        let mut height_map = [0; BOARD_WIDTH];
        let max_height = self.max_height() as usize;
        for i in 0..BOARD_WIDTH {
            for j in (0..max_height).rev() {
                if self.get(i, j) {
                    height_map[i] = j as i8;
                    break;
                }
            }
        }
        height_map
    }

    pub fn to_string(&self, piece: Option<&Piece>) -> String {
        let mut text = String::new();
        for j in (0..BOARD_HEIGHT).rev() {
            for i in 0..BOARD_WIDTH {
                let (in_piece_bounds, in_piece) = match piece {
                    Some(piece) => {
                        let piece_shape = piece.get_shape(None);
                        let p_x = piece.location.0 as usize;
                        let p_y = piece.location.1 as usize;
                        let x = i as i8 - p_x as i8;
                        let y = j as i8 - p_y as i8;
                        let in_piece_bounds = x >= 0
                            && x < PIECE_SHAPE_SIZE as i8
                            && y >= 0
                            && y < PIECE_SHAPE_SIZE as i8;
                        let in_piece = in_piece_bounds && piece_shape[x as usize][y as usize];
                        (in_piece_bounds, in_piece)
                    }
                    None => (false, false),
                };

                if in_piece {
                    write!(text, "{{}}").unwrap();
                } else if self.get(i, j) {
                    write!(text, "[]").unwrap();
                } else if in_piece_bounds {
                    if j >= BOARD_VISIBLE_HEIGHT {
                        write!(text, "▒▒").unwrap();
                    } else {
                        write!(text, "▓▓").unwrap();
                    }
                } else {
                    if j >= BOARD_VISIBLE_HEIGHT {
                        write!(text, "▓▓").unwrap();
                    } else {
                        write!(text, "██").unwrap();
                    }
                }
            }
            writeln!(text).unwrap();
        }
        text
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(None))
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
