use super::game::{Game, GameMove};
use crate::model::{piece::Piece, Board};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Hold: true/false
// Rot: 0..4
// Shift: -4..=5
// Shift_1: -2..=2
// Rot_1: 0..4
// Rot_2: 0..4
#[derive(Debug, Clone, Copy)]
struct ChildTransitionKey {
    pub hold: bool,
    pub rot: i8,
    pub shift: i8,
    pub shift_1: i8,
    pub rot_1: i8,
    pub rot_2: i8,
}
struct ChildTransitions {
    all_transitions: [[[[[[Vec<GameMove>; 4]; 4]; 5]; 10]; 4]; 2],
}
impl ChildTransitions {
    pub fn new() -> Self {
        fn shift_to_vec(shift: i32) -> Vec<GameMove> {
            let mut res = Vec::new();
            for _ in 0..shift.abs() {
                if shift < 0 {
                    res.push(GameMove::ShiftLeft);
                } else {
                    res.push(GameMove::ShiftRight);
                }
            }
            return res;
        }
        fn rot_to_vec(rot: usize) -> Vec<GameMove> {
            match rot {
                0 => vec![],
                1 => vec![GameMove::RotateRight],
                2 => vec![GameMove::Rotate180],
                3 => vec![GameMove::RotateLeft],
                _ => unreachable!(),
            }
        }

        let mut transitions: [[[[[[Vec<GameMove>; 4]; 4]; 5]; 10]; 4]; 2] = Default::default();
        for hold in 0..2 {
            for rot in 0..4 {
                for shift in -4i32..=5 {
                    for shift_1 in -2i32..=2 {
                        for rot_1 in 0..4 {
                            for rot_2 in 0..4 {
                                let vec = &mut transitions[hold][rot][(shift + 4) as usize]
                                    [(shift_1 + 2) as usize][rot_1][rot_2];
                                if hold == 1 {
                                    vec.push(GameMove::Hold);
                                }
                                vec.extend(rot_to_vec(rot));
                                vec.extend(shift_to_vec(shift));
                                if shift_1 != 0 || rot_1 != 0 || rot_2 != 0 {
                                    vec.push(GameMove::SoftDrop);
                                }
                                vec.extend(shift_to_vec(shift_1));
                                vec.extend(rot_to_vec(rot_1));
                                vec.extend(rot_to_vec(rot_2));
                                vec.push(GameMove::HardDrop);
                            }
                        }
                    }
                }
            }
        }
        ChildTransitions {
            all_transitions: transitions,
        }
    }

    pub fn get(&self, key: ChildTransitionKey) -> &[GameMove] {
        let hold = if key.hold { 1 } else { 0 };
        let rot = key.rot as usize;
        let shift = (key.shift + 4) as usize;
        let shift_1 = (key.shift_1 + 2) as usize;
        let rot_1 = key.rot_1 as usize;
        let rot_2 = key.rot_2 as usize;
        &self.all_transitions[hold][rot][shift][shift_1][rot_1][rot_2]
    }
}

lazy_static! {
    static ref CHILD_TRANSITIONS: ChildTransitions = ChildTransitions::new();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildStatesShift {
    None,
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildStatesRot {
    None,
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildStatesOptions {
    final_shift: ChildStatesShift,
    final_rot: ChildStatesRot,
}

pub const NSNR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::None,
    final_rot: ChildStatesRot::None,
};
pub const NSSR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::None,
    final_rot: ChildStatesRot::Single,
};
pub const NSDR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::None,
    final_rot: ChildStatesRot::Double,
};
pub const SSNR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Single,
    final_rot: ChildStatesRot::None,
};
pub const SSSR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Single,
    final_rot: ChildStatesRot::Single,
};
pub const SSDR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Single,
    final_rot: ChildStatesRot::Double,
};
pub const DSNR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Double,
    final_rot: ChildStatesRot::None,
};
pub const DSSR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Double,
    final_rot: ChildStatesRot::Single,
};
pub const DSDR: ChildStatesOptions = ChildStatesOptions {
    final_shift: ChildStatesShift::Double,
    final_rot: ChildStatesRot::Double,
};

