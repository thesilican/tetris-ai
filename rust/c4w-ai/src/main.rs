use ai_api::APIMove;
use ai_api::APIOutput;
use c4w_ai::ai::ai::AI;
use c4w_ai::ai::computed::C4W_TRANSITIONS;
use c4w_ai::model::consts::BOARD_HEIGHT;
use c4w_ai::model::consts::BOARD_VISIBLE_HEIGHT;
use c4w_ai::model::game::Game;
use c4w_ai::model::game::GameMove;
use c4w_ai::model::piece::Piece;
use c4w_ai::model::piece::PieceType;

fn main() {
    // Force an evaluation
    drop(&C4W_TRANSITIONS.center);
    eprintln!("Hello from c4w-ai!");
    let mut game = Game::new();
    let ai = AI::new();

    // Utility function
    fn i32_to_piece(num: i32) -> Piece {
        Piece::new(PieceType::from_i32(num).unwrap())
    }
    loop {
        let input = ai_api::api_read().unwrap();
        game.set_current(i32_to_piece(input.current));
        game.set_hold(input.hold.map(i32_to_piece));
        game.set_queue(input.queue.into_iter().map(i32_to_piece).collect());
        let mut matrix = [0; BOARD_HEIGHT as usize];
        for i in 0..BOARD_VISIBLE_HEIGHT {
            matrix[i as usize] = input.matrix[i as usize];
        }
        game.board.set_matrix(matrix);

        // Evaluate
        let eval = ai.evaluate(&game);
        ai_api::api_write(APIOutput {
            score: Some(eval.score),
            moves: eval
                .moves
                .into_iter()
                .map(|x| match x {
                    GameMove::ShiftLeft => APIMove::ShiftLeft,
                    GameMove::ShiftRight => APIMove::ShiftRight,
                    GameMove::RotateLeft => APIMove::RotateLeft,
                    GameMove::RotateRight => APIMove::RotateRight,
                    GameMove::Rotate180 => APIMove::Rotate180,
                    GameMove::Hold => APIMove::Hold,
                    GameMove::SoftDrop => APIMove::SoftDrop,
                    GameMove::HardDrop => APIMove::HardDrop,
                })
                .collect(),
        })
        .unwrap();
    }
}
