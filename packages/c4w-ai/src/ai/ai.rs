use crate::ai::computed::C4WTransitions;
use crate::ai::computed::C4W_TRANSITIONS;
use crate::ai::consts::AI_MAX_DEPTH;
use crate::ai::consts::LEFT_COL;
use crate::ai::consts::LR_MAX_HEIGHT;
use crate::ai::consts::LR_WIDTH;
use crate::ai::consts::RIGHT_COL;
use crate::ai::gamestate::GameStateMoveOptions;
use crate::model::board::Board;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::game::Game;
use crate::model::game::GameMove;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use ai_api::APIError;
use ai_api::APIMove;
use ai_api::APIRequest;
use ai_api::APIResponse;
use ai_api::TetrisAI;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;

pub struct AIEvaluation {
    pub similar: i32,
    pub total: i32,
    pub game_state_move: Option<GameStateMoveOptions>,
}

pub struct AIGameEvaluation {
    pub score: i32,
    pub moves: Vec<GameMove>,
}
impl From<AIGameEvaluation> for APIResponse {
    fn from(eval: AIGameEvaluation) -> APIResponse {
        APIResponse {
            score: Some(eval.score.into()),
            moves: eval.moves.into_iter().map(|x| x.into()).collect(),
        }
    }
}

pub struct AI {
    pub game: Game,
    pub debug: bool,
    pub upstack: bool,
}

impl TetrisAI for AI {
    fn evaluate(&mut self, request: APIRequest) -> Result<APIResponse, APIError> {
        let get_piece =
            |num: i32| -> Result<Piece, ()> { Ok(Piece::new(PieceType::from_i32(num)?)) };

        self.game.set_current(get_piece(request.current)?);
        self.game.set_hold(match request.hold {
            Some(hold) => Some(get_piece(hold)?),
            None => None,
        });
        self.game.clear_queue();
        for piece in request.queue {
            self.game.append_queue(get_piece(piece)?);
        }

        let mut matrix = [0; BOARD_HEIGHT as usize];
        for (i, row) in request.matrix.iter().enumerate() {
            matrix[i] = *row;
        }
        self.game.board.set_matrix(matrix);

        // let eval = self.evaluate_game(&mut game, None);
        // match eval {
        //     Ok(eval) => Ok(APIResponse {
        //         moves: eval
        //             .moves
        //             .iter()
        //             .map(|x| match x {
        //                 GameMove::ShiftLeft => APIMove::ShiftLeft,
        //                 GameMove::ShiftRight => APIMove::ShiftRight,
        //                 GameMove::RotateLeft => APIMove::RotateLeft,
        //                 GameMove::RotateRight => APIMove::RotateRight,
        //                 GameMove::Rotate180 => APIMove::Rotate180,
        //                 GameMove::Hold => APIMove::Hold,
        //                 GameMove::SoftDrop => APIMove::SoftDrop,
        //                 GameMove::HardDrop => APIMove::HardDrop,
        //             })
        //             .collect(),
        //         score: Some(eval.total.into()),
        //     }),
        //     Err(C4WConstructErr::MissingHold) => Ok(APIResponse {
        //         score: None,
        //         moves: vec![APIMove::Hold],
        //     }),
        //     Err(C4WConstructErr::InvalidBoard) => {
        //         panic!("Not implemented yet")
        //     }
        // }
    }
}
impl AI {
    pub fn new(debug: bool) -> Self {
        AI {
            game: Game::new(),
            upstack: true,
            debug,
        }
    }
    pub fn evaluate_game(&mut self) -> Result<AIEvaluation, C4WConstructErr> {
        let mut state = match C4WState::from_game(&game) {
            Ok(state) => state,
            Err(err) => return Err(err),
        };

        let eval = self.evaluate_recursive(&mut state, AI_MAX_DEPTH, self.upstack);

        if eval.similar == 0 && eval.total != 0 {
            self.upstack = !self.upstack;
        }

        // if debug.unwrap_or(self.debug) {
        //     for game_move in moves.iter() {
        //         game.make_move(game_move).ok();
        //     }
        //     print_state(&game);
        // }
        Ok(eval)
    }
    // Ok, so here's how this function works:
    // evaluate_recursive() takes a game state, a max depth counter, and an upstack flag
    //      The upstack bool tells you whether or not the evaluation is currently trying to upstack or downstack
    // The function then calculates 2 things for that state:
    //      - The number of total child states, including that transition
    //      - The number of child states that dont transition (always upstack/downstack)
    // Simultaneously and unrelatedly, the function picks the best child state, using the following heuristic:
    //      The best state is the state with the most non-transitional child states
    //      If there does not exist any child states with no transitions, then
    //      the best state is the state with the most child states, including transitions
    // The moves to obtain the best child state are returned
    fn evaluate_recursive(&self, game: &mut C4WState, depth: i32, upstack: bool) -> AIEvaluation {
        let is_last = game.queue_ptr == game.queue_pieces.len() - 1;
        if is_last || depth <= 1 {
            return AIEvaluation {
                total: 1,
                similar: 1,
                child_state: AIChildState::None,
            };
        }

        let mut total = 0;
        let mut similar = 0;

        let mut best_is_transitional = true;
        let mut best_score = i32::MIN;
        let mut best_child_state = AIChildState::None;

        // Whether or not upstacking/downstacking is transitional
        let downstack_transitional = upstack;
        let upstack_transitional = !upstack;

        // Go through all center child states
        let mut is_held = false;
        for (hold, state) in self.get_center_transitions(&game) {
            let old_state = game.center.clone();
            game.center = state;
            if is_held != hold {
                game.swap_hold();
                is_held = !is_held;
            }
            game.queue_ptr += 1;
            // Obtain result
            let res = self.evaluate_recursive(game, depth - 1, false);
            // Increment total/similar
            if !downstack_transitional {
                similar += res.similar;
            }
            total += res.total;

            // Update best move
            if !downstack_transitional && res.similar > best_score {
                best_is_transitional = false;
                best_score = res.similar;
                best_child_state = AIChildState::Center(state);
            } else if best_is_transitional && res.total > best_score {
                best_score = res.total;
                best_child_state = AIChildState::Center(state);
            }

            game.queue_ptr -= 1;
            game.center = old_state;
        }
        for (hold, right, state) in self.get_lr_transitions(&game) {
            let old_state = if right {
                let old_state = game.right.clone();
                game.right = state;
                old_state
            } else {
                let old_state = game.left.clone();
                game.left = state;
                old_state
            };
            if is_held != hold {
                game.swap_hold();
                is_held = !is_held;
            }
            game.queue_ptr += 1;

            // Obtain result
            let res = self.evaluate_recursive(game, depth - 1, true);
            // Increment total/similar
            if !upstack_transitional {
                similar += res.similar;
            }
            total += res.total;
            // Update best move
            if !upstack_transitional && res.similar > best_score {
                best_is_transitional = false;
                best_score = res.similar;
                best_child_state = if right {
                    AIChildState::Right(state)
                } else {
                    AIChildState::Left(state)
                };
            } else if best_is_transitional && res.total > best_score {
                best_score = res.total;
                best_child_state = if right {
                    AIChildState::Right(state)
                } else {
                    AIChildState::Left(state)
                };
            }

            game.queue_ptr -= 1;
            if right {
                game.right = old_state
            } else {
                game.left = old_state
            };
        }

        AIEvaluation {
            total,
            similar,
            child_state: best_child_state,
        }
    }
}

