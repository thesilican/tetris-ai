#![feature(test)]
extern crate test;
use common::model::Bag;
use common::model::Game;
use common::model::GameMove;
use common::model::MOVES_0F;
use common::model::MOVES_2F;
use common::model::MOVES_4F;
use test::{black_box, Bencher};

/*
    Progress:
    2021-07-11: 449,750 ns/iter (+/- 7,400)
*/
#[bench]
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
        GameMove::RotateRight,
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
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::RotateLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
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

/*
    Progress:
    2021-07-11: 1,464 ns/iter (+/- 13)
*/
#[bench]
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

/*
    Progress:
    2021-07-11: 419,083 ns/iter (+/- 6,910)
    2021-07-13: 416,748 ns/iter (+/- 17,293)
*/
#[bench]
fn gen_child_states_f4(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_4F);
        black_box(children);
    })
}

/*
    Progress:
    2021-07-11: 67,499 ns/iter (+/- 728)
    2021-07-13: 63,911 ns/iter (+/- 594)
*/
#[bench]
fn gen_child_states_f2(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_2F);
        black_box(children);
    })
}

/*
    Progress:
    2021-07-11: 10,611 ns/iter (+/- 398)
    2021-07-13: 8,228 ns/iter (+/- 338)
*/
#[bench]
fn gen_child_states_0f(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag_shuffled(&mut bag);
    b.iter(|| {
        let children = game.child_states(&MOVES_0F);
        black_box(children);
    })
}
