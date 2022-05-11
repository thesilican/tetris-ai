#![feature(once_cell)]
use common::*;
use pc_finder::PcBoard;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::lazy::SyncLazy;

// Represents the shape of a particular piece in the 4 bottom
// rows of a board matrix
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "CanPieceSer")]
#[serde(into = "CanPieceSer")]
struct CanPiece {
    piece_type: PieceType,
    rotation: i8,
    matrix: [u16; 4],
}
impl CanPiece {
    #[inline]
    fn get(&self, x: i32, y: i32) -> bool {
        self.matrix[y as usize] >> x & 1 == 1
    }
}
impl TryFrom<Piece> for CanPiece {
    type Error = GenericErr;

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        let bit_shape = piece.get_bit_shape(None, None);
        let (min_x, max_x, min_y, max_y) = piece.get_location_bounds(None);
        if piece.location.0 < min_x
            || piece.location.0 > max_x
            || piece.location.1 < min_y
            || piece.location.1 > max_y - 20
        {
            return Err(Default::default());
        }

        let mut matrix = [0; 4];
        for y in 0..4 {
            let i = y - piece.location.1;
            if 0 <= i && i < 4 {
                matrix[y as usize] = bit_shape[i as usize]
            }
        }
        Ok(CanPiece {
            piece_type: piece.piece_type,
            rotation: piece.rotation,
            matrix,
        })
    }
}
impl Display for CanPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '/' } else { '\n' };
        for y in (0..4).rev() {
            for x in 0..10 {
                let bit = if self.get(x, y) { "[]" } else { ".." };
                write!(f, "{}", bit)?;
            }
            if y != 0 {
                write!(f, "{}", sep)?;
            }
        }
        Ok(())
    }
}
impl PartialEq for CanPiece {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}
impl Eq for CanPiece {}
impl PartialOrd for CanPiece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self
            .piece_type
            .to_i8()
            .partial_cmp(&other.piece_type.to_i8())
        {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.rotation.partial_cmp(&other.rotation) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.matrix.partial_cmp(&other.matrix)
    }
}
impl Ord for CanPiece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Default for CanPiece {
    fn default() -> Self {
        Piece::new(PieceType::O, 0, (0, 0)).try_into().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CanPieceSer(u64);
impl From<CanPiece> for CanPieceSer {
    fn from(board: CanPiece) -> Self {
        let arr = board.matrix;
        let p_type = board.piece_type.to_i8();
        let rot = board.rotation;
        let num = ((arr[0] as u64) << 0)
            + ((arr[1] as u64) << 10)
            + ((arr[2] as u64) << 20)
            + ((arr[3] as u64) << 30)
            + ((p_type as u64) << 40)
            + ((rot as u64) << 50);
        CanPieceSer(num)
    }
}
impl From<CanPieceSer> for CanPiece {
    fn from(board: CanPieceSer) -> Self {
        let num = board.0;
        let bitmask: u64 = (1 << 10) - 1;
        let matrix = [
            ((num >> 0) & bitmask) as u16,
            ((num >> 10) & bitmask) as u16,
            ((num >> 20) & bitmask) as u16,
            ((num >> 30) & bitmask) as u16,
        ];
        let piece_type = ((num >> 40) & bitmask) as i8;
        let rotation = ((num >> 50) & bitmask) as i8;
        CanPiece {
            matrix,
            piece_type: piece_type.try_into().unwrap(),
            rotation,
        }
    }
}

#[inline]
fn intersects(board: &PcBoard, piece: &CanPiece) -> bool {
    board
        .0
        .iter()
        .zip(piece.matrix.iter())
        .any(|(&a, &b)| a & b != 0)
}

#[inline]
fn lock(board: &mut PcBoard, piece: &CanPiece) {
    for (b, p) in board.0.iter_mut().zip(piece.matrix.iter()) {
        *b |= *p;
    }
}

fn parity_fail(board: &PcBoard) -> bool {
    let mut queue = ArrDeque::<(i32, i32), 40>::new();
    let mut visited = [[false; 4]; 10];
    for x in 0..10 {
        for y in 0..4 {
            if visited[x as usize][y as usize] {
                continue;
            }
            if board.get(x, y) {
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
                    if board.get(nx, ny) != board.get(x, y) {
                        continue;
                    }
                    count += 1;
                    queue.push_back((nx, ny));
                    visited[nx as usize][ny as usize] = true;
                }
            }
            if count % 4 != 0 {
                return true;
            }
        }
    }
    false
}

static ALL_PIECES: SyncLazy<Vec<CanPiece>> = SyncLazy::new(|| {
    let mut pieces = Vec::new();
    for piece_type in PieceType::all() {
        let max_rot = match piece_type {
            PieceType::O => 1,
            PieceType::I => 2,
            PieceType::T => 4,
            PieceType::L => 4,
            PieceType::J => 4,
            PieceType::S => 2,
            PieceType::Z => 2,
        };
        for rot in 0..max_rot {
            let (min_x, max_x, min_y, max_y) = Piece::info_location_bounds(piece_type, rot);
            for y in min_y..=(max_y - 20) {
                for x in min_x..=max_x {
                    let piece = Piece::new(piece_type, rot, (x, y));
                    let can_piece = CanPiece::try_from(piece).unwrap();
                    pieces.push(can_piece);
                }
            }
        }
    }
    pieces
});

fn add_piece_rec(
    board: PcBoard,
    pieces: [CanPiece; 10],
    len: usize,
    output: &mut Vec<[CanPiece; 10]>,
) {
    for &piece in ALL_PIECES.iter() {
        if len >= 1 {
            if pieces[len - 1] > piece {
                continue;
            }
        }
        if intersects(&board, &piece) {
            continue;
        }
        let mut board = board;
        lock(&mut board, &piece);
        if parity_fail(&board) {
            continue;
        }

        let mut pieces = pieces;
        let mut len = len;
        pieces[len] = piece;
        len += 1;
        if len == 10 {
            output.push(pieces);
            println!("{}", output.len());
        } else {
            add_piece_rec(board, pieces, len, output);
        }
    }
}

fn main() {
    // let mut output = Vec::new();
    // add_piece_rec(PcBoard::new(), Default::default(), 0, &mut output);
    // let file = std::fs::File::create("out.json").unwrap();
    // serde_json::to_writer(file, &output).unwrap();

    // let file = std::fs::File::open("out.json").unwrap();
    // let res = serde_json::from_reader::<_, Vec<[CanPiece; 10]>>(file).unwrap();
    // println!("{:?}", res);
}
