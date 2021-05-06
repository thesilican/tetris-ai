use crate::ai::computed::C4WTransitionInfo;
use crate::ai::computed::C4W_TRANSITION_INFO;
use crate::ai::consts::LEFT_COL;
use crate::ai::consts::LR_MAX_HEIGHT;
use crate::ai::consts::LR_WIDTH;
use crate::ai::consts::RIGHT_COL;
use crate::model::game::Game;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;

pub enum C4WConstructErr {
    HoldIsNone,
    InvalidBoard,
}
#[derive(Debug)]
pub struct C4WMoveErr;
#[derive(Debug)]
pub enum C4WStateChange {
    Left((i8, i8, i8)),
    Center(u16),
    Right((i8, i8, i8)),
}
impl C4WStateChange {
    pub fn is_downstack(&self) -> bool {
        matches!(self, C4WStateChange::Center(_))
    }
    pub fn is_upstack(&self) -> bool {
        !self.is_downstack()
    }
}
pub struct C4WMoveOptions {
    pub change: C4WStateChange,
    pub hold: bool,
}
pub struct C4WUndoInfo {
    change: C4WStateChange,
    hold: bool,
}

pub struct C4WState {
    pub left: (i8, i8, i8),
    pub center: u16,
    pub right: (i8, i8, i8),
    pub queue_ptr: usize,
    pub queue_pieces: Vec<PieceType>,
    pub hold_piece: PieceType,
}
impl C4WState {
    pub fn from_game(game: &Game) -> Result<Self, C4WConstructErr> {
        // Hold/Queue
        let hold_piece = match &game.hold_piece {
            Some(piece) => piece.piece_type.clone(),
            None => return Err(C4WConstructErr::HoldIsNone),
        };
        let mut queue_pieces = Vec::new();
        queue_pieces.push(game.current_piece.piece_type.clone());
        queue_pieces.extend(game.queue_pieces.iter().map(|x| x.piece_type.clone()));

        // States
        let center = C4WTransitionInfo::center_get_state(&game.board);
        if !C4W_TRANSITION_INFO.center.contains_key(&center) {
            return Err(C4WConstructErr::InvalidBoard);
        }
        for i in 0..LR_WIDTH {
            let col = i + LEFT_COL;
            if game.board.holes[col as usize] != 0 {
                return Err(C4WConstructErr::InvalidBoard);
            }
        }
        let left = C4WTransitionInfo::lr_get_state(&game.board, true);
        for i in 0..LR_WIDTH {
            let col = i + RIGHT_COL;
            if game.board.holes[col as usize] != 0 {
                return Err(C4WConstructErr::InvalidBoard);
            }
        }
        let right = C4WTransitionInfo::lr_get_state(&game.board, false);

        Ok(C4WState {
            hold_piece,
            queue_pieces,
            queue_ptr: 0,
            left,
            center,
            right,
        })
    }
    pub fn apply_to_game(&self, game: &mut Game) {
        let mut queue_iter = self.queue_pieces.iter();
        let current = Piece::new(queue_iter.next().unwrap().clone());
        let queue = queue_iter.map(|x| Piece::new(x.clone())).collect();
        let hold = Some(Piece::new(self.hold_piece.clone()));
        game.set_current(current);
        game.set_queue(queue);
        game.set_hold(hold);

        C4WTransitionInfo::lr_set_state(&mut game.board, self.left, true);
        C4WTransitionInfo::lr_set_state(&mut game.board, self.right, false);
        C4WTransitionInfo::center_set_state(&mut game.board, self.center);
    }

