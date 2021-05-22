use crate::model::board::Board;
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
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct GameDropRes {
    pub lines_cleared: i32,
    pub top_out: bool,
}

pub enum GameMoveRes {
    SuccessNorm,
    SuccessDrop(GameDropRes),
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

#[derive(Clone, Debug, Eq)]
pub struct Game {
    pub board: Board,
    pub current_piece: Piece,
    pub hold_piece: Option<PieceType>,
    pub queue_pieces: VecDeque<PieceType>,
    pub can_hold: bool,
}
impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::new(&PieceType::O),
            hold_piece: None,
            queue_pieces: VecDeque::new(),
            can_hold: true,
        }
    }

    pub fn set_current(&mut self, piece: PieceType) {
        self.current_piece.piece_type = piece;
        self.reset_current_piece();
        self.can_hold = true;
    }
    pub fn set_hold(&mut self, piece: Option<PieceType>) {
        self.hold_piece = piece;
        self.can_hold = true;
    }
    pub fn append_queue(&mut self, piece: PieceType) {
        self.queue_pieces.push_back(piece);
    }
    pub fn extend_queue(&mut self, pieces: &[PieceType]) {
        self.queue_pieces.extend(pieces);
    }
    pub fn clear_queue(&mut self) {
        self.queue_pieces.clear();
    }
    pub fn set_queue(&mut self, pieces: &[PieceType]) {
        self.clear_queue();
        self.extend_queue(pieces);
    }
    fn reset_current_piece(&mut self) {
        self.current_piece.reset();
        self.current_piece.shift_down(&self.board);
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
                let hold = match self.hold_piece {
                    Some(hold) => hold,
                    None => match self.queue_pieces.pop_front() {
                        Some(piece) => piece,
                        None => return GameMoveRes::Failed,
                    },
                };
                self.hold_piece = Some(self.current_piece.piece_type);
                self.current_piece.piece_type = hold;
                self.reset_current_piece();
                GameMoveRes::SuccessNorm
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
                let res = self.board.lock(&self.current_piece);
                self.current_piece.piece_type = self.queue_pieces.pop_front().unwrap();
                self.reset_current_piece();
                self.current_piece.shift_down(&self.board);
                self.can_hold = true;

                GameMoveRes::SuccessDrop(GameDropRes {
                    lines_cleared: res.lines_cleared,
                    top_out: res.top_out,
                })
            }
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Board + Current Piece
        let piece = &self.current_piece;
        let piece_shape = piece.get_shape(None);
        let (p_x, p_y) = piece.location;
        let (p_x, p_y) = (p_x as i32, p_y as i32);
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
        // for i in 0..BOARD_WIDTH {
        //     let height = self.board.height_map[i as usize];
        //     write!(f, "{:2}", height)?;
        // }
        // writeln!(f)?;
        // for i in 0..BOARD_WIDTH {
        //     let hole = self.board.holes[i as usize];
        //     write!(f, "{:2}", hole)?;
        // }
        // writeln!(f)?;

        // Curr, Hold, and Queue pieces
        let curr = format!("{}", &self.current_piece);
        let hold = match &self.hold_piece {
            Some(piece) => {
                let can_hold = if self.can_hold { "✓" } else { "✗" };
                format!("{0} {1}", piece, can_hold)
            }
            None => format!(""),
        };
        const MAX_QUEUE_DISPLAY: usize = 7;
        let queue_text = {
            let mut text = self
                .queue_pieces
                .iter()
                .take(MAX_QUEUE_DISPLAY)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if self.queue_pieces.len() > MAX_QUEUE_DISPLAY {
                let amount = self.queue_pieces.len() - MAX_QUEUE_DISPLAY;
                write!(text, " +{}", amount)?;
            }
            text
        };
        write!(f, "[{1}] ({0}) {2}", curr, hold, queue_text)?;

        Ok(())
    }
}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.current_piece == other.current_piece
            && self.hold_piece == other.hold_piece
            && self.queue_pieces == other.queue_pieces
    }
}
impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.current_piece.hash(state);
        self.hold_piece.hash(state);
        self.queue_pieces.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::model::piece::PieceType;
    use std::collections::HashSet;

    #[test]
    fn game_hashes_properly() {
        let mut game1 = Game::new();
        game1.set_current(PieceType::O);
        game1.set_hold(Some(PieceType::I));
        game1.set_queue(&[PieceType::T, PieceType::L]);
        game1.board.set(0, 0, true);
        game1.board.set(9, 22, true);
        let mut game2 = Game::new();
        game2.set_current(PieceType::O);
        game2.set_hold(Some(PieceType::I));
        game2.set_queue(&[PieceType::T, PieceType::L]);
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
        game3.set_current(PieceType::I);
        game3.set_hold(Some(PieceType::I));
        game3.set_queue(&[PieceType::T, PieceType::L]);
        game3.board.set(0, 0, true);
        game3.board.set(9, 22, true);
        assert_ne!(game1, game3);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game3.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by hold_piece
        let mut game4 = Game::new();
        game4.set_current(PieceType::O);
        game4.set_hold(None);
        game4.set_queue(&[PieceType::T, PieceType::L]);
        game4.board.set(0, 0, true);
        game4.board.set(9, 22, true);
        assert_ne!(game1, game4);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game4.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by queue
        let mut game5 = Game::new();
        game5.set_current(PieceType::O);
        game5.set_hold(Some(PieceType::I));
        game5.set_queue(&[PieceType::L, PieceType::L]);
        game5.board.set(0, 0, true);
        game5.board.set(9, 22, true);
        assert_ne!(game1, game5);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game5.clone());
        assert_eq!(hashset.len(), 2);

        // Differs by board
        let mut game6 = Game::new();
        game6.set_current(PieceType::O);
        game6.set_hold(Some(PieceType::I));
        game6.set_queue(&[PieceType::T, PieceType::L]);
        game6.board.set(0, 0, true);
        assert_ne!(game1, game6);
        hashset.clear();
        hashset.insert(game1.clone());
        hashset.insert(game6.clone());
        assert_eq!(hashset.len(), 2);
    }
}
