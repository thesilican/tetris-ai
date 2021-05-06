use c4w_ai::ai::ai::AI;
use c4w_ai::ai::computed::C4W_TRANSITION_INFO;
use c4w_ai::model::game::Game;
use c4w_ai::model::game::GameMove;
use c4w_ai::model::piece::Piece;
use c4w_ai::model::piece::PieceType;
use std::io::Write;

fn main() {
    // Force a calculation
    drop(&C4W_TRANSITION_INFO);
    let mut ai = AI::new(true);
    let mut game = Game::new();
    game.board.set(3, 0, true);
    game.board.set(4, 0, true);
    game.board.set(5, 0, true);
    for piece_type in PieceType::iter_types() {
        game.append_queue(Piece::new(piece_type));
    }
    game.make_move(&GameMove::Hold).unwrap();
    loop {
        print!("{}", game);
        std::io::stdout().flush().unwrap();
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        let eval = ai.evaluate_game(&mut game).unwrap();
        for game_move in eval.moves {
            game.make_move(&game_move).unwrap();
        }
        if game.queue_pieces.len() < 5 {
            for piece_type in PieceType::iter_types() {
                game.append_queue(Piece::new(piece_type));
            }
        }
    }
}
