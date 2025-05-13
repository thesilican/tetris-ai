use crate::{Action, Game, PieceInfo};
use std::sync::LazyLock;

use super::{ActionInfo, LockInfo};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Child {
    pub game: Game,
    pub hold: bool,
    pub rotate: i8,
    pub shift: i8,
    pub fin: Fin,
    pub fin_idx: u8,
    pub lock_info: LockInfo,
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
        let drop: &[Action] = if self.fin_idx == 0 {
            &[]
        } else {
            &[Action::SoftDrop]
        };
        let fin = &self.fin.perm_set().0[self.fin_idx as usize].0;
        hold.iter()
            .chain(rot)
            .chain(shift)
            .chain(drop)
            .chain(fin)
            .chain(std::iter::once(&Action::HardDrop))
            .copied()
    }
}

impl Game {
    // /// Generate children with highly optimized code, no final actions. Assumes
    // /// active piece is in starting position.
    // pub fn children_fast(&self) -> Vec<Child> {
    //     let mut boardset = BoardSet::new();

    //     let mut max_height = BOARD_HEIGHT as i8;
    //     for i in 0..BOARD_HEIGHT as i8 {
    //         if self.board.matrix[i as usize] == 0 {
    //             max_height = i;
    //             break;
    //         }
    //     }

    //     if max_height > BOARD_VISIBLE_HEIGHT as i8
    //         || (self.active.position_x, self.active.position_y)
    //             != PieceInfo::spawn_location(self.active.piece_type)
    //         || self.active.rotation != 0
    //     {
    //         return Vec::new();
    //     }

    //     let mut height_map = [0; BOARD_WIDTH];
    //     for i in 0..BOARD_WIDTH {
    //         for j in (0..max_height).rev() {
    //             if self.board.get(i, j as usize) {
    //                 height_map[i] = j + 1;
    //                 break;
    //             }
    //         }
    //     }

    //     for do_hold in [false, true] {
    //         let piece_type;
    //         let active;
    //         let hold;
    //         let mut queue = self.queue;
    //         match (do_hold, self.hold) {
    //             (false, current_hold) => {
    //                 piece_type = self.active.piece_type;
    //                 let Some(active_type) = queue.dequeue() else {
    //                     continue;
    //                 };
    //                 active = Piece::from_piece_type(active_type);
    //                 hold = current_hold;
    //             }
    //             (true, Some(hold_piece)) => {
    //                 if !self.can_hold {
    //                     continue;
    //                 }
    //                 piece_type = hold_piece;
    //                 let Some(active_type) = queue.dequeue() else {
    //                     continue;
    //                 };
    //                 active = Piece::from_piece_type(active_type);
    //                 hold = Some(self.active.piece_type);
    //             }
    //             (true, None) => {
    //                 if !self.can_hold {
    //                     continue;
    //                 }
    //                 if let Some(val) = queue.dequeue() {
    //                     piece_type = val;
    //                 } else {
    //                     continue;
    //                 };
    //                 let Some(active_type) = queue.dequeue() else {
    //                     continue;
    //                 };
    //                 active = Piece::from_piece_type(active_type);
    //                 hold = Some(self.active.piece_type);
    //             }
    //         }

    //         let rotation_count = match piece_type {
    //             PieceType::O => 1,
    //             PieceType::I => 2,
    //             PieceType::T => 4,
    //             PieceType::L => 4,
    //             PieceType::J => 4,
    //             PieceType::S => 2,
    //             PieceType::Z => 2,
    //         };

    //         for rotation in 0..rotation_count {
    //             let (min_x, max_x, _, _) = PieceInfo::location_bound(piece_type, rotation);

    //             // Piece shift
    //             for position_x in min_x..=max_x {
    //                 let bit_shape = PieceInfo::bit_shape(piece_type, rotation, position_x);
    //                 let piece_map = PieceInfo::height_map(piece_type, rotation);

    //                 let mut position_y = i8::MIN;
    //                 for i in 0..4 {
    //                     let x = position_x + i;
    //                     if !(0..BOARD_WIDTH as i8).contains(&x) || piece_map[i as usize] == -1 {
    //                         continue;
    //                     }
    //                     position_y = position_y.max(height_map[x as usize] - piece_map[i as usize])
    //                 }

    //                 let mut matrix = [0; BOARD_HEIGHT];

    //                 let mut lines_cleared = 0;
    //                 let new_max_height = max_height + 4;
    //                 for j in 0..new_max_height {
    //                     let mut row = self.board.matrix[j as usize];
    //                     let p_y = j - position_y;
    //                     if (0..4).contains(&p_y) {
    //                         row |= bit_shape[p_y as usize];
    //                     }
    //                     if row == (1 << BOARD_WIDTH) - 1 {
    //                         lines_cleared += 1;
    //                     } else {
    //                         matrix[(j - lines_cleared) as usize] = row;
    //                     }
    //                 }
    //                 for j in 0..lines_cleared {
    //                     matrix[(BOARD_HEIGHT as i8 - lines_cleared + j) as usize] = 0;
    //                 }

    //                 let child = Child {
    //                     game: Game {
    //                         board: Board { matrix },
    //                         active,
    //                         hold,
    //                         queue,
    //                         can_hold: true,
    //                     },
    //                     hold: do_hold,
    //                     rotate: rotation,
    //                     shift: position_x - 3,
    //                     finish: 0,
    //                 };
    //                 boardset.insert(child);
    //             }
    //         }
    //     }
    //     boardset.to_entries()
    // }

