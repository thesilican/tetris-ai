use common::model::Bag;
use common::model::Game;
use common::model::GameMove;
use common::model::MOVES_0F_NH;
use common::model::MOVES_2F;
use common::model::MOVES_4F;
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
    let bag = Bag::new(0);
    b.iter(|| {
        let mut game = Game::from_bag(&bag);
        game.swap_hold();
        for _ in 0..100 {
            for game_move in moves.iter() {
                game.make_move(*game_move);
                game.refill_queue(&bag);
            }
        }
        black_box(game);
    })
}

fn copy_game(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        for _ in 0..1000 {
            let copy = game;
            black_box(copy);
        }
    })
}

fn gen_child_states_f4(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_4F);
        black_box(children);
    })
}

fn gen_child_states_f2(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_2F);
        black_box(children);
    })
}

fn gen_child_states_f0(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_0F_NH);
        black_box(children);
    })
}

fn gen_child_states_par_f4(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states_par(&MOVES_4F);
        black_box(children);
    })
}

fn gen_child_states_par_f2(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states_par(&MOVES_2F);
        black_box(children);
    })
}

fn gen_child_states_par_f0(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states_par(&MOVES_0F_NH);
        black_box(children);
    })
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dt_cannon_loop", dt_cannon_loop);
    c.bench_function("copy_game", copy_game);
    c.bench_function("gen_child_states_f4", gen_child_states_f4);
    c.bench_function("gen_child_states_f2", gen_child_states_f2);
    c.bench_function("gen_child_states_f0", gen_child_states_f0);
    c.bench_function("gen_child_states_par_f4", gen_child_states_par_f4);
    c.bench_function("gen_child_states_par_f2", gen_child_states_par_f2);
    c.bench_function("gen_child_states_par_f0", gen_child_states_par_f0);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
