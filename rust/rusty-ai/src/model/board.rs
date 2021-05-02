use crate::model::consts::BOARD_VISIBLE_HEIGHT;
use crate::model::consts::PIECE_COLUMN;
use crate::model::consts::PIECE_MAX_SIZE;
use crate::model::consts::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::model::piece::Piece;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Board {
    pub height_map: [i32; BOARD_WIDTH as usize],
    pub matrix: [u16; BOARD_HEIGHT as usize],
    pub holes: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            height_map: [0; BOARD_WIDTH as usize],
            matrix: [0; BOARD_HEIGHT as usize],
            holes: 0,
        }
    }
    pub fn drop(
        &mut self,
        piece: &Piece,
        drop_info: &BoardDropOptions,
    ) -> Result<(BoardDropResult, BoardUndoInfo), ()> {
        let (lows, heights) = Piece::info_height_map(piece, drop_info.rotation);
        let left = PIECE_COLUMN + drop_info.shift;

        // Find the height that the piece rests on the matrix
        let mut height = i32::MIN;
        for i in 0..PIECE_MAX_SIZE {
            if lows[i as usize] == -1 {
                continue;
            }
            height = std::cmp::max(
                self.height_map[(left + i) as usize] - lows[i as usize],
                height,
            );
        }

        // Prepare undo info
        let undo_info = BoardUndoInfo {
            height_map: self.height_map,
            matrix: self.matrix,
            holes: self.holes,
        };

        // Add blocks to matrix
        for i in 0..PIECE_MAX_SIZE {
            if heights[i as usize] == -1 {
                continue;
            }
            self.holes += height + lows[i as usize] - self.height_map[(left + i) as usize];
            self.height_map[(left + i) as usize] = height + lows[i as usize] + heights[i as usize];
            for j in 0..heights[i as usize] {
                self.matrix[(height + lows[i as usize] + j) as usize] |= 1 << (left + i);
            }
        }

        // Check for line clears
        let mut lines_cleared = 0i32;
        for j in 0..BOARD_HEIGHT {
            let row = self.matrix[(j - lines_cleared) as usize];
            if row == 0 {
                break;
            }
            if row == (1 << BOARD_WIDTH) - 1 {
                let mut i = j - lines_cleared;
                lines_cleared += 1;
                // Copy rows down
                while self.matrix[i as usize] != 0 {
                    self.matrix[i as usize] = self.matrix[(i + 1) as usize];
                    i += 1;
                }
                // Height maps
                for x in 0..BOARD_WIDTH {
                    self.height_map[x as usize] -= 1;
                    // Loop down until non-hole is found
                    for y in (0..self.height_map[x as usize]).rev() {
                        if ((self.matrix[y as usize] >> x) & 1) == 1 {
                            break;
                        }
                        self.height_map[x as usize] -= 1;
                        self.holes -= 1;
                    }
                }
            }
        }

        if self.matrix[BOARD_VISIBLE_HEIGHT as usize] != 0 {
            self.undo(&undo_info);
            return Err(());
        }
        let perfect_clear = self.matrix[0] == 0;
        Ok((
            BoardDropResult {
                lines_cleared,
                perfect_clear,
            },
            undo_info,
        ))
    }
    pub fn undo(&mut self, undo_info: &BoardUndoInfo) {
        self.height_map = undo_info.height_map;
        self.matrix = undo_info.matrix;
        self.holes = undo_info.holes;
    }

    pub fn add_garbage_line(&mut self, line: u16) {
        for i in (1..BOARD_HEIGHT).rev() {
            self.matrix[i as usize] = self.matrix[(i - 1) as usize];
        }
        self.matrix[0] = line;
        self.recalculate_metadata();
    }
    pub fn set_matrix(&mut self, matrix: [u16; BOARD_HEIGHT as usize]) {
        self.matrix = matrix;
        self.recalculate_metadata()
    }
    fn recalculate_metadata(&mut self) {
        self.holes = 0;
        for x in 0..BOARD_WIDTH {
            self.height_map[x as usize] = 0;
            let mut encountered_block = false;
            for j in (0..BOARD_HEIGHT).rev() {
                if (self.matrix[j as usize] >> x) & 1 == 1 {
                    if !encountered_block {
                        self.height_map[x as usize] = j + 1;
                    }
                    encountered_block = true;
                } else {
                    if encountered_block {
                        self.holes += 1;
                    }
                }
            }
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut res = String::new();
        for y in (0..BOARD_VISIBLE_HEIGHT).rev() {
            for x in 0..BOARD_WIDTH {
                let c = match self.matrix[y as usize] >> x & 1 {
                    0 => "░░",
                    1 => "██",
                    _ => panic!("Invalid match for matrix element"),
                };
                res.push_str(c);
            }
            if y != 0 {
                res.push_str("\n");
            }
        }
        write!(f, "{}", res)
    }
}

pub struct BoardDropOptions {
    pub shift: i32,
    pub rotation: i32,
}

pub struct BoardDropResult {
    pub lines_cleared: i32,
    pub perfect_clear: bool,
}

pub struct BoardUndoInfo {
    pub height_map: [i32; BOARD_WIDTH as usize],
    pub matrix: [u16; BOARD_HEIGHT as usize],
    pub holes: i32,
}
