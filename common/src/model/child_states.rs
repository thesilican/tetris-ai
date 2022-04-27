use fnv::{FnvBuildHasher, FnvHashMap};

use super::game::{Game, GameMove, GameMoveRes};
use std::collections::hash_map::Entry;
use std::lazy::SyncLazy;

/// Represents a child state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildState<'a> {
    pub game: Game,
    pub moves: &'a [GameMove],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragment(pub Vec<Vec<GameMove>>);
impl Fragment {
    pub fn new(fragment: Vec<Vec<GameMove>>) -> Self {
        assert!(fragment.len() > 0);
        Fragment(fragment)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragments {
    pub fragments: Vec<Fragment>,
    pub perms: Vec<Vec<GameMove>>,
}

impl Fragments {
    pub fn new(fragments: &[Fragment]) -> Self {
        let mut perms = Vec::new();
        fn gen(perms: &mut Vec<Vec<GameMove>>, fragments: &[Fragment], accum: &[GameMove]) {
            if fragments.len() == 0 {
                perms.push(accum.to_vec());
            } else {
                for fragment in &fragments[0].0 {
                    let mut accum = accum.to_vec();
                    accum.extend(fragment);
                    gen(perms, &fragments[1..], &accum)
                }
            }
        }
        gen(&mut perms, fragments, &[]);
        Fragments {
            fragments: fragments.to_vec(),
            perms,
        }
    }
}

impl Game {
    pub fn child_states<'a>(&self, fragments: &'a Fragments) -> Vec<ChildState<'a>> {
        let mut child_states = Vec::<ChildState<'a>>::with_capacity(100);
        let mut map =
            FnvHashMap::<Game, usize>::with_capacity_and_hasher(100, FnvBuildHasher::default());
        self.child_states_noalloc(fragments, &mut child_states, &mut map);
        child_states
    }
    pub fn child_states_noalloc<'a>(
        &self,
        fragments: &'a Fragments,
        child_states: &mut Vec<ChildState<'a>>,
        map: &mut FnvHashMap<Game, usize>,
    ) {
        // Given a game, an array of fragments, and an array of permutations:
        // Take the first fragment of fragments, iterate over moves list of the fragment,
        // narrow down the permutation array, and recursively generate on the remaining fragments,
        // running the moves on the game in the process.
        // If the fragments array is empty, then we are at a leaf node of the tree of permutations.
        // perms will contain exactly 1 array of moves, and game will have had all the moves applied to it
        fn gen<'a>(
            child_states: &mut Vec<ChildState<'a>>,
            map: &mut FnvHashMap<Game, usize>,
            game: Game,
            fragments: &'a [Fragment],
            perms: &'a [Vec<GameMove>],
        ) {
            if fragments.len() > 0 {
                // Permutate over fragments
                let fragment = &fragments[0];
                let rest = &fragments[1..];
                'l: for (i, moves) in fragment.0.iter().enumerate() {
                    let mut game = game;
                    for &game_move in moves {
                        let res = game.make_move(game_move);
                        // Skip if move ever fails
                        if let GameMoveRes::Fail = res {
                            continue 'l;
                        }
                    }
                    let size = perms.len() / fragment.0.len();
                    let perms = &perms[size * i..size * (i + 1)];
                    gen(child_states, map, game, rest, perms);
                }
            } else {
                // Finished permutating fragments, only one element in perms should remain
                let moves = &*perms[0];
                match map.entry(game) {
                    Entry::Occupied(entry) => {
                        let &index = entry.get();
                        let other_moves = child_states[index].moves;
                        if moves.len() < other_moves.len() {
                            // Replace with faster moves
                            child_states[index].moves = moves;
                        }
                    }
                    Entry::Vacant(entry) => {
                        child_states.push(ChildState { game, moves });
                        entry.insert(child_states.len() - 1);
                    }
                }
            }
        }
        gen(
            child_states,
            map,
            *self,
            &fragments.fragments,
            &fragments.perms,
        );
    }
}

pub static FRAGMENT_HOLD: SyncLazy<Fragment> =
    SyncLazy::new(|| Fragment::new(vec![vec![], vec![GameMove::Hold]]));
pub static FRAGMENT_ROT: SyncLazy<Fragment> = SyncLazy::new(|| {
    Fragment::new(vec![
        vec![],
        vec![GameMove::RotateCW],
        vec![GameMove::Rotate180],
        vec![GameMove::RotateCCW],
    ])
});
pub static FRAGMENT_SHIFT: SyncLazy<Fragment> = SyncLazy::new(|| {
    Fragment::new(vec![
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
    ])
});
pub static FRAGMENT_FINAL: SyncLazy<Fragment> = SyncLazy::new(|| {
    Fragment::new(vec![
        vec![],
        vec![GameMove::SoftDrop, GameMove::ShiftLeft],
        vec![GameMove::SoftDrop, GameMove::ShiftRight],
        vec![GameMove::SoftDrop, GameMove::RotateCCW],
        vec![GameMove::SoftDrop, GameMove::Rotate180],
        vec![GameMove::SoftDrop, GameMove::RotateCW],
    ])
});
pub static FRAGMENT_DROP: SyncLazy<Fragment> =
    SyncLazy::new(|| Fragment::new(vec![vec![GameMove::HardDrop]]));

pub static MOVES_0F_NH: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
    ])
});

pub static MOVES_0F: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_DROP.clone(),
    ])
});

pub static MOVES_1F_NH: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
    ])
});

pub static MOVES_1F: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_DROP.clone(),
    ])
});

pub static MOVES_2F_NH: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
    ])
});

pub static MOVES_2F: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_DROP.clone(),
    ])
});

pub static MOVES_3F_NH: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
    ])
});

pub static MOVES_3F: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_DROP.clone(),
    ])
});

pub static MOVES_4F_NH: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
    ])
});

pub static MOVES_4F: SyncLazy<Fragments> = SyncLazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD.clone(),
        FRAGMENT_ROT.clone(),
        FRAGMENT_SHIFT.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_FINAL.clone(),
        FRAGMENT_DROP.clone(),
    ])
});
