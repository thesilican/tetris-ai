use anyhow::{bail, Result};
use libtetris::*;
use rand::{rngs::StdRng, RngCore, SeedableRng};

/// Verify that the output of children is equal to the moves generated
fn main() -> Result<()> {
    let count = 1_000_000;
    println!("Checking scenarios...\n0/{count}");
    for i in 0..count {
        let mut rng = StdRng::seed_from_u64(i);
        let mut bag = Bag::new_rng7(i as u64);
        let mut game = Game::from_bag(&mut bag);
        for _ in 0..5 {
            let children = game.children_fast();
            // Check children
            for child in children.iter() {
                let mut new_game = game;
                for actions in child.actions() {
                    new_game.apply(actions);
                }
                if new_game != child.game {
                    println!("Begin:\n{game}");
                    println!("Sequence: {:?}", child.actions().collect::<Vec<_>>());
                    println!("Expected:\n{new_game}");
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
