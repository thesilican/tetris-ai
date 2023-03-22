use super::game::{Game, GameActionRes, GameMove};
use once_cell::sync::Lazy;

type Run = &'static [GameMove];

type Fragment = &'static [Run];

#[derive(Debug, Clone, Copy)]
struct Perm<const N: usize> {
    runs: [Run; N],
    len: usize,
    has_soft_drop: bool,
}
impl<const N: usize> Perm<N> {
    fn new(runs: [Run; N]) -> Self {
        let len: usize = runs.iter().map(|x| x.len()).sum();
        let has_soft_drop = runs
            .iter()
            .any(|x| x.iter().any(|&x| x == GameMove::SoftDrop));
        Perm {
            runs,
            len,
            has_soft_drop,
        }
    }
    fn preferred_over(&self, other: &Self) -> bool {
        self.len < other.len || (self.len == other.len && !self.has_soft_drop)
    }
}

#[derive(Debug)]
pub struct Perms<const N: usize> {
    fragments: [Fragment; N],
    perms: Vec<Perm<N>>,
}
impl<const N: usize> Perms<N> {
    fn new(fragments: [Fragment; N]) -> Self {
        let mut perms = Vec::new();
        fn add_perms<const N: usize>(
            fragments: [Fragment; N],
            idx: usize,
            runs: [Run; N],
            perms: &mut Vec<Perm<N>>,
        ) {
            if idx == fragments.len() {
                perms.push(Perm::new(runs));
            } else {
                for &run in fragments[idx] {
                    let mut perm = runs;
                    perm[idx] = run;
                    add_perms(fragments, idx + 1, perm, perms);
                }
            }
        }
        add_perms(fragments, 0, [&[]; N], &mut perms);

        Perms { fragments, perms }
    }
}

/// Represents a child state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildState {
    pub game: Game,
    perm: &'static [Run],
}
impl ChildState {
    pub fn moves(&self) -> impl Iterator<Item = GameMove> + '_ {
        self.perm.iter().flat_map(|x| x.iter().map(|x| *x))
    }
}

impl Game {
    pub fn child_states<const N: usize>(&self, perms: &'static Perms<N>) -> Vec<ChildState> {
        let mut output = Vec::<(Game, &'static Perm<N>)>::new();
        let mut stack = Vec::<(Game, usize, usize)>::new();
        let mut dims = [1; N];
        for (i, fragment) in perms.fragments.iter().enumerate() {
            for j in 0..i {
                dims[j] *= fragment.len();
            }
        }

        stack.push((*self, 0, 0));
        'l: while let Some((game, depth, idx)) = stack.pop() {
            if depth == N {
                let perm = &perms.perms[idx];
                for (other_game, other_perm) in output.iter_mut() {
                    if game != *other_game {
                        continue;
                    }
                    if perm.preferred_over(other_perm) {
                        *other_game = game;
                        *other_perm = perm;
                    }
                    continue 'l;
                }
                output.push((game, perm));
            } else {
                let fragment = perms.fragments[depth];
                'o: for (i, &run) in fragment.iter().enumerate() {
                    let mut game = game;
                    for &game_move in run {
                        let res = game.make_move(game_move);
                        if let GameActionRes::Fail = res {
                            continue 'o;
                        }
                    }
                    stack.push((game, depth + 1, idx + i * dims[depth]));
                }
            }
        }

        output
            .into_iter()
            .map(|(game, perm)| ChildState {
                game,
                perm: &perm.runs,
            })
            .collect()
    }
}

static FRAGMENT_HOLD: Fragment = &[&[], &[GameMove::Hold]];
static FRAGMENT_ROT: Fragment = &[
    &[],
    &[GameMove::RotateCW],
    &[GameMove::Rotate180],
    &[GameMove::RotateCCW],
];
static FRAGMENT_SHIFT: Fragment = &[
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
];
static FRAGMENT_FINAL: Fragment = &[
    &[],
    &[GameMove::SoftDrop, GameMove::ShiftLeft],
    &[GameMove::SoftDrop, GameMove::ShiftRight],
    &[GameMove::SoftDrop, GameMove::RotateCCW],
    &[GameMove::SoftDrop, GameMove::Rotate180],
    &[GameMove::SoftDrop, GameMove::RotateCW],
];
static FRAGMENT_DROP: Fragment = &[&[GameMove::HardDrop]];

pub static PERMS_0F_NH: Lazy<Perms<3>> =
    Lazy::new(|| Perms::new([FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT]));

pub static PERMS_0F: Lazy<Perms<4>> =
    Lazy::new(|| Perms::new([FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT, FRAGMENT_DROP]));

pub static PERMS_1F_NH: Lazy<Perms<4>> =
    Lazy::new(|| Perms::new([FRAGMENT_HOLD, FRAGMENT_ROT, FRAGMENT_SHIFT, FRAGMENT_FINAL]));

pub static PERMS_1F: Lazy<Perms<5>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static PERMS_2F_NH: Lazy<Perms<5>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static PERMS_2F: Lazy<Perms<6>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static PERMS_3F_NH: Lazy<Perms<6>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static PERMS_3F: Lazy<Perms<7>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_DROP,
    ])
});

pub static PERMS_4F_NH: Lazy<Perms<7>> = Lazy::new(|| {
    Perms::new([
        FRAGMENT_HOLD,
        FRAGMENT_ROT,
        FRAGMENT_SHIFT,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
        FRAGMENT_FINAL,
    ])
});

pub static PERMS_4F: Lazy<Perms<8>> = Lazy::new(|| {
    Perms::new([
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
