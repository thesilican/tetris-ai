use anyhow::Result;
use pc_finder::read_tess_stats;

fn main() -> Result<()> {
    let tess_stats = read_tess_stats()?;
    let mut tess_stats = tess_stats.into_iter().collect::<Vec<_>>();
    tess_stats.sort_by_key(|&(_, count)| count);
    for (tess, count) in tess_stats {
        println!("{count}\n{tess}")
    }
    Ok(())
}
