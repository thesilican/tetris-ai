use crate::ai::computed::C4WTransitions;
use crate::ai::computed::C4W_TRANSITIONS;
use crate::ai::consts::LEFT_COL;
use crate::ai::consts::LR_MAX_HEIGHT;
use crate::ai::consts::LR_WIDTH;
use crate::ai::consts::RIGHT_COL;
use crate::model::board::Board;
use crate::model::game::Game;
use crate::model::piece::PieceType;

enum GameStateConstructErr {
    MissingHold,
    InvalidBoard,
}
enum GameStateTransition {
    Left((i8, i8, i8)),
    Center(u16),
    Right((i8, i8, i8)),
}

struct GameStateMoveOptions {
    transition: GameStateTransition,
    hold: bool,
}
struct GameStateUndoInfo {
    prev_state: GameStateTransition,
    hold: bool,
}

struct GameState {
    pub left: (i8, i8, i8),
    pub center: u16,
    pub right: (i8, i8, i8),
    pub queue_ptr: usize,
    pub hold_piece: PieceType,
    pub queue_pieces: Vec<PieceType>,
}
impl GameState {
    fn from_game(game: &Game) -> Result<Self, GameStateConstructErr> {
        let hold_piece = match &game.hold_piece {
            Some(p) => p.piece_type.clone(),
            None => return Err(GameStateConstructErr::MissingHold),
        };
        let mut queue_pieces = Vec::new();
        queue_pieces.push((&game.current_piece.piece_type).clone());
        queue_pieces.extend(game.queue_pieces.iter().map(|x| x.piece_type.clone()));

        let (left, center, right) = match GameState::extract_state_from_board(&game.board) {
            Ok(res) => res,
            Err(_) => return Err(GameStateConstructErr::InvalidBoard),
        };

        Ok(GameState {
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

    fn get_current_piece(&self) -> &PieceType {
        &self.queue_pieces[self.queue_ptr]
    }
    fn make_move(
        &mut self,
        game_state_move: &GameStateMoveOptions,
    ) -> Result<GameStateUndoInfo, ()> {
        if self.queue_ptr == self.queue_pieces.len() - 1 {
            return Err(());
        }
        if game_state_move.hold {
            self.swap_hold();
        }
        self.queue_ptr += 1;
        let prev_state = self.apply_transition(&game_state_move.transition);
        Ok(GameStateUndoInfo {
            hold: game_state_move.hold,
            prev_state,
        })
    }
    fn undo_move(&mut self, undo_info: GameStateUndoInfo) {
        self.apply_transition(&undo_info.prev_state);
        self.queue_ptr -= 1;
        if undo_info.hold {
            self.swap_hold();
        }
    }
    fn apply_transition(&mut self, transition: &GameStateTransition) -> GameStateTransition {
        match transition {
            GameStateTransition::Left(state) => {
                let old_state = self.left.clone();
                self.left = *state;
                GameStateTransition::Left(old_state)
            }
            GameStateTransition::Center(state) => {
                let old_state = self.center.clone();
                self.center = *state;
                GameStateTransition::Center(old_state)
            }
            GameStateTransition::Right(state) => {
                let old_state = self.right.clone();
                self.right = *state;
                GameStateTransition::Right(old_state)
            }
        }
    }
    fn swap_hold(&mut self) {
        std::mem::swap(&mut self.hold_piece, &mut self.queue_pieces[self.queue_ptr]);
    }

    fn get_lr_transitions(&self) -> impl Iterator<Item = GameStateMoveOptions> {
        // Given a game state, iterates all possible new left/right child states

        // Get left/right states
        let mut left_state = self.left.clone();
        let left_height = std::cmp::min(left_state.0, std::cmp::min(left_state.1, left_state.2));
        left_state.0 -= left_height;
        left_state.1 -= left_height;
        left_state.2 -= left_height;
        let left_transitions = match C4W_TRANSITIONS.left.get(&left_state) {
            None => panic!("Unknown game left state {:?}", &self.left),
            Some(p) => p,
        };

        let mut right_state = self.right.clone();
        let right_height =
            std::cmp::min(right_state.0, std::cmp::min(right_state.1, right_state.2));
        right_state.0 -= right_height;
        right_state.1 -= right_height;
        right_state.2 -= right_height;
        let right_transitions = match C4W_TRANSITIONS.right.get(&right_state) {
            None => panic!("Unknown game right state {:?}", &self.right),
            Some(p) => p,
        };

        // Get current/hold piece
        let curr_piece = self.get_current_piece();
        let hold_piece = &self.hold_piece;

        // Returns iterator for all child states
        let get_iter = |right, hold| {
            let state_transitions = if right {
                right_transitions
            } else {
                left_transitions
            };
            let height = if right { right_height } else { left_height };
            let piece = if hold { hold_piece } else { curr_piece };
            let iter = match state_transitions.get(piece) {
                Some(piece_transitions) => piece_transitions.iter(),
                None => panic!("PieceType {:?} not in transition", piece),
            };
            iter.filter_map(move |(state, _)| {
                // Filter states that would be too high
                let new_state = (state.0 + height, state.1 + height, state.2 + height);
                let transition = if right {
                    GameStateTransition::Right(new_state)
                } else {
                    GameStateTransition::Left(new_state)
                };
                if (new_state.0 < LR_MAX_HEIGHT as i8)
                    && (new_state.1 < LR_MAX_HEIGHT as i8)
                    && (new_state.2 < LR_MAX_HEIGHT as i8)
                {
                    Some(GameStateMoveOptions { hold, transition })
                } else {
                    None
                }
            })
        };

        let left_curr = get_iter(false, false);
        let left_hold = get_iter(false, true);
        let right_curr = get_iter(true, false);
        let right_hold = get_iter(true, true);

        left_curr
            .chain(left_hold)
            .chain(right_curr)
            .chain(right_hold)
    }
    fn get_center_transitions(&self) -> impl Iterator<Item = GameStateMoveOptions> {
        // Given a game state, iterates over all possible child states
        let state_transitions = match C4W_TRANSITIONS.center.get(&self.center) {
            None => panic!("Unknown game center state {}", &self.center),
            Some(p) => p,
        };
        let curr_piece = self.get_current_piece();
        let hold_piece = &self.hold_piece;

        let get_iter = |hold| {
            let piece = if hold { hold_piece } else { curr_piece };
            let iter = match state_transitions.get(piece) {
                Some(piece_transitions) => piece_transitions.iter(),
                None => panic!("PieceType {:?} not in transition", piece),
            };
            iter.map(move |(state, _)| GameStateMoveOptions {
                hold,
                transition: GameStateTransition::Center(*state),
            })
        };

        let curr_iter = get_iter(false);
        let hold_iter = get_iter(true);

        curr_iter.chain(hold_iter)
    }
}
