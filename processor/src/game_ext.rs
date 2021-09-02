use common::{
    misc::ArrDeque,
    model::{
        Board, Game, GameDropInfo, GameMove, GameMoveRes, Piece, PieceType, GAME_MAX_QUEUE_LEN,
    },
};
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    convert::{TryFrom, TryInto},
    iter::FromIterator,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
impl From<GameMove> for GameAction {
    fn from(value: GameMove) -> Self {
        match value {
            GameMove::ShiftLeft => GameAction::ShiftLeft,
            GameMove::ShiftRight => GameAction::ShiftRight,
            GameMove::RotateLeft => GameAction::RotateLeft,
            GameMove::RotateRight => GameAction::RotateRight,
            GameMove::Rotate180 => GameAction::Rotate180,
            GameMove::SoftDrop => GameAction::SoftDrop,
            GameMove::Hold => GameAction::Hold,
            GameMove::HardDrop => GameAction::HardDrop,
        }
    }
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
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl FromIterator<PieceType> for LongQueue {
    fn from_iter<T: IntoIterator<Item = PieceType>>(iter: T) -> Self {
        LongQueue(iter.into_iter().collect::<VecDeque<_>>())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildStateAction<'a> {
    pub game: Game,
    pub actions: &'a [GameAction],
}

// Workaround for https://doc.rust-lang.org/error-index.html#E0116
pub trait GameExt {
    fn from_long_queue(queue: &mut LongQueue) -> Self;
    fn apply_action(&mut self, action: GameAction) -> GameActionRes;
    fn refill_long_queue(&mut self, queue: &mut LongQueue);
    fn child_states_action<'a>(
        &self,
        moves_list: &'a [Vec<GameAction>],
    ) -> Vec<ChildStateAction<'a>>;
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
        while self.queue_pieces.len() < GAME_MAX_QUEUE_LEN && queue.len() > 0 {
            self.queue_pieces.push_back(queue.dequeue().unwrap());
        }
    }

    fn from_long_queue(queue: &mut LongQueue) -> Self {
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

    // Basically copy pasted game.child_states(), but using GameAction
    fn child_states_action<'a>(
        &self,
        actions_list: &'a [Vec<GameAction>],
    ) -> Vec<ChildStateAction<'a>> {
        let mut child_states = Vec::<ChildStateAction<'a>>::new();
        let mut map = HashMap::<Game, usize>::new();
        for actions in actions_list {
            let mut game = self.clone();
            for game_action in actions {
                game.apply_action(*game_action);
            }
            match map.entry(game) {
                Entry::Occupied(entry) => {
                    let index = entry.get();
                    let other_actions = child_states[*index].actions;
                    if actions.len() < other_actions.len() {
                        // Replace with faster actions
                        child_states[*index].actions = actions;
                    }
                }
                Entry::Vacant(entry) => {
                    child_states.push(ChildStateAction { game, actions });
                    entry.insert(child_states.len() - 1);
                }
            }
        }
        child_states
    }
}
