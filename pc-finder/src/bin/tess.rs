#![feature(once_cell)]
use common::*;
use pc_finder::*;
use std::{collections::HashSet, lazy::SyncLazy};

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

fn gen_tessellations() -> Vec<Tess> {
    fn rec(
        board: PcBoard,
        pieces: [CanPiece; 10],
        len: usize,
        flags: [i8; 7],
        output: &mut Vec<Tess>,
    ) {
        for &piece in ALL_PIECES.iter() {
            let mut board = board;
            let mut pieces = pieces;
            let mut len = len;
            let mut flags = flags;
            flags[piece.piece_type.to_i8() as usize] += 1;
            if flags.iter().any(|&x| x > 2) {
                continue;
            }
            if flags.iter().filter(|&&x| x == 2).count() > 3 {
                continue;
            }
            if len >= 1 {
                if pieces[len - 1] >= piece {
                    continue;
                }
            }
            if board.intersects(&piece) {
                continue;
            }
            board.lock(&piece);
            if parity_fail(&board) {
                continue;
            }

            pieces[len] = piece;
            len += 1;
            if len == 10 {
                let tess = Tess::new(pieces);
                output.push(tess);
                println!("{}", output.len());
                println!("{}", tess);
            } else {
                rec(board, pieces, len, flags, output);
            }
        }
    }
    let mut output = Vec::new();
    rec(
        PcBoard::new(),
        [Default::default(); 10],
        0,
        [0; 7],
        &mut output,
    );
    {
        // Check that tessellations are unique
        let set = output.iter().map(|&x| x).collect::<HashSet<_>>();
        assert_eq!(set.len(), output.len());
    }
    output
}

fn main() -> GenericResult<()> {
    // Generate tessellations to be used by gen
    let tessellations = gen_tessellations();
    save_tessellations(&tessellations)
}
