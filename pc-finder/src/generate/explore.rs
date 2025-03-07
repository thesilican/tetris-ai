use crate::{PcBoard, Tess};
use anyhow::{bail, Result};
use libtetris::{Board, Fin, Game, Pack, Piece, PieceType, BOARD_WIDTH};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Write},
};

/// Check whether the pieces on a board fit a given tesselation
fn board_fits_tess(board: PcBoard, tess: Tess) -> bool {
    #[inline]
    fn fits(test: [u16; 4], tess: Tess) -> bool {
        // Bits from board and inverted board masked by piece shape
        // Fails if there are both board bits and inverted board bits
        // otherwise succeeds
        for mask in tess.pieces.iter().map(|&x| x.rows) {
            let mut test_normal = test;
            for i in 0..4 {
                test_normal[i] &= mask[i];
            }
            let normal = test_normal.iter().any(|&x| x != 0);
            let mut test_invert = test;
            for i in 0..4 {
                test_invert[i] = !test_invert[i] & mask[i];
            }
            let invert = test_invert.iter().any(|&x| x != 0);
            if normal && invert {
                return false;
            }
        }
        true
    }

    // Number of empty rows
    let clear_rows = board.rows.iter().filter(|&&x| x == 0).count();
    const FULL_ROW: u16 = (1 << BOARD_WIDTH) - 1;

    static PERMS: &[&[[usize; 4]]] = &[
        // 0
        &[[0, 1, 2, 3]],
        // 1
        &[[4, 0, 1, 2], [0, 4, 1, 2], [0, 1, 4, 2], [0, 1, 2, 4]],
        // 2
        &[
            [4, 4, 0, 1],
            [4, 0, 4, 1],
            [4, 0, 1, 4],
            [0, 4, 4, 1],
            [0, 4, 1, 4],
            [0, 0, 4, 4],
        ],
        // 3
        &[[4, 4, 4, 0], [4, 0, 4, 4], [4, 4, 0, 4], [4, 4, 4, 0]],
        // 4
        &[],
    ];

    // Slice perms up to the number of empty rows
    for &perms in &PERMS[0..=clear_rows] {
        for &perm in perms {
            let get_row = |i: usize| {
                if perm[i] == 4 {
                    FULL_ROW
                } else {
                    board.rows[perm[i]]
                }
            };
            // Test is board.rows but with possible inserted full rows
            let test = [get_row(0), get_row(1), get_row(2), get_row(3)];
            if fits(test, tess) {
                return true;
            }
        }
    }
    false
}

fn explore_bfs(
    tessellations: Vec<Tess>,
    tess_stats: &mut HashMap<Tess, u64>,
) -> Vec<(PcBoard, PcBoard)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut edges = Vec::new();
    queue.push_back(PcBoard::new());

    while let Some(parent) = queue.pop_front() {
        if visited.contains(&parent) {
            continue;
        }
        visited.insert(parent);
        println!(
            "{}\n  Queue: {:>8}\nVisited: {:>8}\n",
            parent,
            queue.len(),
            visited.len()
        );

        for piece in PieceType::ALL {
            let game = Game::from_parts(
                Board::from(parent),
                Piece::from_piece_type(piece),
                None,
                &[PieceType::O],
                true,
            );
            let children = game.children(Fin::Full3);
            for child in children {
                let Ok(child) = PcBoard::try_from(child.game.board) else {
                    continue;
                };
                let mut found = false;
                for &tess in tessellations.iter() {
                    if board_fits_tess(child, tess) {
                        found = true;
                        *tess_stats.entry(tess).or_insert(0) += 1;
                        break;
                    }
                }
                if !found {
                    continue;
                }
                if !visited.contains(&child) {
                    queue.push_back(child)
                }
                edges.push((parent, child));
            }
        }
    }
    edges
}

// Use DFS to generate all directed edges of the pc board graph
pub fn explore_graph(tessellations: Vec<Tess>) -> Result<Vec<(PcBoard, PcBoard)>> {
    match read_edges() {
        Ok(edges) => return Ok(edges),
        Err(err) => println!("{err}"),
    }

    println!("Exploring graph edges");
    // Extra info: see which tessellation is the most used
    let mut tess_stats = HashMap::<Tess, u64>::new();
    let output = explore_bfs(tessellations, &mut tess_stats);

    println!("Saving tessellation stats to data/edges-tess-stats.bin");
    let bytes = tess_stats.pack_bytes();
    let mut file = File::create("data/edges-tess-stats.bin")?;
    file.write_all(&bytes)?;

    println!("Saving graph edges to data/edges.bin");
    let bytes = output.pack_bytes();
    let mut file = File::create("data/edges.bin")?;
    file.write_all(&bytes)?;

    Ok(output)
}

pub fn read_edges() -> Result<Vec<(PcBoard, PcBoard)>> {
    println!("Reading graph edges from data/edges.bin");
    let mut file = File::open("data/edges.bin")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Vec::<(PcBoard, PcBoard)>::unpack_bytes(&data)
}

pub fn read_tess_stats() -> Result<HashMap<Tess, u64>> {
    println!("Reading tessellation stats");
    let file = File::open("data/edges-tess-stats.bin");
    if let Ok(mut file) = file {
        println!("Reading tessellation states from data/edges-tess-stats.bin");
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let output = HashMap::<Tess, u64>::unpack_bytes(&data)?;
        return Ok(output);
    }
    bail!("Could not open file data/edges-tess-stats.bin");
}
