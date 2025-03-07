use anyhow::Result;
use pc_finder::read_pruned;

fn main() -> Result<()> {
    let edges = read_pruned()?;
    for (i, (from, to)) in edges.iter().enumerate() {
        println!("{i}\n{from}\n         \\/\n{to}\n");
    }
    Ok(())
}
