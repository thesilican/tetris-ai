#![feature(once_cell)]
use common::*;
use pc_finder::*;
use std::{fs::File, lazy::SyncLazy};

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

fn add_piece_rec(board: PcBoard, pieces: [CanPiece; 10], len: usize, output: &mut Vec<Tess>) {
    for &piece in ALL_PIECES.iter() {
        if len >= 1 {
            if pieces[len - 1] > piece {
                continue;
            }
        }
        if board.intersects(&piece) {
            continue;
        }
        let mut board = board;
        board.lock(&piece);
        if parity_fail(&board) {
            continue;
        }

        let mut pieces = pieces;
        let mut len = len;
        pieces[len] = piece;
        len += 1;
        if len == 10 {
            output.push(Tess { pieces });
            println!("{}", output.len());
        } else {
            add_piece_rec(board, pieces, len, output);
        }
    }
}

fn save_tessellations(file_name: &str, tessellations: &[Tess]) {
    let file = File::create(file_name).unwrap();
    serde_json::to_writer(file, &tessellations).unwrap();
}

fn load_tessellations(file_name: &str) -> Vec<Tess> {
    let file = File::open(file_name).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn gen_tessellations() {
    // Generate tessellations
    let mut tessellations = Vec::new();
    add_piece_rec(PcBoard::new(), Default::default(), 0, &mut tessellations);
    save_tessellations("tess.json", &tessellations);

    // Filter tessellations
    let tessellations_7 = tessellations
        .into_iter()
        .filter(|tess| {
            let mut flags = [0; 7];
            for piece in tess.pieces {
                flags[piece.piece_type.to_i8() as usize] += 1;
            }
            flags.iter().all(|&x| 1 <= x && x <= 2)
        })
        .collect::<Vec<_>>();
    save_tessellations("tess-7.json", &tessellations_7)
}

fn main() {
    let tessellations = load_tessellations("tess-7.json");
    for tess in tessellations {
        println!("{}\n", tess);
    }
}
