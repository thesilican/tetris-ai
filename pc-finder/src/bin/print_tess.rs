use anyhow::Result;
use pc_finder::generate_tessellations;

fn main() -> Result<()> {
    let tessellations = generate_tessellations()?;
    for (i, tess) in tessellations.into_iter().enumerate() {
        println!("{i}\n{tess}");
    }
    Ok(())
}
