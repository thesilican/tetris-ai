use common::model::{Game, GameDropInfo, GameMove, GameMoveRes, PieceType, GAME_MAX_QUEUE_LEN};
use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    iter::FromIterator,
};

#[derive(Debug, Clone, Copy)]
pub enum GameAction {
    ShiftLeft,
    ShiftRight,
    ShiftDown,
    RotateLeft,
    Rotate180,
    RotateRight,
    Hold,
    SoftDrop,
    HardDrop,
    AddGarbage { col: i32, height: i32 },
}
impl TryFrom<GameAction> for GameMove {
    type Error = ();

    fn try_from(value: GameAction) -> Result<Self, Self::Error> {
        match value {
            GameAction::ShiftLeft => Ok(GameMove::ShiftLeft),
            GameAction::ShiftRight => Ok(GameMove::ShiftRight),
            GameAction::RotateLeft => Ok(GameMove::RotateLeft),
            GameAction::Rotate180 => Ok(GameMove::Rotate180),
            GameAction::RotateRight => Ok(GameMove::RotateRight),
            GameAction::Hold => Ok(GameMove::Hold),
            GameAction::SoftDrop => Ok(GameMove::SoftDrop),
            GameAction::HardDrop => Ok(GameMove::HardDrop),
            GameAction::ShiftDown => Err(()),
            GameAction::AddGarbage { .. } => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameActionRes {
    SuccessNorm,
    SuccessDrop(GameDropInfo),
    Failed,
}
impl From<GameMoveRes> for GameActionRes {
    fn from(res: GameMoveRes) -> Self {
        match res {
            GameMoveRes::SuccessNorm => GameActionRes::SuccessNorm,
            GameMoveRes::SuccessDrop(drop_info) => GameActionRes::SuccessDrop(drop_info),
            GameMoveRes::Failed => GameActionRes::Failed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LongQueue(VecDeque<PieceType>);
impl LongQueue {
    pub fn new() -> Self {
        LongQueue(VecDeque::new())
    }
    pub fn enqueue(&mut self, piece: PieceType) {
        self.0.push_back(piece)
    }
    pub fn dequeue(&mut self) -> Option<PieceType> {
        self.0.pop_front()
    }
}
impl FromIterator<PieceType> for LongQueue {
    fn from_iter<T: IntoIterator<Item = PieceType>>(iter: T) -> Self {
        LongQueue(iter.into_iter().collect::<VecDeque<_>>())
    }
}

// Workaround for https://doc.rust-lang.org/error-index.html#E0116
pub trait GameExt {
    fn apply_action(&mut self, action: GameAction) -> GameActionRes;
    fn refill_long_queue(&mut self, queue: &mut LongQueue);
}

impl GameExt for Game {
    fn apply_action(&mut self, action: GameAction) -> GameActionRes {
        match action {
            GameAction::ShiftLeft
            | GameAction::ShiftRight
            | GameAction::RotateLeft
            | GameAction::Rotate180
            | GameAction::RotateRight
            | GameAction::Hold
            | GameAction::SoftDrop
            | GameAction::HardDrop => {
                let game_move = action.try_into().unwrap();
                let res = self.make_move(game_move);
                res.into()
            }
            GameAction::ShiftDown => {
                self.current_piece.shift_down(&self.board);
                GameActionRes::SuccessNorm
            }
            GameAction::AddGarbage { col, height } => {
                self.board.add_garbage(col, height);
                GameActionRes::SuccessNorm
            }
        }
    }
    fn refill_long_queue(&mut self, queue: &mut LongQueue) {
        while self.queue_pieces.len() < GAME_MAX_QUEUE_LEN {
            self.queue_pieces.push_back(queue.dequeue().unwrap());
        }
    }
}
