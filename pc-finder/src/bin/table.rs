use std::{collections::HashSet, sync::atomic::AtomicUsize};

use common::*;
use pc_finder::*;

fn main() -> GenericResult<()> {
    let pruned = load_pruned()?;
    let mut visited = HashSet::new();
    let mut table = PcTable::new();
    for (i, &board) in pruned.iter().enumerate() {
        for piece in PieceType::all() {
            let key = PcTableKey::new(board, piece);
            let game = Game::from_parts(
                Board::from(board),
                Piece::from(piece),
                None,
                &[PieceType::O],
                true,
            );
            let child_states = game.child_states(FRAGMENTS);
            visited.clear();
            for child in child_states {
                let board = match PcBoard::try_from(child.game.board) {
                    Ok(board) => board,
                    Err(_) => continue,
                };
                if !pruned.contains(&board) || !visited.insert(board) {
                    continue;
                }
                let val = PcTableLeaf::new(board, child.moves);
                table.insert_leaf(key, val);
            }
            println!(
                "{}\nBoard: {:>6} Piece: {} Children: {:>3}\n",
                board,
                i,
                piece,
                visited.len()
            );
        }
    }
    save_pc_table(table.map())?;
    table.save_to_file("data/pc-table.bin")?;
    Ok(())
}
