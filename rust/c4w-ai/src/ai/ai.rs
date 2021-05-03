use crate::ai::computed::C4WTransitions;
use crate::ai::computed::C4W_TRANSITIONS;
use crate::ai::consts::LEFT_COL;
use crate::ai::consts::LR_MAX_HEIGHT;
use crate::ai::consts::LR_WIDTH;
use crate::ai::consts::RIGHT_COL;
use crate::model::board::Board;
use crate::model::game::Game;
use crate::model::game::GameMove;
use crate::model::piece::PieceType;
use lazy_static::lazy_static;

pub struct AI {
    pub upstack: bool,
}

lazy_static! {
    static ref HOLD_MOVE: Vec<GameMove> = vec![GameMove::Hold];
}

impl AI {
    pub fn new() -> Self {
        AI { upstack: true }
    }
    pub fn evaluate(&self, game: &Game) -> AIEvaluation {
        match C4WState::from_game(game) {
            Ok(state) => self.evaluate_recursive(&state),
            Err(C4WConstructErr::MissingHold) => AIEvaluation {
                score: 0.0,
                moves: &HOLD_MOVE,
            },
            Err(C4WConstructErr::InvalidBoard) => {
                todo!()
            }
        }
    }
    fn evaluate_recursive(&self, game: &C4WState) -> AIEvaluation {
        todo!();
    }
}

enum C4WConstructErr {
    MissingHold,
    InvalidBoard,
}
struct C4WState {
    pub left: (i8, i8, i8),
    pub center: u16,
    pub right: (i8, i8, i8),
    pub queue_ptr: usize,
    pub hold_piece: PieceType,
    pub queue_pieces: Vec<PieceType>,
}
impl C4WState {
    fn from_game(game: &Game) -> Result<Self, C4WConstructErr> {
        let hold_piece = match &game.hold_piece {
            Some(p) => p.piece_type.clone(),
            None => return Err(C4WConstructErr::MissingHold),
        };
        let mut queue_pieces = Vec::new();
        queue_pieces.push((&game.current_piece.piece_type).clone());
        queue_pieces.extend(game.queue_pieces.iter().map(|x| x.piece_type.clone()));

        let (left, center, right) = match C4WState::extract_state_from_board(&game.board) {
            Ok(res) => res,
            Err(_) => return Err(C4WConstructErr::InvalidBoard),
        };

        Ok(C4WState {
            left,
            center,
            right,
            queue_ptr: 0,
            hold_piece,
            queue_pieces,
        })
    }
    fn extract_state_from_board(board: &Board) -> Result<((i8, i8, i8), u16, (i8, i8, i8)), ()> {
        // Check if left/right are valid
        // Lol
        for is_left in &[true, false] {
            for i in 0..LR_WIDTH {
                let col = if *is_left {
                    i + LEFT_COL
                } else {
                    i + RIGHT_COL
                };
                if board.holes[col as usize] != 0 {
                    return Err(());
                }
                if board.height_map[col as usize] >= LR_MAX_HEIGHT {
                    return Err(());
                }
            }
        }
        let left = C4WTransitions::lr_get_state(&board, true);
        let right = C4WTransitions::lr_get_state(&board, false);

        // Check if center is valid
        let center = C4WTransitions::center_get_state(&board);
        if !C4W_TRANSITIONS.center.contains_key(&center) {
            return Err(());
        }

        Ok((left, center, right))
    }

    fn swap_hold(&mut self) {
        std::mem::swap(&mut self.hold_piece, &mut self.queue_pieces[self.queue_ptr]);
    }
    fn get_current_piece(&self) -> &PieceType {
        &self.queue_pieces[self.queue_ptr]
    }
}

pub struct AIEvaluation {
    pub score: f64,
    pub moves: &'static Vec<GameMove>,
}