// Utility function to print the game
fn print_state(game: &Game) {
    let mut text = String::new();
    let piece = &game.current_piece;
    let board = &game.board;

    // Print board/shape combo
    let piece_shape = piece.get_shape(None);
    let (p_x, p_y) = piece.location;
    for j in (0..BOARD_HEIGHT).rev() {
        for i in 0..BOARD_WIDTH {
            let in_piece_bounds = i - p_x >= 0
                && i - p_x < PIECE_SHAPE_SIZE
                && j - p_y >= 0
                && j - p_y < PIECE_SHAPE_SIZE;
            let in_piece = in_piece_bounds && piece_shape[(i - p_x) as usize][(j - p_y) as usize];

            if in_piece {
                write!(text, "██").unwrap();
            } else if board.get(i, j) {
                write!(text, "▓▓").unwrap();
            } else if in_piece_bounds {
                write!(text, "▒▒").unwrap();
            } else {
                write!(text, "░░").unwrap();
            }
        }
        writeln!(text).unwrap();
    }
    // Board height/holes info
    for i in 0..BOARD_WIDTH {
        let height = board.height_map[i as usize];
        write!(text, "{:2}", height).unwrap();
    }
    writeln!(text).unwrap();
    for i in 0..BOARD_WIDTH {
        let hole = board.holes[i as usize];
        write!(text, "{:2}", hole).unwrap();
    }
    writeln!(text).unwrap();
    // Other info
    let curr = &game.current_piece.to_string();
    let hold = match &game.hold_piece {
        Some(piece) => piece.to_string(),
        None => String::from("null"),
    };
    let mut queue_text = String::new();
    for piece in &game.queue_pieces {
        queue_text.push_str(&piece.to_string());
        queue_text.push(' ');
    }
    writeln!(text, "Curr: {} Hold: {} Queue: {}", curr, hold, queue_text).unwrap();
    writeln!(
        text,
        "Can hold: {} Was hold empty: {}",
        game.can_hold, game.hold_was_empty
    )
    .unwrap();
    // Print
    println!("{}", text);
}
