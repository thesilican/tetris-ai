use common::api::ai::{TetrisAI, TetrisAIRes};
use common::model::game::Game;
use common::model::piece::{Piece, PieceType};
use rusty_ai::ai::RustyAI;
use rusty_ai::aiweights::{AIWeights, NUM_AI_WEIGHTS};

fn main() {
    let mut weights = AIWeights::new();
    weights.values = [
        1.0, // PC
        1.0, // 1 Line
        1.0, // 2 Line
        1.0, // 3 Line
        1.0, // 4 Line
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Holes
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Height
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Height Delta
    ];
    let mut game = Game::new();
    game.set_hold(Some(Piece::new(&PieceType::I)));
    extend_queue(&mut game);
    let mut ai = RustyAI::new(&weights, 3, 100);
    println!("{}", game);
    loop {
        let res = ai.api_evaluate(&mut game);
        if let TetrisAIRes::Success { moves, .. } = &res {
            for game_move in moves.iter() {
                game.make_move(game_move);
            }
            println!("{}", game);
            println!("{}", res);
        } else {
            println!("{}", res);
            break;
        }
        if game.queue_pieces.len() < 7 {
            extend_queue(&mut game);
        }
    }
}

fn extend_queue(game: &mut Game) {
    for piece_type in PieceType::iter_types() {
        game.append_queue(Piece::new(&piece_type));
    }
}
