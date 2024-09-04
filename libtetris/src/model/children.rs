use crate::{
    Action, Board, Game, Piece, PieceInfo, PieceType, BOARD_HEIGHT, BOARD_VISIBLE_HEIGHT,
    BOARD_WIDTH,
};
use std::sync::LazyLock;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Child {
    pub game: Game,
    hold: bool,
    rotate: i8,
    shift: i8,
    // A 4 digit base-6 number representing the final actions
    // of a child game (why did I do this)
    finish: u16,
}

impl Child {
    pub fn actions(&self) -> impl Iterator<Item = Action> {
        let hold: &[Action] = if self.hold { &[Action::Hold] } else { &[] };
        let rot: &[Action] = match self.rotate {
            0 => &[],
            1 => &[Action::RotateCw],
            2 => &[Action::Rotate180],
            3 => &[Action::RotateCcw],
            _ => panic!("rotation out of bounds"),
        };
        let shift = if self.shift < 0 {
            std::iter::repeat(&Action::ShiftLeft).take(-self.shift as usize)
        } else {
            std::iter::repeat(&Action::ShiftRight).take(self.shift as usize)
        };
        let drop: &[Action] = if self.finish == 0 {
            &[]
        } else {
            &[Action::SoftDrop]
        };
        let fin: &[Action] = match self.finish % 6 {
            0 => &[],
            1 => &[Action::ShiftLeft],
            2 => &[Action::ShiftRight],
            3 => &[Action::RotateCw],
            4 => &[Action::RotateCcw],
            5 => &[Action::Rotate180],
            _ => unreachable!(),
        };
        let fin2: &[Action] = match (self.finish / 6) % 6 {
            0 => &[],
            1 => &[Action::ShiftLeft],
            2 => &[Action::ShiftRight],
            3 => &[Action::RotateCw],
            4 => &[Action::RotateCcw],
            5 => &[Action::Rotate180],
            _ => unreachable!(),
        };
        let fin3: &[Action] = match (self.finish / 36) % 6 {
            0 => &[],
            1 => &[Action::ShiftLeft],
            2 => &[Action::ShiftRight],
            3 => &[Action::RotateCw],
            4 => &[Action::RotateCcw],
            5 => &[Action::Rotate180],
            _ => unreachable!(),
        };
        let fin4: &[Action] = match (self.finish / 216) % 6 {
            0 => &[],
            1 => &[Action::ShiftLeft],
            2 => &[Action::ShiftRight],
            3 => &[Action::RotateCw],
            4 => &[Action::RotateCcw],
            5 => &[Action::Rotate180],
            _ => unreachable!(),
        };
        hold.iter()
            .chain(rot)
            .chain(shift)
            .chain(drop)
            .chain(fin)
            .chain(fin2)
            .chain(fin3)
            .chain(fin4)
            .chain(std::iter::once(&Action::HardDrop))
            .copied()
    }
}

impl Game {
    /// Generate children with highly optimized code, no final actions. Assumes
    /// active piece is in starting position.
    pub fn children_fast(&self) -> Vec<Child> {
        let mut boardset = BoardSet::new();

        let mut max_height = BOARD_HEIGHT as i8;
        for i in 0..BOARD_HEIGHT as i8 {
            if self.board.matrix[i as usize] == 0 {
                max_height = i;
                break;
            }
        }

        if max_height > BOARD_VISIBLE_HEIGHT as i8
            || (self.active.position_x, self.active.position_y)
                != PieceInfo::spawn_location(self.active.piece_type)
            || self.active.rotation != 0
        {
            return Vec::new();
        }

        let mut height_map = [0; BOARD_WIDTH];
        for i in 0..BOARD_WIDTH {
            for j in (0..max_height).rev() {
                if self.board.get(i, j as usize) {
                    height_map[i] = j + 1;
                    break;
                }
            }
        }

        for do_hold in [false, true] {
            let piece_type;
            let active;
            let hold;
            let mut queue = self.queue;
            match (do_hold, self.hold) {
                (false, current_hold) => {
                    piece_type = self.active.piece_type;
                    let Some(active_type) = queue.dequeue() else {
                        continue;
                    };
                    active = Piece::from_piece_type(active_type);
                    hold = current_hold;
                }
                (true, Some(hold_piece)) => {
                    piece_type = hold_piece;
                    let Some(active_type) = queue.dequeue() else {
                        continue;
                    };
                    active = Piece::from_piece_type(active_type);
                    hold = Some(self.active.piece_type);
                }
                (true, None) => {
                    if let Some(val) = queue.dequeue() {
                        piece_type = val;
                    } else {
                        continue;
                    };
                    let Some(active_type) = queue.dequeue() else {
                        continue;
                    };
                    active = Piece::from_piece_type(active_type);
                    hold = Some(self.active.piece_type);
                }
            }

            let rotation_count = match piece_type {
                PieceType::O => 1,
                PieceType::I => 2,
                PieceType::T => 4,
                PieceType::L => 4,
                PieceType::J => 4,
                PieceType::S => 2,
                PieceType::Z => 2,
            };

            for rotation in 0..rotation_count {
                let (min_x, max_x, _, _) = PieceInfo::location_bound(piece_type, rotation);

                // Piece shift
                for position_x in min_x..=max_x {
                    let bit_shape = PieceInfo::bit_shape(piece_type, rotation, position_x);
                    let piece_map = PieceInfo::height_map(piece_type, rotation);

                    let mut position_y = i8::MIN;
                    for i in 0..4 {
                        let x = position_x + i;
                        if !(0..BOARD_WIDTH as i8).contains(&x) || piece_map[i as usize] == -1 {
                            continue;
                        }
                        position_y = position_y.max(height_map[x as usize] - piece_map[i as usize])
                    }

                    let mut matrix = [0; BOARD_HEIGHT];

                    let mut lines_cleared = 0;
                    let new_max_height = max_height + 4;
                    for j in 0..new_max_height {
                        let mut row = self.board.matrix[j as usize];
                        let p_y = j - position_y;
                        if (0..4).contains(&p_y) {
                            row |= bit_shape[p_y as usize];
                        }
                        if row == (1 << BOARD_WIDTH) - 1 {
                            lines_cleared += 1;
                        } else {
                            matrix[(j - lines_cleared) as usize] = row;
                        }
                    }
                    for j in 0..lines_cleared {
                        matrix[(BOARD_HEIGHT as i8 - lines_cleared + j) as usize] = 0;
                    }

                    let child = Child {
                        game: Game {
                            board: Board { matrix },
                            active,
                            hold,
                            queue,
                            can_hold: true,
                        },
                        hold: do_hold,
                        rotate: rotation,
                        shift: position_x - 3,
                        finish: 0,
                    };
                    boardset.insert(child);
                }
            }
        }
        boardset.to_entries()
    }

