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
    let moves = vec![
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::RotateCCW,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::Rotate180,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
    ];
    bench_fn("dt_cannon_loop", count, |_| {
        let mut bag = Bag::new_fixed(&PieceType::ALL);
        let mut game = Game::from_bag(&mut bag);
        game.swap_hold();
        for _ in 0..100 {
            for &game_move in moves.iter() {
                game.make_move(game_move);
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
            let mut child_states = game.child_states(&PERMS_4F);
            child_states.shuffle(&mut rng);
            game = child_states[0].game;
        }
        scenarios.push(game);
    }

    bench_fn("gen_child_states_f0", count, |i| {
        let game = scenarios[i];
        let child_states = game.child_states(&PERMS_0F);
        black_box(child_states);
    });
    bench_fn("gen_child_states_f2", count, |i| {
        let game = scenarios[i];
        let child_states = game.child_states(&PERMS_2F);
        black_box(child_states);
    });
    bench_fn("gen_child_states_f4", count, |i| {
        let game = scenarios[i];
        let child_states = game.child_states(&PERMS_4F);
        black_box(child_states);
    });
}
