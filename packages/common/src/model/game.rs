use crate::misc::GenericErr;
use crate::model::board::Board;
use crate::model::board::BoardUndoInfo;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::piece::Piece;
use crate::model::piece::PieceType;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct GameDropResult {
    pub lines_cleared: i32,
    pub block_out: bool,
}

#[derive(Debug)]
pub struct GameUndoInfo {
    pub board: BoardUndoInfo,
    pub piece: Piece,
    pub hold: bool,
    pub hold_empty: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMove {
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Rotate180,
    Hold,
    SoftDrop,
    HardDrop,
}
impl GameMove {
    fn to_string(&self) -> String {
        let slice = match self {
            GameMove::ShiftLeft => "shiftLeft",
            GameMove::ShiftRight => "shiftRight",
            GameMove::RotateLeft => "rotateLeft",
            GameMove::RotateRight => "rotateRight",
            GameMove::Rotate180 => "rotate180",
            GameMove::Hold => "hold",
            GameMove::SoftDrop => "softDrop",
            GameMove::HardDrop => "hardDrop",
        };
        String::from(slice)
    }
}
impl Display for GameMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub current_piece: Piece,
    pub hold_piece: Option<Piece>,
    pub queue_pieces: VecDeque<Piece>,
    pub can_hold: bool,
    pub hold_was_empty: bool,
}
impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::new(&PieceType::O),
            hold_piece: None,
            queue_pieces: VecDeque::new(),
            can_hold: true,
            hold_was_empty: true,
        }
    }

    pub fn set_current(&mut self, piece: Piece) {
        self.current_piece = piece;
        self.can_hold = true;
        self.hold_was_empty = self.hold_piece.is_none();
    }
    pub fn set_hold(&mut self, piece: Option<Piece>) {
        self.hold_piece = piece;
        self.can_hold = true;
        self.hold_was_empty = self.hold_piece.is_none();
    }
    pub fn append_queue(&mut self, piece: Piece) {
        self.queue_pieces.push_back(piece);
    }
    pub fn extend_queue(&mut self, pieces: impl Iterator<Item = Piece>) {
        self.queue_pieces.extend(pieces);
    }
    pub fn clear_queue(&mut self) {
        self.queue_pieces.clear();
    }
    pub fn set_queue(&mut self, pieces: impl Iterator<Item = Piece>) {
        self.clear_queue();
        self.extend_queue(pieces);
    }

    pub fn make_move(
        &mut self,
        game_move: &GameMove,
    ) -> Result<Option<(GameDropResult, GameUndoInfo)>, GenericErr> {
        match game_move {
            GameMove::ShiftLeft => {
                let res = self.current_piece.shift_left(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not shift left".into()),
                }
            }
            GameMove::ShiftRight => {
                let res = self.current_piece.shift_right(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not shift right".into()),
                }
            }
            GameMove::RotateLeft => {
                let res = self.current_piece.rotate_left(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not rotate left".into()),
                }
            }
            GameMove::RotateRight => {
                let res = self.current_piece.rotate_right(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not rotate right".into()),
                }
            }
            GameMove::Rotate180 => {
                let res = self.current_piece.rotate_180(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not rotate 180".into()),
                }
            }
            GameMove::Hold => {
                if !self.can_hold {
                    return Err("Could not hold".into());
                }
                match &self.hold_piece {
                    Some(hold) => {
                        let curr = self.current_piece.clone();
                        self.current_piece = hold.clone();
                        self.current_piece.reset();
                        self.hold_piece = Some(curr);
                        self.can_hold = false;
                        Ok(None)
                    }
                    None => {
                        if self.queue_pieces.len() == 0 {
                            return Err("Could not hold".into());
                        }
                        self.hold_piece = Some(self.current_piece.clone());
                        self.current_piece = self.queue_pieces.pop_front().unwrap();
                        self.current_piece.reset();
                        self.can_hold = false;
                        Ok(None)
                    }
                }
            }
            GameMove::SoftDrop => {
                let res = self.current_piece.soft_drop(&self.board);
                match res {
                    true => Ok(None),
                    false => Err("Could not soft drop".into()),
                }
            }
            GameMove::HardDrop => {
                if self.queue_pieces.len() == 0 {
                    return Err("Could not hard drop".into());
                }

                self.current_piece.soft_drop(&self.board);
                let (res, undo_info) = self.board.lock(&self.current_piece);
                let undo_piece = self.current_piece.clone();
                self.current_piece = self.queue_pieces.pop_front().unwrap();
                self.current_piece.reset();

                let hold = !self.can_hold;
                let hold_empty = self.hold_was_empty;

                self.hold_was_empty = self.hold_piece.is_none();
                self.can_hold = true;
                Ok(Some((
                    GameDropResult {
                        lines_cleared: res.lines_cleared,
                        block_out: res.block_out,
                    },
                    GameUndoInfo {
                        board: undo_info,
                        hold,
                        hold_empty,
                        piece: undo_piece,
                    },
                )))
            }
        }
    }

    pub fn undo_move(&mut self, undo: GameUndoInfo) {
        self.board.undo_lock(undo.board);
        self.queue_pieces.push_front(self.current_piece.clone());
        self.current_piece = undo.piece;
        if undo.hold {
            if undo.hold_empty {
                self.queue_pieces.push_front(self.current_piece.clone());
                self.current_piece = self.hold_piece.clone().unwrap();
                self.current_piece.reset();
                self.hold_was_empty = true;
                self.hold_piece = None;
            } else {
                let current_piece = self.current_piece.clone();
                self.current_piece = self.hold_piece.clone().unwrap();
                self.current_piece.reset();
                self.hold_was_empty = false;
                self.hold_piece = Some(current_piece);
            }
        } else {
            self.current_piece.reset();
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let piece = &self.current_piece;
        let piece_shape = piece.get_shape(None);
        let (p_x, p_y) = piece.location;
        for j in (0..BOARD_HEIGHT).rev() {
            for i in 0..BOARD_WIDTH {
                let in_piece_bounds = i - p_x >= 0
                    && i - p_x < PIECE_SHAPE_SIZE
                    && j - p_y >= 0
                    && j - p_y < PIECE_SHAPE_SIZE;
                let in_piece =
                    in_piece_bounds && piece_shape[(i - p_x) as usize][(j - p_y) as usize];

                if in_piece {
                    write!(f, "██")?;
                } else if self.board.get(i, j) {
                    write!(f, "▓▓")?;
                } else if in_piece_bounds {
                    write!(f, "▒▒")?;
                } else {
                    write!(f, "░░")?;
                }
            }
            writeln!(f)?;
        }
        // Board height/holes info
        for i in 0..BOARD_WIDTH {
            let height = self.board.height_map[i as usize];
            write!(f, "{:2}", height)?;
        }
        writeln!(f)?;
        for i in 0..BOARD_WIDTH {
            let hole = self.board.holes[i as usize];
            write!(f, "{:2}", hole)?;
        }
        writeln!(f)?;
        // Other info
        let curr = &self.current_piece.to_string();
        let hold = match &self.hold_piece {
            Some(piece) => piece.to_string(),
            None => String::from("null"),
        };
        let mut queue_text = String::new();
        for piece in &self.queue_pieces {
            queue_text.push_str(&piece.to_string());
            queue_text.push(' ');
        }
        writeln!(f, "Curr: {} Hold: {} Queue: {}", curr, hold, queue_text)?;
        Ok(())
    }
}
