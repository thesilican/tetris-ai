use crate::{NormPiece, PcBoard, Tess};
use anyhow::Result;
use libtetris::{ArrDeque, Pack, Piece, PieceInfo, PieceType};
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
};

// Generate all possible permutations of normalized pieces
fn generate_all_norm_pieces() -> Vec<NormPiece> {
    let mut pieces = Vec::new();
    let mut dups = HashSet::new();
    for piece_type in PieceType::ALL {
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
            let (min_x, max_x, min_y, max_y) = PieceInfo::location_bound(piece_type, rot);
            for y in min_y..=(max_y - 20) {
                for x in min_x..=max_x {
                    let piece = Piece::new(piece_type, rot, (x, y));
                    let normed = NormPiece::try_from(piece).unwrap();
                    if !dups.contains(&normed) {
                        pieces.push(normed);
                        dups.insert(normed);
                    }
                }
            }
        }
    }
    pieces
}

// Check whether a given board has valid parity
fn parity_check(board: &PcBoard) -> bool {
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
            queue.push_back((x, y)).unwrap();
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
                    queue.push_back((nx, ny)).unwrap();
                    visited[nx as usize][ny as usize] = true;
                }
            }
            if count % 4 != 0 {
                return false;
            }
        }
    }
    true
}

// Recursively iterate over all board combinations
fn recurse(
    board: PcBoard,
    pieces: [NormPiece; 10],
    len: usize,
    flags: [i8; 7],
    output: &mut Vec<Tess>,
    all_pieces: &[NormPiece],
) {
    for &piece in all_pieces.iter() {
        let mut board = board;
        let mut pieces = pieces;
        let mut len = len;
        let mut flags = flags;
        flags[piece.piece_type.to_u8() as usize] += 1;
        if flags.iter().any(|&x| x > 2) {
            continue;
        }
        if flags.iter().filter(|&&x| x == 2).count() > 3 {
            continue;
        }
        if len >= 1 && pieces[len - 1] >= piece {
            continue;
        }
        if board.intersects(&piece) {
            continue;
        }
        board.lock(&piece);
        if !parity_check(&board) {
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
            recurse(board, pieces, len, flags, output, all_pieces);
        }
    }
}

pub fn generate_tessellations() -> Result<Vec<Tess>> {
    println!("Generating tessellations");
    let file = File::open("data/tessellations.bin");
    if let Ok(mut file) = file {
        println!("Reading tessellations from data/tessellations.bin");
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let output: Vec<Tess> = Vec::<Tess>::unpack_bytes(&data)?;
        return Ok(output);
    }

    let all_pieces = generate_all_norm_pieces();
    let mut output = Vec::new();
    recurse(
        PcBoard::new(),
        [Default::default(); 10],
        0,
        [0; 7],
        &mut output,
        &all_pieces,
    );

    // Save to file
    println!("Saving tessellations to data/tessellations.bin");
    let bytes = output.pack_bytes();
    let mut file = File::create("data/tessellations.bin")?;
    file.write_all(&bytes)?;

    Ok(output)
}