    /// Generate children, up to 4 final actions.
    pub fn children(&self, fin: usize) -> Vec<Child> {
        assert!(fin <= 4);

        let mut boardset = BoardSet::new();
        for hold in [false, true] {
            let mut game = self.clone();
            if hold {
                game.apply(Action::Hold);
            }

            for rotate in 0..4 {
                let mut game = game.clone();
                match rotate {
                    0 => {}
                    1 => {
                        game.apply(Action::RotateCw);
                    }
                    2 => {
                        game.apply(Action::Rotate180);
                    }
                    3 => {
                        game.apply(Action::RotateCcw);
                    }
                    _ => unreachable!(),
                }

                let (min_x, max_x, _, _) =
                    PieceInfo::location_bound(game.active.piece_type, game.active.rotation);

                // Piece shift
                for position_x in min_x..=max_x {
                    let mut game = game.clone();
                    let shift = position_x - game.active.position_x;
                    if shift < 0 {
                        for _ in 0..(-shift) {
                            game.apply(Action::ShiftLeft);
                        }
                    } else {
                        for _ in 0..shift {
                            game.apply(Action::ShiftRight);
                        }
                    }

                    for perm in &FIN_PERMUTATIONS[fin] {
                        let mut game = game.clone();
                        if perm.actions.len() > 0 {
                            game.apply(Action::SoftDrop);
                        }
                        for &action in &perm.actions {
                            game.apply(action);
                        }

                        game.apply(Action::HardDrop);
                        let child = Child {
                            game,
                            hold,
                            rotate,
                            shift,
                            finish: perm.id,
                        };
                        boardset.insert(child);
                    }
                }
            }
        }
        boardset.to_entries()
    }
}

#[derive(Debug, Clone)]
pub struct Perm {
    actions: Vec<Action>,
    id: u16,
}

pub static FIN_PERMUTATIONS: LazyLock<Vec<Vec<Perm>>> = LazyLock::new(|| {
    use Action::{Rotate180, RotateCcw, RotateCw, ShiftLeft, ShiftRight};
    let mut fin_permutations = Vec::<Vec<Perm>>::new();
    fin_permutations.push(vec![Perm {
        actions: Vec::new(),
        id: 0,
    }]);
    for i in 1..=4 {
        let prev_perms = fin_permutations.last().unwrap();
        let mut new_perms = prev_perms.clone();
        for prev_perm in prev_perms {
            if prev_perm.actions.len() == i - 1 {
                for (j, action) in [ShiftLeft, ShiftRight, RotateCw, RotateCcw, Rotate180]
                    .into_iter()
                    .enumerate()
                {
                    let mut actions = prev_perm.actions.clone();
                    actions.push(action);
                    let new_perm = Perm {
                        actions,
                        id: prev_perm.id + 6u16.pow(i as u32 - 1) * (j as u16 + 1),
                    };
                    new_perms.push(new_perm);
                }
            }
        }
        fin_permutations.push(new_perms);
    }
    fin_permutations
});

const BUCKETS: usize = 255;

/// Hashset of board
pub struct BoardSet {
    entries: Box<[Option<Child>; BUCKETS]>,
    insert_order: Vec<usize>,
}

impl BoardSet {
    pub fn new() -> Self {
        BoardSet {
            entries: Box::new([None; BUCKETS]),
            insert_order: Vec::with_capacity(BUCKETS),
        }
    }

    fn hash(&self, child: Child) -> u32 {
        let mut hash: u32 = 0;
        for (i, &row) in child.game.board.matrix.iter().enumerate() {
            hash ^= (row as u32).wrapping_mul(i as u32 + 1234567890);
        }
        if child.hold {
            hash ^= 1234567890;
        }
        hash
    }

    pub fn insert(&mut self, child: Child) {
        let mut idx = self.hash(child) as usize % BUCKETS;

        while let Some(entry) = &mut self.entries[idx] {
            if entry.hold == child.hold && entry.game.board == child.game.board {
                if child.finish < entry.finish {
                    *entry = child;
                }
                return;
            }
            idx = (idx + 1) % BUCKETS;
        }
        self.entries[idx] = Some(child);
        self.insert_order.push(idx);
    }

    pub fn to_entries(self) -> Vec<Child> {
        self.insert_order
            .into_iter()
            .map(|idx| self.entries[idx].unwrap())
            .collect()
    }
}
