use libtetris::*;
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
    let count = 10_000;

    // DT Cannon Loop
    let actions = vec![
        Action::Hold,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCw,
        Action::ShiftRight,
        Action::HardDrop,
        Action::HardDrop,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::HardDrop,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::Hold,
        Action::HardDrop,
        Action::Hold,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCcw,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::RotateCw,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::RotateCcw,
        Action::RotateCcw,
        Action::SoftDrop,
        Action::RotateCcw,
        Action::HardDrop,
        Action::RotateCw,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::RotateCcw,
        Action::RotateCcw,
        Action::HardDrop,
        Action::Hold,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCcw,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::SoftDrop,
        Action::RotateCw,
        Action::HardDrop,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::Hold,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCcw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::HardDrop,
        Action::Hold,
        Action::RotateCcw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::RotateCcw,
        Action::HardDrop,
        Action::RotateCcw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
        Action::RotateCcw,
        Action::ShiftRight,
        Action::SoftDrop,
        Action::HardDrop,
        Action::Hold,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::ShiftRight,
        Action::HardDrop,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::HardDrop,
        Action::RotateCw,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::SoftDrop,
        Action::RotateCcw,
        Action::HardDrop,
        Action::Hold,
        Action::Rotate180,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::ShiftLeft,
        Action::HardDrop,
    ];
    bench_fn("dt_cannon_loop", count / 4, |_| {
        let mut bag = Bag::new_fixed(&PieceType::ALL);
        let mut game = Game::from_bag(&mut bag);
        game.swap_hold();
        for _ in 0..100 {
            for &action in actions.iter() {
                game.apply(action);
                game.refill_queue(&mut bag);
            }
        }
        black_box(game);
    });

    // Generating child states
    let mut rng = StdRng::seed_from_u64(0);
    let mut scenarios = Vec::new();
    for i in 0..count {
        let mut bag = Bag::new_rng7(i as u64);
        let mut game = Game::from_bag(&mut bag);
        for _ in 0..3 {
            let mut child_states = game.children_fast();
            child_states.shuffle(&mut rng);
            game = child_states[0].game;
        }
        scenarios.push(game);
    }

    bench_fn("gen_children_fast", count, |i| {
        let game = scenarios[i];
        let child_states = game.children_fast();
        black_box(&child_states);
    });

    bench_fn("gen_children_0", count, |i| {
        let game = scenarios[i];
        let child_states = game.children(0);
        black_box(&child_states);
    });

    bench_fn("gen_children_1", count, |i| {
        let game = scenarios[i];
        let child_states = game.children(1);
        black_box(&child_states);
    });

    bench_fn("gen_children_2", count / 4, |i| {
        let game = scenarios[i];
        let child_states = game.children(2);
        black_box(&child_states);
    });

    bench_fn("gen_children_3", count / 16, |i| {
        let game = scenarios[i];
        let child_states = game.children(3);
        black_box(&child_states);
    });

    bench_fn("gen_children_4", count / 64, |i| {
        let game = scenarios[i];
        let child_states = game.children(4);
        black_box(&child_states);
    });
}
