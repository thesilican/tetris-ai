use common::*;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{hint::black_box, time::Instant};

fn bench_fn<F: FnMut(usize)>(name: &str, count: usize, mut f: F) {
    let start = Instant::now();
    for i in 0..count {
        f(i);
    }
    let end = Instant::now();
    let diff = (end - start) / count as u32;
    println!("- {name}: {diff:?}");
}

fn main() {
    println!("Starting bench report");
    let count = 1000;

    // DT Cannon Loop
    let actions = vec![
        GameAction::Hold,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::HardDrop,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::HardDrop,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCcw,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::RotateCcw,
        GameAction::RotateCcw,
        GameAction::SoftDrop,
        GameAction::RotateCcw,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::RotateCcw,
        GameAction::RotateCcw,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCcw,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::SoftDrop,
        GameAction::RotateCw,
        GameAction::HardDrop,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCcw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::RotateCcw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::RotateCcw,
        GameAction::HardDrop,
        GameAction::RotateCcw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
        GameAction::RotateCcw,
        GameAction::ShiftRight,
        GameAction::SoftDrop,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::ShiftRight,
        GameAction::HardDrop,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::HardDrop,
        GameAction::RotateCw,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::SoftDrop,
        GameAction::RotateCcw,
        GameAction::HardDrop,
        GameAction::Hold,
        GameAction::Rotate180,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::ShiftLeft,
        GameAction::HardDrop,
    ];
    bench_fn("dt_cannon_loop", count, |_| {
        let mut bag = Bag::new_fixed(&PieceType::ALL);
        let mut game = Game::from_bag(&mut bag);
        game.swap_hold();
        for _ in 0..100 {
            for &action in actions.iter() {
                game.apply_action(action);
                game.refill_queue(&mut bag);
            }
        }
        black_box(game);
    });

    // Child States
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

    bench_fn("gen_children", count, |i| {
        let game = scenarios[i];
        let child_states = game.children().unwrap();
        black_box(child_states);
    });
}