    /// Generate children, up to 4 final actions.
    pub fn children(&self, fin: Fin) -> Vec<Child> {
        let mut boardset = BoardSet::new();
        for hold in [false, true] {
            let mut game = self.clone();
            if hold {
                let result = game.apply(Action::Hold);
                if let ActionInfo::Fail = result {
                    continue;
                }
            }

            for rotate in 0..4 {
                let mut game = game.clone();
                let result = match rotate {
                    0 => ActionInfo::Success,
                    1 => game.apply(Action::RotateCw),
                    2 => game.apply(Action::Rotate180),
                    3 => game.apply(Action::RotateCcw),
                    _ => unreachable!(),
                };
                if let ActionInfo::Fail = result {
                    continue;
                }

                let (min_x, max_x, _, _) =
                    PieceInfo::location_bound(game.active.piece_type, game.active.rotation);

                // Piece shift
                'shift: for position_x in min_x..=max_x {
                    let mut game = game.clone();
                    let shift = position_x - game.active.position_x;
                    if shift < 0 {
                        for _ in 0..(-shift) {
                            let result = game.apply(Action::ShiftLeft);
                            if let ActionInfo::Fail = result {
                                continue 'shift;
                            }
                        }
                    } else {
                        for _ in 0..shift {
                            let result = game.apply(Action::ShiftRight);
                            if let ActionInfo::Fail = result {
                                continue 'shift;
                            }
                        }
                    }

                    for (i, Perm(actions)) in fin.perm_set().0.iter().enumerate() {
                        let mut game = game.clone();
                        if actions.len() > 0 {
                            let result = game.apply(Action::SoftDrop);
                            if let ActionInfo::Fail = result {
                                continue;
                            }
                        }
                        for &action in actions {
                            let result = game.apply(action);
                            if let ActionInfo::Fail = result {
                                continue;
                            }
                        }

                        let action_info = game.apply(Action::HardDrop);
                        let lock_info = match action_info {
                            ActionInfo::Lock(info) => info,
                            ActionInfo::Success => unreachable!(),
                            ActionInfo::Fail => continue,
                        };

                        let child = Child {
                            game,
                            hold,
                            rotate,
                            shift,
                            fin,
                            fin_idx: i as u8,
                            lock_info,
                        };
                        boardset.insert(child);
                    }
                }
            }
        }
        boardset.to_entries()
    }
}

// Which set of final actions to do for a child
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Fin {
    None,
    Simple1,
    Simple2,
    Simple3,
    Full1,
    Full2,
    Full3,
}

impl Fin {
    pub fn perm_set(&self) -> &'static PermSet {
        &FINS[*self as u8 as usize]
    }
}

#[derive(Debug, Clone)]
pub struct PermSet(Vec<Perm>);

#[derive(Debug, Clone)]
pub struct Perm(Vec<Action>);

pub static FINS: LazyLock<Vec<PermSet>> = LazyLock::new(|| {
    use Action::{Rotate180, RotateCcw, RotateCw, ShiftLeft, ShiftRight};

    let simple = PermSet(vec![
        Perm(vec![]),
        Perm(vec![RotateCw]),
        Perm(vec![RotateCcw]),
        Perm(vec![ShiftLeft]),
        Perm(vec![ShiftRight]),
    ]);

    let full = PermSet(vec![
        Perm(vec![]),
        Perm(vec![RotateCw]),
        Perm(vec![Rotate180]),
        Perm(vec![RotateCcw]),
        Perm(vec![ShiftLeft]),
        Perm(vec![ShiftRight]),
    ]);

    fn product(set1: &PermSet, set2: &PermSet) -> PermSet {
        let mut output = Vec::new();
        for Perm(a) in &set1.0 {
            for Perm(b) in &set2.0 {
                let mut actions = Vec::<Action>::new();
                actions.extend(a);
                actions.extend(b);
                output.push(Perm(actions));
            }
        }
        PermSet(output)
    }

    let mut output = Vec::new();
    // None
    output.push(PermSet(vec![Perm(vec![])]));
    // Simple1
    output.push(simple.clone());
    // Simple2
    output.push(product(&simple, &simple));
    // Simple3
    output.push(product(&simple, &product(&simple, &simple)));
    // Full1
    output.push(full.clone());
    // Full2
    output.push(product(&full, &full));
    // Full3
    output.push(product(&full, &product(&full, &full)));
    output
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

    fn hash(&self, child: Child) -> u64 {
        const NOISE: u64 = 0x0123456789abcdef;
        let mut hash: u64 = 0;
        for (i, row) in child.game.board.matrix.chunks_exact(4).enumerate() {
            let num = row[0] as u64
                + ((row[1] as u64) << 16)
                + ((row[2] as u64) << 32)
                + ((row[3] as u64) << 48);
            hash ^= num.wrapping_mul(i as u64 + NOISE);
        }
        if child.hold {
            hash ^= NOISE;
        }
        hash
    }

    pub fn insert(&mut self, child: Child) {
        let mut idx = self.hash(child) as usize % BUCKETS;

        while let Some(entry) = &mut self.entries[idx] {
            if entry.hold == child.hold && entry.game.board == child.game.board {
                if child.fin_idx < entry.fin_idx {
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
