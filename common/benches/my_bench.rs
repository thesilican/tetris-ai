use common::*;
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

fn dt_cannon_loop(b: &mut Bencher) {
    // moves is 175 GameMoves long
    // Assumption: Queue starts with [O] I T L ...
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
    let mut bag = Bag::new_fixed(&PieceType::ALL);
    b.iter(|| {
        let mut game = Game::from_bag(&mut bag);
        game.swap_hold();
        for _ in 0..100 {
            for &game_move in moves.iter() {
                game.make_move(game_move);
                game.refill_queue(&mut bag);
            }
        }
        black_box(game);
    })
}

fn copy_game(b: &mut Bencher) {
    let mut bag = Bag::new_rng7(0);
    let game = Game::from_bag(&mut bag);
    b.iter(|| {
        for _ in 0..1000 {
            let copy = game;
            black_box(copy);
        }
    })
}

fn gen_child_states_f4(b: &mut Bencher) {
    let mut bag = Bag::new_rng7(0);
    let game = Game::from_bag(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_4F);
        black_box(children);
    })
}

fn gen_child_states_f2(b: &mut Bencher) {
    let mut bag = Bag::new_rng7(0);
    let game = Game::from_bag(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_2F);
        black_box(children);
    })
}

fn gen_child_states_f0(b: &mut Bencher) {
    let mut bag = Bag::new_rng7(0);
    let game = Game::from_bag(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_0F_NH);
        black_box(children);
    })
}

macro_rules! bench_function {
    ($c:ident, $i:ident) => {
        $c.bench_function(stringify!($i), $i)
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_function!(c, dt_cannon_loop);
    bench_function!(c, copy_game);
    bench_function!(c, gen_child_states_f0);
    bench_function!(c, gen_child_states_f2);
    bench_function!(c, gen_child_states_f4);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
