use crate::{PcBoard, PcTable, PcTableChild};
use anyhow::Result;
use libtetris::{Board, Game, Pack, Piece, PieceType};
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
};
use tinyvec::TinyVec;

fn construct_table(pruned: Vec<(PcBoard, PcBoard)>) -> PcTable {
    let mut boards = HashSet::new();
    for (parent, _) in pruned {
        boards.insert(parent);
    }

    let mut table = PcTable::new();
    let mut visited = HashSet::new();
    for (i, &parent) in boards.iter().enumerate() {
        for piece in PieceType::ALL {
            let game = Game::from_parts(
                Board::from(parent),
                Piece::from_piece_type(piece),
                None,
                &[PieceType::O],
                true,
            );
            let children = game.children(4);
            visited.clear();
            for child_state in children {
                let Ok(child) = PcBoard::try_from(child_state.game.board) else {
                    continue;
                };
                if !boards.contains(&child) || !visited.contains(&child) {
                    continue;
                }
                visited.insert(child);
                let actions = child_state.actions().collect::<TinyVec<[_; 8]>>();
                table.insert_child(parent, piece, PcTableChild::new(child, actions));
            }
            println!(
                "{}\nBoard: {:>6} Piece: {} Children: {:>3}\n",
                parent,
                i,
                piece,
                visited.len()
            );
        }
    }
    table
}

pub fn generate_pc_table(pruned: Vec<(PcBoard, PcBoard)>) -> Result<PcTable> {
    println!("Constructing PcTable");
    let file = File::open("data/pc-table.bin");
    if let Ok(mut file) = file {
        println!("Reading PcTable from data/pc-table.bin");
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let output = PcTable::unpack_bytes(&data)?;
        return Ok(output);
    }

    let output = construct_table(pruned);

    println!("Saving PcTable to data/pc-table.bin");
    let bytes = output.pack_bytes();
    let mut file = File::create("data/pc-table.bin")?;
    file.write_all(&bytes)?;

    Ok(output)
}
