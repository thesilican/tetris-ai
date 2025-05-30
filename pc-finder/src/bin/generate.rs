use std::fs;

use anyhow::Result;
use pc_finder::{explore_graph, generate_pc_table, generate_tessellations, prune_graph};

fn main() -> Result<()> {
    fs::create_dir_all("data/")?;
    let tessellations = generate_tessellations()?;
    let edges = explore_graph(tessellations)?;
    let pruned = prune_graph(edges)?;
    generate_pc_table(pruned)?;
    Ok(())
}
