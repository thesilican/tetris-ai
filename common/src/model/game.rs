use crate::generic_err;
use crate::misc::ArrDeque;
use crate::model::board::Board;
use crate::model::piece::Piece;
use crate::model::piece::PieceAction;
use crate::model::piece::PieceType;
use crate::model::BAG_LEN;
use crate::model::{Bag, Stream, GAME_MAX_QUEUE_LEN};
use crate::GenericResult;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SwapHoldRes {
    Success,
    Fail,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameLockInfo {
    pub lines_cleared: i8,
    pub top_out: bool,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameActionRes {
    Success,
    SuccessDrop(GameLockInfo),
    Fail,
}
impl From<GameActionRes> for GameMoveRes {
    fn from(val: GameActionRes) -> Self {
        match val {
            GameActionRes::Success => GameMoveRes::Success,
            GameActionRes::SuccessDrop(x) => GameMoveRes::SuccessDrop(x),
            GameActionRes::Fail => GameMoveRes::Fail,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameMoveRes {
    Success,
    SuccessDrop(GameLockInfo),
    Fail,
}
impl From<GameMoveRes> for GameActionRes {
    fn from(val: GameMoveRes) -> Self {
        match val {
            GameMoveRes::Success => GameActionRes::Success,
            GameMoveRes::SuccessDrop(x) => GameActionRes::SuccessDrop(x),
            GameMoveRes::Fail => GameActionRes::Fail,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameMove {
    ShiftLeft,
    ShiftRight,
    RotateCW,
    Rotate180,
    RotateCCW,
    SoftDrop,
    Hold,
    HardDrop,
}
impl GameMove {
    pub fn from_u8(val: u8) -> GenericResult<Self> {
        match val {
            0 => Ok(GameMove::ShiftLeft),
            1 => Ok(GameMove::ShiftRight),
            2 => Ok(GameMove::RotateCW),
            3 => Ok(GameMove::Rotate180),
            4 => Ok(GameMove::RotateCCW),
            5 => Ok(GameMove::Hold),
            6 => Ok(GameMove::SoftDrop),
            7 => Ok(GameMove::HardDrop),
            _ => generic_err!("unknown u8 value for GameMove"),
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
            GameMove::ShiftLeft => 0,
            GameMove::ShiftRight => 1,
            GameMove::RotateCW => 2,
            GameMove::Rotate180 => 3,
            GameMove::RotateCCW => 4,
            GameMove::SoftDrop => 5,
            GameMove::Hold => 6,
            GameMove::HardDrop => 7,
        }
    }
}
impl Display for GameMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMove::ShiftLeft => write!(f, "shiftLeft"),
            GameMove::ShiftRight => write!(f, "shiftRight"),
            GameMove::RotateCW => write!(f, "rotateCW"),
            GameMove::Rotate180 => write!(f, "rotate180"),
            GameMove::RotateCCW => write!(f, "rotateCCW"),
            GameMove::Hold => write!(f, "hold"),
            GameMove::SoftDrop => write!(f, "softDrop"),
            GameMove::HardDrop => write!(f, "hardDrop"),
        }
    }
}
impl TryFrom<GameAction> for GameMove {
    type Error = ();

    fn try_from(value: GameAction) -> Result<Self, Self::Error> {
        match value {
            GameAction::ShiftLeft => Ok(GameMove::ShiftLeft),
            GameAction::ShiftRight => Ok(GameMove::ShiftRight),
            GameAction::ShiftDown => Err(()),
            GameAction::RotateCW => Ok(GameMove::RotateCW),
            GameAction::Rotate180 => Ok(GameMove::Rotate180),
            GameAction::RotateCCW => Ok(GameMove::RotateCCW),
            GameAction::SoftDrop => Ok(GameMove::SoftDrop),
            GameAction::Hold => Ok(GameMove::Hold),
            GameAction::Lock => Err(()),
            GameAction::AddGarbage { .. } => Err(()),
        }
    }
}
impl Default for GameMove {
    fn default() -> Self {
        Self::from_u8(0).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameAction {
    ShiftLeft,
    ShiftRight,
    ShiftDown,
    RotateCW,
    Rotate180,
    RotateCCW,
    SoftDrop,
    Hold,
    Lock,
    AddGarbage { col: usize, height: i8 },
}
impl TryFrom<GameMove> for GameAction {
    type Error = ();

    fn try_from(value: GameMove) -> Result<Self, Self::Error> {
        match value {
            GameMove::ShiftLeft => Ok(GameAction::ShiftLeft),
            GameMove::ShiftRight => Ok(GameAction::ShiftRight),
            GameMove::RotateCW => Ok(GameAction::RotateCW),
            GameMove::Rotate180 => Ok(GameAction::Rotate180),
            GameMove::RotateCCW => Ok(GameAction::RotateCCW),
            GameMove::SoftDrop => Ok(GameAction::SoftDrop),
            GameMove::Hold => Ok(GameAction::Hold),
            GameMove::HardDrop => Err(()),
        }
    }
}
impl From<PieceAction> for GameAction {
    fn from(value: PieceAction) -> Self {
        match value {
            PieceAction::ShiftLeft => GameAction::ShiftLeft,
            PieceAction::ShiftRight => GameAction::ShiftRight,
            PieceAction::ShiftDown => GameAction::ShiftDown,
            PieceAction::RotateCW => GameAction::RotateCW,
            PieceAction::Rotate180 => GameAction::Rotate180,
            PieceAction::RotateCCW => GameAction::RotateCCW,
            PieceAction::SoftDrop => GameAction::SoftDrop,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    #[serde(rename = "current")]
    pub current_piece: Piece,
    #[serde(rename = "hold")]
    pub hold_piece: Option<PieceType>,
    #[serde(rename = "queue")]
    pub queue_pieces: ArrDeque<PieceType, GAME_MAX_QUEUE_LEN>,
    #[serde(rename = "canHold")]
    pub can_hold: bool,
}
impl Game {
    pub fn from_parts(
        board: Board,
        current_piece: Piece,
        hold_piece: Option<PieceType>,
        queue_pieces: &[PieceType],
        can_hold: bool,
    ) -> Self {
        Game {
            board,
            current_piece,
            hold_piece,
            queue_pieces: queue_pieces.iter().collect(),
            can_hold,
        }
    }
    pub fn from_bag(bag: &Bag) -> Self {
        let mut iter = bag.pieces().iter();
        Game {
            board: Board::new(),
            current_piece: Piece::from(*iter.next().unwrap()),
            hold_piece: None,
            queue_pieces: iter.copied().collect(),
            can_hold: true,
        }
    }
    pub fn from_bag_shuffled(bag: &mut Bag) -> Self {
        bag.shuffle();
        Game::from_bag(bag)
    }
    pub fn from_pieces(
        current_piece: PieceType,
        hold_piece: Option<PieceType>,
        queue_pieces: &[PieceType],
    ) -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::from(current_piece),
            hold_piece,
            queue_pieces: queue_pieces.into_iter().collect(),
            can_hold: true,
        }
    }
    pub fn from_stream(queue: &mut Stream) -> Self {
        Game {
            board: Board::new(),
            current_piece: Piece::from(queue.dequeue().unwrap()),
            hold_piece: None,
            queue_pieces: {
                let mut arr = ArrDeque::new();
                while arr.len() < GAME_MAX_QUEUE_LEN {
                    arr.push_back(queue.dequeue().unwrap());
                }
                arr
            },
            can_hold: true,
        }
    }

    pub fn set_current(&mut self, piece: PieceType) {
        self.current_piece.piece_type = piece;
        self.current_piece.reset();
        self.can_hold = true;
    }
    pub fn set_hold(&mut self, piece: Option<PieceType>) {
        self.hold_piece = piece;
        self.can_hold = true;
    }
    pub fn set_queue(&mut self, pieces: &[PieceType]) {
        self.clear_queue();
        self.extend_queue(pieces);
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
    pub fn refill_queue(&mut self, bag: &Bag) {
        const THRESHOLD: usize = GAME_MAX_QUEUE_LEN - BAG_LEN;
        if self.queue_pieces.len() <= THRESHOLD {
            self.extend_queue(bag.pieces());
        }
    }
    pub fn refill_queue_shuffled(&mut self, bag: &mut Bag) {
        const THRESHOLD: usize = GAME_MAX_QUEUE_LEN - BAG_LEN;
        if self.queue_pieces.len() <= THRESHOLD {
            bag.shuffle();
            self.extend_queue(bag.pieces());
        }
    }
    pub fn refill_queue_stream(&mut self, stream: &mut Stream) {
        while self.queue_pieces.len() < GAME_MAX_QUEUE_LEN && stream.len() > 0 {
            self.queue_pieces.push_back(stream.dequeue().unwrap());
        }
    }
    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }
    pub fn swap_hold(&mut self) -> SwapHoldRes {
        let hold = match self.hold_piece {
            Some(hold) => hold,
            None => match self.queue_pieces.pop_front() {
                Some(piece) => piece,
                None => return SwapHoldRes::Fail,
            },
        };
        self.hold_piece = Some(self.current_piece.piece_type);
        self.current_piece.piece_type = hold;
        self.current_piece.reset();
        SwapHoldRes::Success
    }

    pub fn apply_action(&mut self, game_action: GameAction) -> GameActionRes {
        match game_action {
            GameAction::ShiftLeft
            | GameAction::ShiftRight
            | GameAction::ShiftDown
            | GameAction::RotateCW
            | GameAction::Rotate180
            | GameAction::RotateCCW
            | GameAction::SoftDrop => {
                let piece_move = PieceAction::try_from(game_action).unwrap();
                self.current_piece.apply_action(piece_move, &self.board);
                GameActionRes::Success
            }
            GameAction::Hold => {
                if !self.can_hold {
                    return GameActionRes::Fail;
                }
                match self.swap_hold() {
                    SwapHoldRes::Success => {
                        self.can_hold = false;
                        GameActionRes::Success
                    }
                    SwapHoldRes::Fail => GameActionRes::Fail,
                }
            }
            GameAction::Lock => {
                if self.queue_pieces.len() == 0 {
                    return GameActionRes::Fail;
                }

                self.current_piece.soft_drop(&self.board);
                let res = self.board.lock(&self.current_piece);
                self.current_piece.piece_type = self.queue_pieces.pop_front().unwrap();
                self.current_piece.reset();
                self.can_hold = true;

                GameActionRes::SuccessDrop(GameLockInfo {
                    lines_cleared: res.lines_cleared,
                    top_out: res.top_out,
                })
            }
            GameAction::AddGarbage { col, height } => {
                self.board.add_garbage(col, height);
                GameActionRes::Success
            }
        }
    }
    pub fn make_move(&mut self, game_move: GameMove) -> GameMoveRes {
        match game_move {
            GameMove::ShiftLeft
            | GameMove::ShiftRight
            | GameMove::RotateCW
            | GameMove::Rotate180
            | GameMove::RotateCCW
            | GameMove::SoftDrop
            | GameMove::Hold => {
                let action = GameAction::try_from(game_move).unwrap();
                self.apply_action(action).into()
            }
            GameMove::HardDrop => {
                if self.queue_pieces.len() == 0 {
                    return GameMoveRes::Fail;
                }

                self.apply_action(GameAction::SoftDrop);
                let res = self.apply_action(GameAction::Lock);
                self.apply_action(GameAction::ShiftDown);

                if let GameActionRes::SuccessDrop(_) = res {
                    res.into()
                } else {
                    panic!("Expected GameActionRes::SuccessDrop(_)")
                }
            }
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Board + Current Piece
        write!(f, "{}", self.board.to_string(Some(&self.current_piece)))?;

        // Curr, Hold, and Queue pieces
        let curr = format!("{}", &self.current_piece);
        let hold = match &self.hold_piece {
            Some(piece) => {
                let can_hold = if self.can_hold { "✓" } else { "✗" };
                format!("{} {}", piece, can_hold)
            }
            None => format!(""),
        };
        let queue_text = self
            .queue_pieces
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "[{}] {}\n{}", hold, curr, queue_text)?;

        Ok(())
    }
}
