use anyhow::Result;
use pc_finder::read_pc_table;

fn main() -> Result<()> {
    let table = read_pc_table()?;
    let mut pairs = table.map.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|&(k, _)| k);
    for (i, (key, val)) in pairs.iter().enumerate() {
        println!("{i} - {}\n{}\n", key.piece, key.board);
        for child in val {
            println!("Sequence: {:?}\n{}\n", child.actions(), child.board);
        }
        println!();
    }
    Ok(())
}
