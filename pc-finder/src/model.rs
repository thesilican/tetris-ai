use common::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, convert::TryInto, fmt::Display, lazy::SyncLazy};

// Fragments used for generating child PcBoards
pub static FRAGMENTS: &SyncLazy<Fragments> = &MOVES_2F;

/// Represents the bottom 4 rows of a tetris board
/// Invariant: must be valid (see PcBoard::is_valid())
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "PcBoardSer")]
#[serde(into = "PcBoardSer")]
pub struct PcBoard([u16; 4]);
impl PcBoard {
    pub const fn new() -> Self {
        PcBoard([0; 4])
    }
    pub const fn from_rows(rows: [u16; 4]) -> Self {
        PcBoard(rows)
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.0[y as usize] >> x & 1 == 1
    }
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, on: bool) {
        if on {
            self.0[y as usize] |= 1 << x;
        } else {
            self.0[y as usize] &= !(1 << x);
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut queue = ArrDeque::<(i32, i32), 40>::new();
        let mut visited = [[false; 4]; 10];
        let mut parity_fail = false;
        'l: for x in 0..10 {
            for y in 0..4 {
                if visited[x as usize][y as usize] {
                    continue;
                }
                if self.get(x, y) {
                    continue;
                }

                // Mark adjacent cells as visited
                let mut count = 1;
                queue.push_back((x, y));
                visited[x as usize][y as usize] = true;
                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let (nx, ny) = (x + dx, y + dy);
                        if !(0..10).contains(&nx) || !(0..4).contains(&ny) {
                            continue;
                        }
                        if visited[nx as usize][ny as usize] {
                            continue;
                        }
                        if self.get(nx, ny) != self.get(x, y) {
                            continue;
                        }
                        count += 1;
                        queue.push_back((nx, ny));
                        visited[nx as usize][ny as usize] = true;
                    }
                }
                if count % 4 != 0 {
                    parity_fail = true;
                    break 'l;
                }
            }
        }
        if parity_fail && self.0[3] != 0 {
            return false;
        }
        true
    }

    pub fn child_boards(&self) -> Vec<PcBoard> {
        let mut result = HashSet::new();
        for piece_type in PieceType::all() {
            let game = Game::from_parts(
                (*self).into(),
                Piece::from(piece_type),
                None,
                &[PieceType::O],
                true,
            );
            let child_states = game.child_states(FRAGMENTS);
            let boards = child_states
                .into_iter()
                .filter_map(|x| PcBoard::try_from(x.game.board).ok());
            result.extend(boards);
        }
        result.into_iter().collect()
    }

    pub fn from_u64(val: u64) -> Self {
        PcBoard::from(PcBoardSer(val))
    }
    pub fn to_u64(self) -> u64 {
        PcBoardSer::from(self).0
    }
    pub fn from_i64(val: i64) -> Self {
        Self::from_u64(val as u64)
    }
    pub fn to_i64(self) -> i64 {
        self.to_u64() as i64
    }
}

impl TryFrom<Board> for PcBoard {
    type Error = ();

    /// Fails if the height of the board is greater than 4
    /// or if self is not valid
    fn try_from(value: Board) -> Result<Self, Self::Error> {
        if value.matrix[4] != 0 {
            return Err(());
        }
        let board = PcBoard(value.matrix[0..4].try_into().unwrap());
        if !board.is_valid() {
            return Err(());
        }
        Ok(board)
    }
}
impl From<PcBoard> for Board {
    fn from(pc_board: PcBoard) -> Self {
        let mut board = Board::new();
        for (i, row) in pc_board.0.into_iter().enumerate() {
            board.set_row(i, row);
        }
        board
    }
}
impl Display for PcBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '/' } else { '\n' };
        for y in (0..4).rev() {
            for x in 0..10 {
                let bit = if self.get(x, y) { "[]" } else { "▒▒" };
                write!(f, "{}", bit)?;
            }
            if y != 0 {
                write!(f, "{}", sep)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct PcBoardSer(u64);
impl From<PcBoard> for PcBoardSer {
    fn from(board: PcBoard) -> Self {
        let arr = board.0;
        let num = ((arr[0] as u64) << 0)
            + ((arr[1] as u64) << 16)
            + ((arr[2] as u64) << 32)
            + ((arr[3] as u64) << 48);
        PcBoardSer(num)
    }
}
impl From<PcBoardSer> for PcBoard {
    fn from(board: PcBoardSer) -> Self {
        let num = board.0;
        let bitmask: u64 = (1 << 16) - 1;
        let arr = [
            ((num >> 0) & bitmask) as u16,
            ((num >> 16) & bitmask) as u16,
            ((num >> 32) & bitmask) as u16,
            ((num >> 48) & bitmask) as u16,
        ];
        PcBoard(arr)
    }
}
