use anyhow::Result;
use pc_finder::read_edges;

fn main() -> Result<()> {
    let edges = read_edges()?;
    for (i, (from, to)) in edges.iter().enumerate() {
        println!("{i}\n{from}\n         \\/\n{to}\n");
    }
    Ok(())
}
