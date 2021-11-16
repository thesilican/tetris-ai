use super::game::{Game, GameMove};
use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::lazy::SyncLazy;

/// Represents a child state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildState<'a> {
    pub game: Game,
    pub moves: &'a [GameMove],
}

impl Game {
    // Given a list of list of moves: &[Vec<GameMove>]
    // Return an array of unique child states
    // Which includes a game state plus a list of moves used to get there
    pub fn child_states<'a>(&self, moves_list: &'a [Vec<GameMove>]) -> Vec<ChildState<'a>> {
        let mut child_states = Vec::<ChildState<'a>>::new();
        let mut map = HashMap::<Game, usize>::new();
        for moves in moves_list {
            let mut game = self.clone();
            for game_move in moves {
                game.make_move(*game_move);
            }
            // Ignore topped-out games
            if game.board.topped_out() {
                continue;
            }
            match map.entry(game) {
                Entry::Occupied(entry) => {
                    let index = entry.get();
                    let other_moves = child_states[*index].moves;
                    if moves.len() < other_moves.len() {
                        // Replace with faster moves
                        child_states[*index].moves = moves;
                    }
                }
                Entry::Vacant(entry) => {
                    child_states.push(ChildState { game, moves });
                    entry.insert(child_states.len() - 1);
                }
            }
        }
        child_states
    }
}

pub static FRAGMENT_HOLD: SyncLazy<Vec<Vec<GameMove>>> =
    SyncLazy::new(|| vec![vec![], vec![GameMove::Hold]]);
pub static FRAGMENT_ROT: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![],
        vec![GameMove::RotateCW],
        vec![GameMove::Rotate180],
        vec![GameMove::RotateCCW],
    ]
});
pub static FRAGMENT_SHIFT: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![GameMove::ShiftLeft; 5],
        vec![GameMove::ShiftLeft; 4],
        vec![GameMove::ShiftLeft; 3],
        vec![GameMove::ShiftLeft; 2],
        vec![GameMove::ShiftLeft; 1],
        vec![],
        vec![GameMove::ShiftRight; 1],
        vec![GameMove::ShiftRight; 2],
        vec![GameMove::ShiftRight; 3],
        vec![GameMove::ShiftRight; 4],
        vec![GameMove::ShiftRight; 5],
    ]
});
pub static FRAGMENT_FINAL: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![],
        vec![GameMove::ShiftLeft],
        vec![GameMove::ShiftRight],
        vec![GameMove::RotateCCW],
        vec![GameMove::Rotate180],
        vec![GameMove::RotateCW],
    ]
});

pub static MOVES_0F_NH: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for hold in &*FRAGMENT_HOLD {
        for rot in &*FRAGMENT_ROT {
            for shift in &*FRAGMENT_SHIFT {
                let mut moves = Vec::new();
                moves.extend(hold);
                moves.extend(rot);
                moves.extend(shift);
                moves_list.push(moves);
            }
        }
    }
    moves_list
});

pub static MOVES_0F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let moves_list = MOVES_0F_NH
        .clone()
        .into_iter()
        .map(|mut x| {
            x.push(GameMove::HardDrop);
            x
        })
        .collect();
    moves_list
});

pub static MOVES_1F_NH: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    let mut moves_set = HashSet::new();
    for hold in &*FRAGMENT_HOLD {
        for rot in &*FRAGMENT_ROT {
            for shift in &*FRAGMENT_SHIFT {
                for final_1 in &*FRAGMENT_FINAL {
                    let mut moves = Vec::new();
                    moves.extend(hold);
                    moves.extend(rot);
                    moves.extend(shift);
                    moves.push(GameMove::SoftDrop);
                    moves.extend(final_1);
                    while let Some(GameMove::SoftDrop) = moves.last() {
                        moves.pop();
                    }
                    if !moves_set.contains(&moves) {
                        moves_set.insert(moves.clone());
                        moves_list.push(moves);
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_1F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let moves_list = MOVES_1F_NH
        .clone()
        .into_iter()
        .map(|mut x| {
            x.push(GameMove::HardDrop);
            x
        })
        .collect();
    moves_list
});

pub static MOVES_2F_NH: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    let mut moves_set = HashSet::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for hold in &*FRAGMENT_HOLD {
                for rot in &*FRAGMENT_ROT {
                    for shift in &*FRAGMENT_SHIFT {
                        let mut moves = Vec::new();
                        moves.extend(hold);
                        moves.extend(rot);
                        moves.extend(shift);
                        moves.push(GameMove::SoftDrop);
                        moves.extend(final_1);
                        moves.push(GameMove::SoftDrop);
                        moves.extend(final_2);
                        while let Some(GameMove::SoftDrop) = moves.last() {
                            moves.pop();
                        }
                        if !moves_set.contains(&moves) {
                            moves_set.insert(moves.clone());
                            moves_list.push(moves);
                        }
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_2F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let moves_list = MOVES_2F_NH
        .clone()
        .into_iter()
        .map(|mut x| {
            x.push(GameMove::HardDrop);
            x
        })
        .collect();
    moves_list
});

pub static MOVES_3F_NH: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    let mut moves_set = HashSet::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for final_3 in &*FRAGMENT_FINAL {
                for hold in &*FRAGMENT_HOLD {
                    for rot in &*FRAGMENT_ROT {
                        for shift in &*FRAGMENT_SHIFT {
                            let mut moves = Vec::new();
                            moves.extend(hold);
                            moves.extend(rot);
                            moves.extend(shift);
                            moves.push(GameMove::SoftDrop);
                            moves.extend(final_1);
                            moves.push(GameMove::SoftDrop);
                            moves.extend(final_2);
                            moves.push(GameMove::SoftDrop);
                            moves.extend(final_3);
                            while let Some(GameMove::SoftDrop) = moves.last() {
                                moves.pop();
                            }
                            if !moves_set.contains(&moves) {
                                moves_set.insert(moves.clone());
                                moves_list.push(moves);
                            }
                        }
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_3F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let moves_list = MOVES_3F_NH
        .clone()
        .into_iter()
        .map(|mut x| {
            x.push(GameMove::HardDrop);
            x
        })
        .collect();
    moves_list
});

pub static MOVES_4F_NH: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    let mut moves_set = HashSet::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for final_3 in &*FRAGMENT_FINAL {
                for final_4 in &*FRAGMENT_FINAL {
                    for hold in &*FRAGMENT_HOLD {
                        for rot in &*FRAGMENT_ROT {
                            for shift in &*FRAGMENT_SHIFT {
                                let mut moves = Vec::new();
                                moves.extend(hold);
                                moves.extend(rot);
                                moves.extend(shift);
                                moves.push(GameMove::SoftDrop);
                                moves.extend(final_1);
                                moves.push(GameMove::SoftDrop);
                                moves.extend(final_2);
                                moves.push(GameMove::SoftDrop);
                                moves.extend(final_3);
                                moves.push(GameMove::SoftDrop);
                                moves.extend(final_4);
                                while let Some(GameMove::SoftDrop) = moves.last() {
                                    moves.pop();
                                }
                                if !moves_set.contains(&moves) {
                                    moves_set.insert(moves.clone());
                                    moves_list.push(moves);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_4F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let moves_list = MOVES_4F_NH
        .clone()
        .into_iter()
        .map(|mut x| {
            x.push(GameMove::HardDrop);
            x
        })
        .collect();
    moves_list
});
