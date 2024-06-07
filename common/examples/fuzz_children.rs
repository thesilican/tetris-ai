use anyhow::{bail, Result};
use common::*;
use rand::{rngs::StdRng, RngCore, SeedableRng};

fn main() -> Result<()> {
    let count = 1_000_000;
    println!("Checking scenarios...\n0/{count}");
    for i in 0..count {
        let mut rng = StdRng::seed_from_u64(i);
        let mut bag = Bag::new_rng7(i as u64);
        let mut game = Game::from_bag(&mut bag);
        for _ in 0..5 {
            let children = game.children().unwrap();
            // Check children
            for child in children.iter() {
                let mut game = game;
                for actions in child.actions() {
                    game.apply_action(actions);
                }
                if game != child.game {
                    println!("Expected:\n{game}");
                    println!("Got:\n{}", child.game);
                    bail!("mismatched games");
                }
            }
            // Set game to random child
            let idx = rng.next_u64() as usize % children.len();
            game = children[idx].game;
        }
        println!("\x1b[F{i}/{count}")
    }
    println!("All correct!");
    Ok(())
}
