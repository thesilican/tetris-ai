use anyhow::Result;
use pc_finder::generate_tessellations;

fn main() -> Result<()> {
    let tess = generate_tessellations()?;
    Ok(())
}
