use common::model::consts::PIECE_NUM_TYPES;
use common::model::game::Game;
use common::model::game::GameMove;
use common::model::piece::Piece;
use common::model::piece::PieceType;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::time::Instant;

fn main() {
    const NUM_PIECES: i32 = 1_000_000;
    const MOVES_PER_PIECE: i32 = 10;
    const CLEAR_INTERVAL: i32 = MOVES_PER_PIECE * 10;

    let mut rng = StdRng::seed_from_u64(100);
    let mut game = Game::new();
    let mut moves = Vec::new();
    game.set_current(get_rand_piece(&mut rng));
    game.set_hold(Some(get_rand_piece(&mut rng)));
    for _ in 0..(NUM_PIECES - 1) {
        game.append_queue(get_rand_piece(&mut rng));
    }
    for _ in 0..NUM_PIECES {
        for _ in 0..(MOVES_PER_PIECE - 1) {
            moves.push(get_rand_move(&mut rng));
        }
        moves.push(GameMove::HardDrop);
    }

    let start = Instant::now();
    for (i, game_move) in moves.into_iter().enumerate() {
        if i % CLEAR_INTERVAL as usize == 0 {
            // Clear the board every 10 moves
            game.board.set_cols([0; 10]);
        }
        game.make_move(&game_move).ok();
    }
    let end = start.elapsed();
    assert!(game.queue_pieces.len() == 0);
    println!("Made {} moves in {:?}", NUM_PIECES * MOVES_PER_PIECE, end);
}

fn get_rand_piece(rng: &mut StdRng) -> Piece {
    let distr = Uniform::new(0, PIECE_NUM_TYPES);
    let num = distr.sample(rng);
    Piece::new(&PieceType::from_i32(num).unwrap())
}

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
