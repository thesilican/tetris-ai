use std::collections::HashSet;

use common::*;
use pc_finder::*;

fn main() -> GenericResult<()> {
    let pruned = load_visited()?;
    let mut visited = HashSet::new();
    let mut table = PcTable::new();
    for (i, &board) in pruned.iter().enumerate() {
        for piece in PieceType::all() {
            let game = Game::from_parts(
                Board::from(board),
                Piece::from(piece),
                None,
                &[PieceType::O],
                true,
            );
            let key = PcTableKey::new(board, piece);
            let child_states = game.child_states(FRAGMENTS);
            visited.clear();
            for child in child_states {
                let board = match PcBoard::try_from(child.game.board) {
                    Ok(board) => board,
                    Err(_) => continue,
                };
                if !visited.insert(board) {
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
    let ser = table.base64_serialize();
    let de = PcTable::base64_deserialize(&ser)?;
    println!("{}", de.len());
    Ok(())
}
