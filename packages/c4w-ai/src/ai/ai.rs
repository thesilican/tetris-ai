use crate::ai::c4wstate::trim_lr;
use crate::ai::c4wstate::C4WConstructErr;
use crate::ai::c4wstate::C4WMoveOptions;
use crate::ai::c4wstate::C4WState;
use crate::ai::c4wstate::C4WStateChange;
use crate::ai::computed::C4W_TRANSITION_INFO;
use crate::ai::consts::AI_MAX_DEPTH;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::game::Game;
use crate::model::game::GameMove;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use ai_api::APIError;
use ai_api::APIMove;
use ai_api::APIRequest;
use ai_api::APIResponse;
use ai_api::TetrisAI;
use std::convert::TryFrom;
use std::convert::TryInto;

struct AIEval {
    total: i32,
    similar: i32,
    best_child: Option<C4WMoveOptions>,
}
pub struct AIGameEval {
    pub score: Option<f64>,
    pub moves: Vec<GameMove>,
}
#[derive(Debug)]
pub struct AIGameEvalErr(pub String);
impl From<&str> for AIGameEvalErr {
    fn from(text: &str) -> AIGameEvalErr {
        AIGameEvalErr(String::from(text))
    }
}

pub struct AI {
    pub debug: bool,
    pub upstack: bool,
}
impl AI {
    pub fn new(debug: bool) -> Self {
        AI {
            debug,
            upstack: false,
        }
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
    fn evaluate_recursive(&self, mut state: &mut C4WState, upstack: bool, depth: i32) -> AIEval {
        let is_last = state.queue_ptr == state.queue_pieces.len() - 1;
        if is_last || depth <= 1 {
            return AIEval {
                total: 1,
                similar: 1,
                best_child: None,
            };
        }

        let mut total = 0;
        let mut similar = 0;

        let mut best_no_transitions = false;
        let mut best_score = 0;
        let mut best_child = None;

        // Special case: May not downstack if any of the left/right columns is 3 or less
        let may_downstack = state.left.0 > 3
            && state.left.1 > 3
            && state.left.2 > 3
            && state.right.0 > 3
            && state.right.1 > 3
            && state.right.2 > 3;

        let iter_moves = state.get_moves().collect::<Vec<_>>();
        for state_move in iter_moves.into_iter() {
            // XNOR
            let move_is_upstack = state_move.change.is_upstack();
            let is_transitional = move_is_upstack != upstack;

            if !move_is_upstack && !may_downstack {
                continue;
            }

            let undo_info = state.make_move(&state_move).unwrap();
            let res = self.evaluate_recursive(&mut state, move_is_upstack, depth - 1);
            state.undo_move(&undo_info);

            // Evaluate result
            total += res.total;
            if !is_transitional {
                similar += res.similar;
            }

            // Update best move
            if !is_transitional && (!best_no_transitions || res.similar > best_score) {
                best_no_transitions = true;
                best_score = res.total;
                best_child = Some(state_move);
            } else if !best_no_transitions && res.total > best_score {
                best_score = res.total;
                best_child = Some(state_move);
            }
        }

        AIEval {
            total,
            similar,
            best_child,
        }
    }

    pub fn evaluate_game(&mut self, game: &mut Game) -> Result<AIGameEval, AIGameEvalErr> {
        let mut state = match C4WState::from_game(game) {
            Ok(state) => state,
            Err(C4WConstructErr::HoldIsNone) => {
                return Ok(AIGameEval {
                    score: None,
                    moves: vec![GameMove::Hold],
                })
            }
            Err(C4WConstructErr::InvalidBoard) => todo!("Implement invalid state"),
        };
        let eval = self.evaluate_recursive(&mut state, self.upstack, AI_MAX_DEPTH);
        // Determine game moves
        let move_options = match eval.best_child {
            Some(move_options) => move_options,
            None => return Err("Evaluation returned no child state".into()),
        };

        if match move_options.change {
            C4WStateChange::Left((a, b, c)) => a == 8 && b == 7 && c == 8,
            _ => false,
        } {
            println!();
        }

        self.upstack = move_options.change.is_upstack();
        let piece = if move_options.hold {
            state.get_hold_piece()
        } else {
            state.get_current_piece()
        };
        let moves = match move_options.change {
            C4WStateChange::Left(new_state) => {
                let (old_state, _) = trim_lr(state.left);
                let (new_state, _) = trim_lr(new_state);
                C4W_TRANSITION_INFO
                    .left
                    .get(&old_state)
                    .unwrap()
                    .get(piece)
                    .unwrap()
                    .get(&new_state)
                    .unwrap()
            }
            C4WStateChange::Center(new_state) => {
                let old_state = state.center;
                C4W_TRANSITION_INFO
                    .center
                    .get(&old_state)
                    .unwrap()
                    .get(piece)
                    .unwrap()
                    .get(&new_state)
                    .unwrap()
            }
            C4WStateChange::Right(new_state) => {
                let (old_state, _) = trim_lr(state.right);
                let (new_state, _) = trim_lr(new_state);
                C4W_TRANSITION_INFO
                    .right
                    .get(&old_state)
                    .unwrap()
                    .get(piece)
                    .unwrap()
                    .get(&new_state)
                    .unwrap()
            }
        };
        let moves = moves
            .iter()
            .map(|x| (*x).clone())
            .chain(std::iter::once(GameMove::HardDrop));
        // Prepend hold move if hold
        let moves = if move_options.hold {
            std::iter::once(GameMove::Hold).chain(moves).collect()
        } else {
            moves.collect()
        };
        if self.debug {
            eprintln!(
                "Eval Result: New state: {:?} Hold: {} Total: {}, Similar: {}",
                &move_options.change, move_options.hold, eval.total, eval.similar
            );
            eprintln!("Moves: {:?}", &moves);
        }
        Ok(AIGameEval {
            score: Some(eval.total.into()),
            moves,
        })
    }
}

// Look at all these conversions
impl TryFrom<APIRequest> for Game {
    type Error = APIError;
    fn try_from(req: APIRequest) -> Result<Game, APIError> {
        let i32_to_piece = |x| match PieceType::from_i32(x) {
            Ok(p) => Ok(Piece::new(p)),
            Err(_) => Err("Invalid piece type"),
        };
        let current_piece = i32_to_piece(req.current)?;
        let hold_piece = match req.hold {
            Some(p) => Some(i32_to_piece(p)?),
            None => None,
        };
        let mut queue_pieces = vec![];
        for piece in req.queue {
            queue_pieces.push(i32_to_piece(piece)?);
        }
        let mut matrix = [0; BOARD_HEIGHT as usize];
        for (i, row) in req.matrix.iter().enumerate() {
            matrix[i] = *row;
        }

        let mut game = Game::new();
        game.set_current(current_piece);
        game.set_hold(hold_piece);
        game.set_queue(queue_pieces);
        game.board.set_matrix(matrix);
        Ok(game)
    }
}
impl From<AIGameEval> for APIResponse {
    fn from(eval: AIGameEval) -> APIResponse {
        APIResponse {
            score: eval.score,
            moves: eval.moves.iter().map(|x| x.into()).collect(),
        }
    }
}
impl From<&GameMove> for APIMove {
    fn from(game_move: &GameMove) -> APIMove {
        match game_move {
            GameMove::ShiftLeft => APIMove::ShiftLeft,
            GameMove::ShiftRight => APIMove::ShiftRight,
            GameMove::RotateLeft => APIMove::RotateLeft,
            GameMove::RotateRight => APIMove::RotateRight,
            GameMove::Rotate180 => APIMove::Rotate180,
            GameMove::Hold => APIMove::Hold,
            GameMove::SoftDrop => APIMove::SoftDrop,
            GameMove::HardDrop => APIMove::HardDrop,
        }
    }
}
impl From<AIGameEvalErr> for APIError {
    fn from(err: AIGameEvalErr) -> APIError {
        APIError(err.0)
    }
}
impl TetrisAI for AI {
    fn evaluate(&mut self, req: APIRequest) -> Result<APIResponse, APIError> {
        let mut game = req.try_into()?;
        match self.evaluate_game(&mut game) {
            Ok(eval) => Ok(eval.into()),
            Err(err) => Err(err.into()),
        }
    }
}