type ChildState = (Game, &'static [GameMove]);
type PieceKey = ([u16; 4], i8);

impl Game {
    pub fn child_states(&self, options: ChildStatesOptions) -> Vec<ChildState> {
        assert!(
            self.queue_pieces.len() >= 1
                || (self.hold_piece.is_some() && self.queue_pieces.len() >= 2)
        );

        // Utility functions
        fn roted_piece(mut piece: Piece, board: &Board, rot: i8) -> Piece {
            match rot {
                0 => {}
                1 => {
                    piece.rotate_right(board);
                }
                2 => {
                    piece.rotate_180(board);
                }
                3 => {
                    piece.rotate_left(board);
                }
                _ => unreachable!(),
            }
            piece
        }
        fn shifted_piece(mut piece: Piece, board: &Board, shift: i8) -> Piece {
            for _ in 0..shift.abs() {
                if shift < 0 {
                    piece.shift_left(&board);
                } else {
                    piece.shift_right(&board);
                }
            }
            piece
        }
        fn get_key(piece: Piece) -> PieceKey {
            let mut y = piece.location.1;
            let mut shape = *piece.get_bit_shape(None, None);
            while shape[0] == 0 {
                for i in 0..3 {
                    shape[i] = shape[i + 1];
                }
                shape[3] = 0;
                y += 1;
            }
            (shape, y)
        }

        // Get current and hold pieces
        let board = &self.board;
        let current_piece = self.current_piece;
        let hold_piece = {
            let mut hold = Piece::from(self.hold_piece.unwrap_or(self.queue_pieces[0]));
            hold.reset(&self.board);
            hold
        };

        // Return value
        let mut res = Vec::<ChildState>::new();
        // Map from piece key to corresponding entry in res vec
        let mut visited = HashMap::<PieceKey, usize>::new();

        // Big tower of loops
        let shift_1_range: &[i8] = match options.final_shift {
            ChildStatesShift::None => &[0],
            ChildStatesShift::Single => &[0, -1, 1],
            ChildStatesShift::Double => &[0, -2, -1, 1, 2],
        };
        let (rot_1_end, rot_2_end) = match options.final_rot {
            ChildStatesRot::None => (1, 1),
            ChildStatesRot::Single => (4, 1),
            ChildStatesRot::Double => (4, 4),
        };
        // Shift_1, Rotation_1, Rotation_2 (outside so that it's last)
        for shift_1 in shift_1_range {
            let shift_1 = *shift_1;
            for rot_1 in 0..rot_1_end {
                for rot_2 in 0..rot_2_end {
                    if rot_1 == 0 && rot_2 != 0 {
                        continue;
                    }
                    // Hold
                    for hold in [false, true] {
                        let piece = if hold {
                            hold_piece.clone()
                        } else {
                            current_piece.clone()
                        };
                        // Rotation
                        for rot in 0..4 {
                            let piece = roted_piece(piece, board, rot);
                            let (left, right) = *piece.get_shift_bounds(None);
                            // Shift
                            for shift in -left..=right {
                                let mut piece = shifted_piece(piece, board, shift);
                                if shift_1 != 0 || rot_1 != 0 || rot_2 != 0 {
                                    piece.soft_drop(board);
                                }
                                piece = shifted_piece(piece, board, shift_1);
                                piece = roted_piece(piece, board, rot_1);
                                piece = roted_piece(piece, board, rot_2);
                                piece.soft_drop(board);

                                // Get key and permutation
                                let key = get_key(piece);
                                let perm = CHILD_TRANSITIONS.get(ChildTransitionKey {
                                    hold,
                                    rot,
                                    shift,
                                    shift_1,
                                    rot_1,
                                    rot_2,
                                });
                                // Add or maybe update list
                                match visited.get(&key) {
                                    Some(index) => {
                                        let old_perm = &mut res[*index].1;
                                        if perm.len() < old_perm.len() {
                                            *old_perm = perm;
                                        }
                                    }
                                    None => {
                                        // Get child
                                        let mut game = self.clone();
                                        for game_move in perm {
                                            game.make_move(*game_move);
                                        }
                                        if !game.board.topped_out() {
                                            visited.insert(key, res.len());
                                            res.push((game, perm))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }
}
