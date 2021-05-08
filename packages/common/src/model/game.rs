use crate::model::board::Board;
use crate::model::board::BoardUndoInfo;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::piece::Piece;
use crate::model::piece::PieceMoveRes;
use crate::model::piece::PieceType;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

#[derive(Debug)]
pub struct GameDropRes {
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

pub enum GameMoveRes {
    SuccessNorm,
    SuccessDrop(GameDropRes, GameUndoInfo),
    Failed,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    pub fn make_move(&mut self, game_move: &GameMove) -> GameMoveRes {
        match game_move {
            GameMove::ShiftLeft => {
                let res = self.current_piece.shift_left(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::ShiftRight => {
                let res = self.current_piece.shift_right(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::RotateLeft => {
                let res = self.current_piece.rotate_left(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::RotateRight => {
                let res = self.current_piece.rotate_right(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::Rotate180 => {
                let res = self.current_piece.rotate_180(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::Hold => {
                if !self.can_hold {
                    return GameMoveRes::Failed;
                }
                match &mut self.hold_piece {
                    Some(hold) => {
                        std::mem::swap(&mut self.current_piece, hold);
                        // According to SRS, the piece should shift down as soon as it's spawned
                        self.current_piece.reset();
                        self.current_piece.shift_down(&self.board);
                        self.can_hold = false;
                        GameMoveRes::SuccessNorm
                    }
                    None => {
                        if self.queue_pieces.len() == 0 {
                            return GameMoveRes::Failed;
                        }
                        self.hold_piece = Some(self.current_piece.clone());
                        self.current_piece = self.queue_pieces.pop_front().unwrap();
                        // According to SRS, the piece should shift down as soon as it's spawned
                        self.current_piece.reset();
                        self.current_piece.shift_down(&self.board);
                        self.can_hold = false;
                        GameMoveRes::SuccessNorm
                    }
                }
            }
            GameMove::SoftDrop => {
                let res = self.current_piece.soft_drop(&self.board);
                match res {
                    PieceMoveRes::Success => GameMoveRes::SuccessNorm,
                    PieceMoveRes::Failed => GameMoveRes::Failed,
                }
            }
            GameMove::HardDrop => {
                if self.queue_pieces.len() == 0 {
                    return GameMoveRes::Failed;
                }

                self.current_piece.soft_drop(&self.board);
                let (res, undo_info) = self.board.lock(&self.current_piece);
                let undo_piece = self.current_piece.clone();
                self.current_piece = self.queue_pieces.pop_front().unwrap();
                // According to SRS, the piece should shift down as soon as it's spawned
                self.current_piece.reset();
                self.current_piece.shift_down(&self.board);

                let hold = !self.can_hold;
                let hold_empty = self.hold_was_empty;

                self.hold_was_empty = self.hold_piece.is_none();
                self.can_hold = true;
                GameMoveRes::SuccessDrop(
                    GameDropRes {
                        lines_cleared: res.lines_cleared,
                        block_out: res.block_out,
                    },
                    GameUndoInfo {
                        board: undo_info,
                        hold,
                        hold_empty,
                        piece: undo_piece,
                    },
                )
            }
        }
    }

    pub fn undo_move(&mut self, undo: GameUndoInfo) {
        assert_eq!(self.can_hold, false, "Expected can_hold to be true");
        self.board.undo_lock(undo.board);
        self.queue_pieces.push_front(self.current_piece.clone());
        self.current_piece = undo.piece;
        self.can_hold = true;
        if undo.hold {
            if undo.hold_empty {
                self.queue_pieces.push_front(self.current_piece.clone());
                self.current_piece = self.hold_piece.clone().unwrap();
                self.current_piece.reset();
                self.current_piece.shift_down(&self.board);
                self.hold_was_empty = true;
                self.hold_piece = None;
            } else {
                let current_piece = self.current_piece.clone();
                self.current_piece = self.hold_piece.clone().unwrap();
                self.current_piece.reset();
                self.current_piece.shift_down(&self.board);
                self.hold_was_empty = false;
                self.hold_piece = Some(current_piece);
            }
        } else {
            self.current_piece.reset();
            self.current_piece.shift_down(&self.board);
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Board + Current Piece
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

        // Curr, Hold, and Queue pieces
        let curr = &self.current_piece.to_string();
        let hold = match &self.hold_piece {
            Some(piece) => piece.to_string(),
            None => String::from("None"),
        };
        const MAX_QUEUE_DISPLAY: usize = 7;
        let mut queue_text = self
            .queue_pieces
            .iter()
            .take(MAX_QUEUE_DISPLAY)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        if self.queue_pieces.len() > MAX_QUEUE_DISPLAY {
            let amount = self.queue_pieces.len() - MAX_QUEUE_DISPLAY;
            write!(queue_text, " +{}", amount)?;
        }
        writeln!(f, "Curr: {}, Hold: {}, Queue: {}", curr, hold, queue_text)?;

        // Other info
        let can_hold = self.can_hold;
        let hold_was_empty = self.hold_was_empty;
        writeln!(
            f,
            "Can Hold: {}, Hold Was Empty: {}",
            can_hold, hold_was_empty
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::model::piece::Piece;
    use crate::model::piece::PieceType;
    use std::collections::HashSet;

    #[test]
    fn game_hashes_correctly() {
        let mut game1 = Game::new();
        game1.set_current(Piece::new(&PieceType::O));
        game1.set_hold(Some(Piece::new(&PieceType::I)));
        game1.set_queue(vec![Piece::new(&PieceType::T), Piece::new(&PieceType::L)].into_iter());
        game1.board.set(0, 0, true);
        game1.board.set(9, 22, true);
        let mut game2 = Game::new();
        game2.set_current(Piece::new(&PieceType::O));
        game2.set_hold(Some(Piece::new(&PieceType::I)));
        game2.set_queue(vec![Piece::new(&PieceType::T), Piece::new(&PieceType::L)].into_iter());
        game2.board.set(0, 0, true);
        game2.board.set(9, 22, true);
        let mut hashset = HashSet::new();

        // Game 1 and Game 2 should be identical
        assert_eq!(game1, game2);
        hashset.insert(game1.clone());
        hashset.insert(game2.clone());
        assert_eq!(hashset.len(), 1);

        // Differs by current piece
        let mut game3 = Game::new();
        game3.set_current(Piece::new(&PieceType::I));
        game3.set_hold(Some(Piece::new(&PieceType::I)));
        game3.set_queue(vec![Piece::new(&PieceType::T), Piece::new(&PieceType::L)].into_iter());
        game3.board.set(0, 0, true);
        game3.board.set(9, 22, true);
        assert_ne!(game1, game3);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game3.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by hold_piece
        let mut game4 = Game::new();
        game4.set_current(Piece::new(&PieceType::O));
        game4.set_hold(None);
        game4.set_queue(vec![Piece::new(&PieceType::T), Piece::new(&PieceType::L)].into_iter());
        game4.board.set(0, 0, true);
        game4.board.set(9, 22, true);
        assert_ne!(game1, game4);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game4.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by queue
        let mut game5 = Game::new();
        game5.set_current(Piece::new(&PieceType::O));
        game5.set_hold(Some(Piece::new(&PieceType::I)));
        game5.set_queue(vec![Piece::new(&PieceType::L), Piece::new(&PieceType::L)].into_iter());
        game5.board.set(0, 0, true);
        game5.board.set(9, 22, true);
        assert_ne!(game1, game5);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game5.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by board
        let mut game6 = Game::new();
        game6.set_current(Piece::new(&PieceType::O));
        game6.set_hold(Some(Piece::new(&PieceType::I)));
        game6.set_queue(vec![Piece::new(&PieceType::T), Piece::new(&PieceType::L)].into_iter());
        game6.board.set(0, 0, true);
        assert_ne!(game1, game6);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game6.clone());
        assert_eq!(hashset.len(), 2);
    }
}
