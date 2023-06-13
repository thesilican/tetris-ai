use anyhow::Result;
use common::*;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::hint::black_box;

fn main() -> Result<()> {
    let count = 100_000;
    let mut rng = StdRng::seed_from_u64(0);
    let mut scenarios = Vec::new();
    for i in 0..count {
        let mut bag = Bag::new_rng7(i as u64);
        let mut game = Game::from_bag(&mut bag);
        for _ in 0..3 {
            let mut child_states = game.children().unwrap();
            child_states.shuffle(&mut rng);
            game = child_states[0].game;
        }
        scenarios.push(game);
    }
    for i in 0..count {
        let game = scenarios[i];
        let child_states = game.children().unwrap();
        black_box(child_states);
    }
    Ok(())
}
