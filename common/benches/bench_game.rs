#![feature(test)]
extern crate test;
use common::model::Bag;
use common::model::Game;
use common::model::GameMove;
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
    let mut bag = Bag::new(0);
    b.iter(|| {
        let mut game = Game::from_bag(&mut bag, false);
        game.swap_hold();
        for _ in 0..100 {
            for game_move in moves.iter() {
                game.make_move(*game_move);
                game.refill_queue(&mut bag, false);
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
    let game = Game::from_bag(&mut bag, true);
    b.iter(|| {
        for _ in 0..1000 {
            let copy = game;
            black_box(copy);
        }
    })
}

/*
    Progress
    2021-07-11: 119,021 ns/iter (+/- 5,106)
*/
#[bench]
fn gen_child_states(b: &mut Bencher) {
    let mut bag = Bag::new(0);
    let game = Game::from_bag(&mut bag, true);
    b.iter(|| {
        let children = game.child_states_dr();
        black_box(children);
    })
}
