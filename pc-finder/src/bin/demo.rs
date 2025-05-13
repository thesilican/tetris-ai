use anyhow::Result;
use libtetris::Ai;
use pc_finder::{read_pc_table, PcFinderAi};

fn main() -> Result<()> {
    let data = read_pc_table()?;
    PcFinderAi::new(data).demo();
    Ok(())
}
