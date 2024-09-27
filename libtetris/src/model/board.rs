use super::piece::Piece;
use crate::PieceInfo;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter, Write},
    hash::Hash,
};

/// Width of the board
pub const BOARD_WIDTH: usize = 10;

/// Height of the board
pub const BOARD_HEIGHT: usize = 24;

/// Visible height of the board
/// Any pieces placed above this is considered a top-out
pub const BOARD_VISIBLE_HEIGHT: usize = 20;

/// Information about the board after locking a piece
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockInfo {
    pub top_out: bool,
    pub lines_cleared: i8,
}

/// Represents a rectangular grid of tiles using a bitboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "crate::serde::SerializedBoard")]
#[serde(into = "crate::serde::SerializedBoard")]
pub struct Board {
    /// A bitboard representation of the board, the first 10 bits of each u16
    /// row represents a tile.
    pub matrix: [u16; BOARD_HEIGHT],
}

impl Board {
    /// Create a new board
    pub fn new() -> Self {
        Board {
            matrix: [0; BOARD_HEIGHT],
        }
    }

    /// Get the tile at position (x, y)
    pub fn get(&self, x: usize, y: usize) -> bool {
        (self.matrix[y] >> x) & 1 != 0
    }

    /// Set the tile value at position (x, y)
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

    /// Set an entire bitboard row
    pub fn set_row(&mut self, y: usize, row: u16) {
        assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        self.matrix[y] = row;
    }

    /// Set the entire board from a given bitboard array
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT]) {
        for row in matrix {
            assert_eq!(row & !((1 << BOARD_WIDTH) - 1), 0);
        }
        self.matrix = matrix;
    }

    /// Add a number of garbage rows with a hole in the specified column
    pub fn add_garbage(&mut self, col: usize, height: u32) {
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

    /// Check whether a piece intersects with the board
    pub fn intersects_with(&self, piece: &Piece) -> bool {
        let p_y = piece.position_y as i32;
        let shape = PieceInfo::bit_shape(piece.piece_type, piece.rotation, piece.position_x);
        for j in 0..4 {
            let y = p_y + j as i32;
            if y < 0 || y >= BOARD_HEIGHT as i32 {
                continue;
            }
            let row = self.matrix[y as usize];
            if shape[j] & row != 0 {
                return true;
            }
        }
        false
    }

    /// Lock a piece onto the board
    pub fn lock(&mut self, piece: &Piece) -> LockInfo {
        let p_y = piece.position_y;
        let shape = PieceInfo::bit_shape(piece.piece_type, piece.rotation, piece.position_x);
        for j in 0..4 {
            let y = p_y + j;
            if !(0..BOARD_HEIGHT as i8).contains(&y) {
                continue;
            }
            self.matrix[y as usize] |= shape[j as usize];
        }

        let mut lines_cleared = 0;
        for j in 0..BOARD_HEIGHT {
            if self.matrix[j] == (1 << BOARD_WIDTH) - 1 {
                lines_cleared += 1;
            } else {
                self.matrix[j - lines_cleared as usize] = self.matrix[j];
            }
        }
        for j in 0..lines_cleared {
            self.matrix[(BOARD_HEIGHT as i8 - lines_cleared + j) as usize] = 0;
        }

        // Check for top-out
        let top_out = self.topped_out();

        LockInfo {
            lines_cleared,
            top_out,
        }
    }

    /// Check whether the board is currently topped out
    pub fn topped_out(&self) -> bool {
        self.matrix[BOARD_VISIBLE_HEIGHT] != 0
    }

    /// Returns the maximum height of a column of the board
    pub fn max_height(&self) -> i8 {
        for i in 0..BOARD_HEIGHT {
            if self.matrix[i] == 0 {
                return i as i8;
            }
        }
        BOARD_HEIGHT as i8
    }

    /// Gives a count of the number of holes in each column. A hole is defined
    /// as an empty tile that has a full tile somewhere above it.
    pub fn holes(&self) -> [i8; BOARD_WIDTH] {
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

    /// Gives the row of the highest tile in each column.
    pub fn height_map(&self) -> [i8; BOARD_WIDTH] {
        let mut height_map = [0; BOARD_WIDTH];
        let max_height = self.max_height();
        for i in 0..BOARD_WIDTH {
            for j in (0..max_height).rev() {
                if self.get(i, j as usize) {
                    height_map[i] = j + 1;
                    break;
                }
            }
        }
        height_map
    }

    /// Return a string representation of the board
    pub fn to_string(&self, piece: Option<&Piece>) -> String {
        let mut text = String::new();
        for j in (0..BOARD_HEIGHT).rev() {
            write!(text, "{j:>2}").unwrap();
            for i in 0..BOARD_WIDTH {
                let (in_piece_bounds, in_piece) = match piece {
                    Some(piece) => {
                        let piece_shape = PieceInfo::shape(piece.piece_type, piece.rotation);
                        let p_x = piece.position_x as usize;
                        let p_y = piece.position_y as usize;
                        let x = i as i8 - p_x as i8;
                        let y = j as i8 - p_y as i8;
                        let in_piece_bounds = (0..4).contains(&x) && (0..4).contains(&y);
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
                    write!(text, "--").unwrap();
                } else {
                    write!(text, "..").unwrap();
                }
            }
            writeln!(text).unwrap();
        }
        write!(text, "  ").unwrap();
        for i in 0..BOARD_WIDTH {
            write!(text, "{i:>2}").unwrap();
        }
        writeln!(text).unwrap();
        text
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(None))
    }
}
