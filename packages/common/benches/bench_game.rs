#![feature(test)]
extern crate test;
use common::model::game::Game;
use common::model::game::GameMove;
use common::model::piece::Piece;
use common::model::piece::PieceType;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::SeedableRng;
use test::Bencher;

fn get_rand_move(rng: &mut StdRng) -> GameMove {
    let distr = Uniform::new(0, 7);
    let num = distr.sample(rng);
    match num {
        0 => GameMove::ShiftLeft,
        1 => GameMove::ShiftRight,
        2 => GameMove::RotateLeft,
        3 => GameMove::RotateRight,
        4 => GameMove::Rotate180,
        5 => GameMove::Hold,
        6 => GameMove::SoftDrop,
        _ => unreachable!(),
    }
}

#[bench]
pub fn random_drops(b: &mut Bencher) {
    // Setup
    const NUM_PIECES: i32 = 1_000;
    const MOVES_PER_PIECE: i32 = 4;
    const CLEAR_INTERVAL: i32 = MOVES_PER_PIECE * 10;
    let mut rng = StdRng::seed_from_u64(100);
    let mut moves = Vec::new();
    for _ in 0..NUM_PIECES {
        for _ in 0..MOVES_PER_PIECE {
            moves.push(get_rand_move(&mut rng));
        }
        moves.push(GameMove::HardDrop);
    }
    b.iter(|| {
        let mut game = Game::new();
        for (i, game_move) in moves.iter().enumerate() {
            if i % CLEAR_INTERVAL as usize == 0 {
                // Clear the board every 10 moves
                game.board.set_cols([0; 10]);
            }
            game.make_move(&game_move);
        }
        if game.queue_pieces.len() < 7 {
            for piece_type in PieceType::iter_types() {
                game.append_queue(Piece::new(&piece_type));
            }
        }
    })
}

#[bench]
fn only_drop_o(b: &mut Bencher) {
    const NUM_PIECES: i32 = 1_000;
    let cycle = vec![
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
    ];
    b.iter(|| {
        let mut game = Game::new();
        for _ in 0..(NUM_PIECES / 5) {
            for game_move in cycle.iter() {
                game.make_move(&game_move);
            }
            if game.queue_pieces.len() < 7 {
                for _ in 0..7 {
                    game.append_queue(Piece::new(&PieceType::O));
                }
            }
        }
    })
}
