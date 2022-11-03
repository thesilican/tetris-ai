use super::game::{Game, GameActionRes, GameMove};
use once_cell::sync::Lazy;

/// Represents a child state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildState {
    pub game: Game,
    moves: &'static [&'static [GameMove]],
}
impl ChildState {
    pub fn moves(&self) -> impl Iterator<Item = GameMove> {
        self.moves.iter().flat_map(|x| x.iter().map(|x| *x))
    }
}

#[derive(Debug, Clone, Copy)]
struct Fragment(&'static [&'static [GameMove]]);
impl Fragment {
    pub const fn new(fragment: &'static [&'static [GameMove]]) -> Self {
        assert!(fragment.len() > 0);
        Fragment(fragment)
    }
}

#[derive(Debug)]
struct Perm {
    moves: Vec<&'static [GameMove]>,
    len: usize,
    has_soft_drop: bool,
}
impl Perm {
    fn new(moves: Vec<&'static [GameMove]>) -> Self {
        let len: usize = moves.iter().map(|x| x.len()).sum();
        let has_soft_drop = moves
            .iter()
            .any(|x| x.iter().any(|&x| x == GameMove::SoftDrop));
        Perm {
            moves,
            len,
            has_soft_drop,
        }
    }
    fn preferred_over(&self, other: &Self) -> bool {
        self.len < other.len || (self.len == other.len && !self.has_soft_drop)
    }
}

#[derive(Debug)]
pub struct Fragments {
    fragments: Vec<Fragment>,
    perms: Vec<Perm>,
}

impl Fragments {
    fn new(fragments: &[Fragment]) -> Self {
        let mut perms = Vec::new();
        fn gen(perms: &mut Vec<Perm>, fragments: &[Fragment], accum: &[&'static [GameMove]]) {
            if fragments.len() == 0 {
                perms.push(Perm::new(accum.to_vec()));
            } else {
                for &fragment in fragments[0].0 {
                    let mut accum = accum.to_vec();
                    accum.push(fragment);
                    gen(perms, &fragments[1..], &accum)
                }
            }
        }
        gen(&mut perms, &fragments, &[]);
        Fragments {
            fragments: fragments.to_vec(),
            perms,
        }
    }
}

impl Game {
    pub fn child_states(&self, fragments: &'static Fragments) -> Vec<ChildState> {
        let mut output = Vec::with_capacity(100);
        self.child_states_noalloc(fragments, &mut output);
        output
            .into_iter()
            .map(|x| ChildState {
                game: x.0,
                moves: &x.1.moves,
            })
            .collect()
    }
    fn child_states_noalloc(
        &self,
        fragments: &'static Fragments,
        output: &mut Vec<(Game, &'static Perm)>,
    ) {
        // Given a game, an array of fragments, and an array of permutations:
        // Take the first fragment of fragments, iterate over moves list of the fragment,
        // narrow down the permutation array, and recursively generate on the remaining fragments,
        // running the moves on the game in the process.
        // If the fragments array is empty, then we are at a leaf node of the tree of permutations.
        // perms will contain exactly 1 array of moves, and game will have had all the moves applied to it
        fn gen(
            output: &mut Vec<(Game, &'static Perm)>,
            game: Game,
            fragments: &'static [Fragment],
            perms: &'static [Perm],
        ) {
            if fragments.len() > 0 {
                // Permutate over fragments
                let fragment = &fragments[0];
                let rest = &fragments[1..];
                'l: for (i, &moves) in fragment.0.iter().enumerate() {
                    let mut game = game;
                    for &game_move in moves {
                        let res = game.make_move(game_move);
                        // Skip if move ever fails
                        if let GameActionRes::Fail = res {
                            continue 'l;
                        }
                    }
                    let size = perms.len() / fragment.0.len();
                    let perms = &perms[size * i..size * (i + 1)];
                    gen(output, game, rest, perms);
                }
            } else {
                // Finished permutating fragments, only one element in perms should remain
                let perm = &perms[0];
                for (other_game, other_perm) in output.iter_mut() {
                    if game != *other_game {
                        continue;
                    }
                    if perm.preferred_over(other_perm) {
                        *other_game = game;
                        *other_perm = perm;
                    }
                    return;
                }
                output.push((game, perm));
            }
        }
        gen(output, *self, &fragments.fragments, &fragments.perms);
    }
}

static FRAGMENT_HOLD: Fragment = Fragment::new(&[&[], &[GameMove::Hold]]);
static FRAGMENT_ROT: Fragment = Fragment::new(&[
    &[],
    &[GameMove::RotateCW],
    &[GameMove::Rotate180],
    &[GameMove::RotateCCW],
]);
static FRAGMENT_SHIFT: Fragment = Fragment::new(&[
    &[GameMove::ShiftLeft; 5],
    &[GameMove::ShiftLeft; 4],
    &[GameMove::ShiftLeft; 3],
    &[GameMove::ShiftLeft; 2],
    &[GameMove::ShiftLeft; 1],
    &[],
    &[GameMove::ShiftRight; 1],
    &[GameMove::ShiftRight; 2],
    &[GameMove::ShiftRight; 3],
    &[GameMove::ShiftRight; 4],
    &[GameMove::ShiftRight; 5],
]);
static FRAGMENT_FINAL: Fragment = Fragment::new(&[
    &[],
    &[GameMove::SoftDrop, GameMove::ShiftLeft],
    &[GameMove::SoftDrop, GameMove::ShiftRight],
    &[GameMove::SoftDrop, GameMove::RotateCCW],
    &[GameMove::SoftDrop, GameMove::Rotate180],
    &[GameMove::SoftDrop, GameMove::RotateCW],
]);
static FRAGMENT_DROP: Fragment = Fragment::new(&[&[GameMove::HardDrop]]);

pub static MOVES_0F_NH: Lazy<Fragments> =
    Lazy::new(|| Fragments::new(&[FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT]));

pub static MOVES_0F: Lazy<Fragments> =
    Lazy::new(|| Fragments::new(&[FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT, FRAGMENT_DROP]));

pub static MOVES_1F_NH: Lazy<Fragments> =
    Lazy::new(|| Fragments::new(&[FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT, FRAGMENT_FINAL]));

pub static MOVES_1F: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static MOVES_2F_NH: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static MOVES_2F: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static MOVES_3F_NH: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static MOVES_3F: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static MOVES_4F_NH: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static MOVES_4F: Lazy<Fragments> = Lazy::new(|| {
    Fragments::new(&[
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});
