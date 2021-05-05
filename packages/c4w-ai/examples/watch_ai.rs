use c4w_ai::ai::ai::C4WConstructErr;
use c4w_ai::ai::ai::AI;
use c4w_ai::model::game::Game;
use c4w_ai::model::game::GameMove;
use c4w_ai::model::piece::Piece;
use c4w_ai::model::piece::PieceType;

fn main() {
    let mut ai = AI::new(true);
    let mut game = Game::new();
    game.board.set(3, 0, true);
    game.board.set(4, 0, true);
    game.board.set(5, 0, true);
    for piece_type in PieceType::iter_types() {
        game.append_queue(Piece::new(piece_type));
    }
    game.make_move(&GameMove::Hold);
    println!("Starting loop");
    loop {
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        match ai.evaluate_game(&mut game, None) {
            Ok(eval) => {
                for game_move in eval.moves {
                    game.make_move(game_move).unwrap();
                }
            }
            Err(C4WConstructErr::MissingHold) => {
                game.make_move(&GameMove::Hold).unwrap();
            }
            Err(C4WConstructErr::InvalidBoard) => {
                println!("Invalid board");
            }
        }
    }
}