    pub fn make_move(&mut self, c4w_move: &C4WMoveOptions) -> Result<C4WUndoInfo, C4WMoveErr> {
        if self.queue_ptr >= self.queue_pieces.len() - 1 {
            return Err(C4WMoveErr);
        }
        let undo_hold = c4w_move.hold;
        if c4w_move.hold {
            self.swap_hold();
        }
        self.queue_ptr += 1;
        // Replace state
        let old_state = match c4w_move.change {
            C4WStateChange::Left(new_state) => {
                let old_state = C4WStateChange::Left(self.left);
                self.left = new_state;
                old_state
            }
            C4WStateChange::Center(new_state) => {
                let old_state = C4WStateChange::Center(self.center);
                self.center = new_state;
                old_state
            }
            C4WStateChange::Right(new_state) => {
                let old_state = C4WStateChange::Right(self.right);
                self.right = new_state;
                old_state
            }
        };
        Ok(C4WUndoInfo {
            hold: undo_hold,
            change: old_state,
        })
    }
    pub fn undo_move(&mut self, undo_info: &C4WUndoInfo) {
        match undo_info.change {
            C4WStateChange::Left(new_state) => self.left = new_state,
            C4WStateChange::Center(new_state) => self.center = new_state,
            C4WStateChange::Right(new_state) => self.right = new_state,
        }
        self.queue_ptr -= 1;
        if undo_info.hold {
            self.swap_hold();
        }
    }

    pub fn get_moves(&self) -> impl Iterator<Item = C4WMoveOptions> {
        let current_piece = &self.queue_pieces[self.queue_ptr];
        let hold_piece = &self.hold_piece;

        let center_iter = {
            let transitions = C4W_TRANSITION_INFO
                .center
                .get(&self.center)
                .expect("Unknown center state");
            let current_iter = transitions
                .get(current_piece)
                .expect("Expected piece")
                .iter()
                .map(|(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Center(*state),
                    hold: false,
                });
            let hold_iter = transitions
                .get(hold_piece)
                .expect("Expected piece")
                .iter()
                .map(|(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Center(*state),
                    hold: true,
                });
            current_iter.chain(hold_iter)
        };
        let left_iter = {
            let (state, height) = trim_lr(self.left);
            let transitions = C4W_TRANSITION_INFO
                .left
                .get(&state)
                .expect("Unknown left state");
            let current_iter = transitions
                .get(current_piece)
                .expect("Expected piece")
                .iter()
                .filter(move |(state, _)| is_lr_valid(**state, height))
                .map(move |(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Left(inflate_lr(*state, height)),
                    hold: false,
                });
            let hold_iter = transitions
                .get(hold_piece)
                .expect("Expected piece")
                .iter()
                .filter(move |(state, _)| is_lr_valid(**state, height))
                .map(move |(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Left(inflate_lr(*state, height)),
                    hold: true,
                });
            current_iter.chain(hold_iter)
        };
        let right_iter = {
            let (state, height) = trim_lr(self.right);
            let transitions = C4W_TRANSITION_INFO
                .right
                .get(&state)
                .expect("Unknown right state");
            let current_iter = transitions
                .get(current_piece)
                .expect("Expected piece")
                .iter()
                .filter(move |(state, _)| is_lr_valid(**state, height))
                .map(move |(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Right(inflate_lr(*state, height)),
                    hold: false,
                });
            let hold_iter = transitions
                .get(hold_piece)
                .expect("Expected piece")
                .iter()
                .filter(move |(state, _)| is_lr_valid(**state, height))
                .map(move |(state, _)| C4WMoveOptions {
                    change: C4WStateChange::Right(inflate_lr(*state, height)),
                    hold: true,
                });
            current_iter.chain(hold_iter)
        };
        center_iter.chain(left_iter).chain(right_iter)
    }

    fn swap_hold(&mut self) {
        std::mem::swap(&mut self.hold_piece, &mut self.queue_pieces[self.queue_ptr]);
    }
    pub fn get_current_piece(&self) -> &PieceType {
        &self.queue_pieces[self.queue_ptr]
    }
    pub fn get_hold_piece(&self) -> &PieceType {
        &self.hold_piece
    }
}

pub fn trim_lr(state: (i8, i8, i8)) -> ((i8, i8, i8), i8) {
    use std::cmp::min;
    let height = min(state.0, min(state.1, state.2));
    return (
        (state.0 - height, state.1 - height, state.2 - height),
        height,
    );
}
pub fn inflate_lr(state: (i8, i8, i8), height: i8) -> (i8, i8, i8) {
    (state.0 + height, state.1 + height, state.2 + height)
}
pub fn is_lr_valid(state: (i8, i8, i8), height: i8) -> bool {
    state.0 + height < LR_MAX_HEIGHT as i8
        && state.1 + height < LR_MAX_HEIGHT as i8
        && state.2 + height < LR_MAX_HEIGHT as i8
}
