#![feature(once_cell)]
use common::*;
use pc_finder::*;
use std::{fs::File, lazy::SyncLazy};

// Canonical Piece
// Represents the shape of a particular piece in the 4 bottom
// rows of a board matrix
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

fn add_piece_rec(board: PcBoard, pieces: Tessellation, len: usize, output: &mut Vec<Tessellation>) {
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

fn gen_tessellations() {
    // Generate tessellations
    let mut tessellations = Vec::new();
    add_piece_rec(PcBoard::new(), Default::default(), 0, &mut tessellations);
    let file = File::create("tes.json").unwrap();
    serde_json::to_writer(file, &tessellations).unwrap();

    // Filter tessellations
    let filtered_tessellations = tessellations
        .into_iter()
        .filter(|tes| {
            let mut flags = [0; 7];
            for piece in tes {
                flags[piece.piece_type.to_i8() as usize] += 1;
            }
            flags.iter().all(|&x| 1 <= x && x <= 2)
        })
        .collect::<Vec<_>>();
    let file = File::create("tes-7.json").unwrap();
    serde_json::to_writer(file, &filtered_tessellations).unwrap();
}

fn print_tessellation(tes: Tessellation) {
    let mut output = String::new();
    for y in (0..4).rev() {
        for x in 0..10 {
            for p in tes {
                let text = match p.piece_type {
                    PieceType::O => "\x1b[33m[]\x1b[0m",
                    PieceType::I => "\x1b[36m[]\x1b[0m",
                    PieceType::T => "\x1b[37m[]\x1b[0m",
                    PieceType::L => "\x1b[30m[]\x1b[0m",
                    PieceType::J => "\x1b[34m[]\x1b[0m",
                    PieceType::S => "\x1b[32m[]\x1b[0m",
                    PieceType::Z => "\x1b[31m[]\x1b[0m",
                };
                if p.get(x, y) {
                    output.push_str(text);
                    break;
                }
            }
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn main() {
    gen_tessellations();
}
