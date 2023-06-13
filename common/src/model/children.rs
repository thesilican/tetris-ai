use std::{error::Error, fmt::Display};

use crate::{ActionResult, Game, GameMove, BOARD_WIDTH, PIECE_SHAPE_SIZE};

#[derive(Debug)]
pub enum ChildError {
    /// Cannot generate children when y position of piece
    /// is below the maximum height of the stack
    PieceTooLow,
    /// Cannot generate children when piece is against the wall
    AgainstWall,
}
impl Display for ChildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChildError::PieceTooLow => write!(f, "piece too low"),
            ChildError::AgainstWall => write!(f, "against wall"),
        }
    }
}
impl Error for ChildError {}

pub struct Child {
    pub game: Game,
    hold: bool,
    rot: i8,
    shift: i8,
    fin: i8,
}
impl Child {
    pub fn moves(&self) -> impl Iterator<Item = GameMove> {
        let hold: &[GameMove] = if self.hold { &[GameMove::Hold] } else { &[] };
        let rot: &[GameMove] = match self.rot {
            0 => &[],
            1 => &[GameMove::RotateCW],
            2 => &[GameMove::Rotate180],
            3 => &[GameMove::RotateCCW],
            _ => panic!("rotation out of bounds"),
        };
        let shift = if self.shift < 0 {
            std::iter::repeat(&GameMove::ShiftLeft).take(-self.shift as usize)
        } else {
            std::iter::repeat(&GameMove::ShiftRight).take(self.shift as usize)
        };
        let drop: &[GameMove] = if self.fin == 0 {
            &[]
        } else {
            &[GameMove::SoftDrop]
        };
        let fin: &[GameMove] = match self.fin {
            0 => &[],
            1 => &[GameMove::ShiftLeft],
            2 => &[GameMove::ShiftRight],
            3 => &[GameMove::RotateCW],
            4 => &[GameMove::Rotate180],
            5 => &[GameMove::RotateCCW],
            _ => panic!("fin out of bounds"),
        };
        hold.iter()
            .chain(rot.iter())
            .chain(shift)
            .chain(drop)
            .chain(fin)
            .chain(std::iter::once(&GameMove::HardDrop))
            .copied()
    }
}

impl Game {
    pub fn children(&self) -> Result<Vec<Child>, ChildError> {
        let mut child_states = Vec::<Child>::with_capacity(100);

        // Calculate height map
        let max_height = self.board.max_height();
        let height_map = self.board.height_map();
        let x = self.active.location.0 as i32;
        let y = self.active.location.1 as i32;
        if y < max_height as i32 {
            return Err(ChildError::PieceTooLow);
        }
        if x < 0 || x > (BOARD_WIDTH - PIECE_SHAPE_SIZE) as i32 {
            return Err(ChildError::AgainstWall);
        }

        // Find holes
        let mut holes = [(0, 0); 240];
        let mut holes_len = 0;
        for i in 0..BOARD_WIDTH {
            if height_map[i as usize] == 0 {
                continue;
            }
            for j in 0..(height_map[i as usize] - 1) as usize {
                if !self.board.get(i, j) {
                    holes[holes_len] = (i, j);
                    holes_len += 1;
                }
            }
        }

        // Piece hold
        for hold in [false, true] {
            let mut game = *self;
            if hold {
                if let ActionResult::Fail = game.make_move(GameMove::Hold) {
                    continue;
                }
            }
            if game.queue.len() == 0 {
                continue;
            }

            // Piece rot
            for rot in [0, 1, 2, 3] {
                let mut game = game;
                game.active.rotation = (game.active.rotation + rot) % 4;
                let shift_bounds = game.active.get_location_bounds(None);
                let min_x = shift_bounds.0 as i32;
                let max_x = shift_bounds.1 as i32;

                // Piece shift
                for piece_x in min_x..=max_x {
                    let mut game = game;
                    let shift = piece_x as i8 - game.active.location.0;
                    game.active.location.0 = piece_x as i8;

                    // Soft drop
                    let x = game.active.location.0 as i32;
                    let piece_map = game.active.get_height_map(None);
                    let mut min_y = i32::MIN;
                    for i in 0..PIECE_SHAPE_SIZE {
                        if piece_map[i].0 == -1 {
                            continue;
                        }
                        let col = x + (i as i32);
                        if col < 0 || col > BOARD_WIDTH as i32 {
                            continue;
                        }
                        let best_y = (height_map[col as usize] as i32) - (piece_map[i].0 as i32);
                        min_y = min_y.max(best_y);
                    }
                    game.active.location.1 = min_y as i8;

                    // Final move
                    // 0 - nothing
                    // 1 - shift left
                    // 2 - shift right
                    // 3 - rot cw
                    // 4 - rot 180
                    // 5 - rot ccw
                    for fin in 0..6 {
                        let mut game = game;
                        if holes_len == 0 && fin != 0 {
                            continue;
                        }
                        let success = match fin {
                            0 => true,
                            1 => game.active.shift_left(&game.board),
                            2 => game.active.shift_right(&game.board),
                            3 => game.active.rotate_cw(&game.board),
                            4 => game.active.rotate_180(&game.board),
                            5 => game.active.rotate_ccw(&game.board),
                            _ => unreachable!(),
                        };
                        if !success {
                            continue;
                        }
                        if fin != 0 {
                            game.active.soft_drop(&game.board);
                        }
                        let mut found = false;
                        for i in 0..holes_len {
                            let (x, y) = holes[i];
                            let px = (x as i8) - game.active.location.0;
                            let py = (y as i8) - game.active.location.1;
                            if px < 0 || px >= 4 || py < 0 || py >= 4 {
                                continue;
                            }
                            let shape = game.active.get_shape(None);
                            let bit = shape[px as usize][py as usize];
                            if bit {
                                found = true;
                                break;
                            }
                        }
                        if !found && holes_len > 0 && fin != 0 {
                            continue;
                        }

                        game.board.lock(&game.active);
                        game.active.piece_type = game.queue.pop_front().unwrap();
                        game.active.reset();
                        game.can_hold = true;
                        let child_state = Child {
                            game,
                            hold,
                            rot,
                            shift,
                            fin,
                        };
                        child_states.push(child_state);
                    }
                }
            }
        }
        Ok(child_states)
    }
}
