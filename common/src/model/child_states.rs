use super::game::{Game, GameMove};
use std::collections::{hash_map::Entry, HashMap};
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
        let mut map = HashMap::<Game, usize, _>::with_capacity_and_hasher(
            100,
            fnv::FnvBuildHasher::default(),
        );

        fn gen<'a>(
            child_states: &mut Vec<ChildState<'a>>,
            map: &mut HashMap<Game, usize, fnv::FnvBuildHasher>,
            game: Game,
            fragments: &'a [Fragment],
            perms: &'a [Vec<GameMove>],
        ) {
            if fragments.len() > 0 {
                let fragment = &fragments[0];
                let rest = &fragments[1..];
                for (i, moves) in fragment.0.iter().enumerate() {
                    let mut game = game;
                    for game_move in moves {
                        game.make_move(*game_move);
                    }
                    let size = perms.len() / fragment.0.len();
                    let perms = &perms[size * i..size * (i + 1)];
                    gen(child_states, map, game, rest, perms);
                }
            } else {
                let moves = &*perms[0];
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
        }
        gen(
            &mut child_states,
            &mut map,
            *self,
            &fragments.fragments,
            &fragments.perms,
        );
        child_states
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
