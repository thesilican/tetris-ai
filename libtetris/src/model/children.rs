use crate::{
    Action, Board, Game, Piece, PieceInfo, PieceType, BOARD_HEIGHT, BOARD_VISIBLE_HEIGHT,
    BOARD_WIDTH,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Child {
    pub game: Game,
    hold: bool,
    rotate: i8,
    shift: i8,
    finish: i8,
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
        let fin: &[Action] = match self.finish {
            0 => &[],
            1 => &[Action::ShiftLeft],
            2 => &[Action::ShiftRight],
            3 => &[Action::RotateCw],
            4 => &[Action::Rotate180],
            5 => &[Action::RotateCcw],
            _ => panic!("fin out of bounds"),
        };
        hold.iter()
            .chain(rot.iter())
            .chain(shift)
            .chain(drop)
            .chain(fin)
            .chain(std::iter::once(&Action::HardDrop))
            .copied()
    }
}

impl Game {
    pub fn children_fast(&self) -> Vec<Child> {
        let mut visited = BoardSet::new();

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
                    visited.insert(child);
                }
            }
        }
        visited.entries
    }
}

const BUCKETS: usize = 123;

pub struct BoardSet {
    hashset: [bool; BUCKETS],
    entries: Vec<Child>,
}

impl BoardSet {
    pub fn new() -> Self {
        BoardSet {
            hashset: [false; BUCKETS],
            entries: Vec::with_capacity(128),
        }
    }

    fn hash(&self, child: Child) -> u32 {
        let mut hash: u32 = 0;
        for (i, &row) in child.game.board.matrix.iter().enumerate() {
            hash += row as u32 * (i as u32 + 7);
        }
        if child.hold {
            hash += 1;
        }
        hash % BUCKETS as u32
    }

    pub fn insert(&mut self, child: Child) {
        let idx = self.hash(child) as usize;

        if self.hashset[idx] == false {
            self.hashset[idx] = true;
            self.entries.push(child);
            return;
        }

        if !self
            .entries
            .iter()
            .any(|other| other.hold == child.hold && other.game.board == child.game.board)
        {
            self.entries.push(child);
        }
    }
}
