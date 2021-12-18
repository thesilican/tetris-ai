use common::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the bottom 4 rows of a tetris board
/// Invariant: must be valid (see PcBoard::is_valid())
pub struct PcBoard([u16; 4]);

impl PcBoard {
    pub const fn new() -> Self {
        PcBoard([0; 4])
    }
    pub const fn from_rows(rows: [u16; 4]) -> Self {
        PcBoard(rows)
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        self.0[y as usize] >> x & 1 == 1
    }
    pub fn set(&mut self, x: i32, y: i32, on: bool) {
        if on {
            self.0[y as usize] |= 1 << x;
        } else {
            self.0[y as usize] &= !(1 << x);
        }
    }

    pub fn is_valid(&self) -> bool {
        // Returns the number of empty tiles that are all connected to each other
        // horizontally/vertically (the "neighbourhood") for an empty cell,
        // or returns None if that cell is filled
        fn get_neighbourhood_count(board: &PcBoard, x: i32, y: i32) -> Option<i32> {
            if board.get(x, y) {
                return None;
            }

            let mut visited = HashSet::new();
            visited.insert((x, y));
            let mut stack = vec![(x, y)];
            while let Some((x, y)) = stack.pop() {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if !(0..10).contains(&nx) || !(0..4).contains(&ny) {
                        continue;
                    }
                    if visited.contains(&(nx, ny)) {
                        continue;
                    }
                    if board.get(nx, ny) {
                        continue;
                    }
                    stack.push((nx, ny));
                    visited.insert((nx, ny));
                }
            }
            Some(visited.len() as i32)
        }
        for x in 0..10 {
            for y in 0..4 {
                let count = get_neighbourhood_count(self, x, y);
                if let Some(count) = count {
                    if count % 4 != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
    // pub fn children(&self, piece_type: PieceType) -> impl Iterator<Item = PcChild> {
    //     let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
    //     game.board = Board::from(*self);
    //     game.child_states(&MOVES_2F)
    //         .into_iter()
    //         .filter(|child_state| child_state.game.board.matrix[2] == 0)
    //         .map(PcChild::from)
    // }
}
impl TryFrom<Board> for PcBoard {
    type Error = ();

    /// Fails if the height of the board is greater than 4
    /// or if self is not valid
    fn try_from(value: Board) -> Result<Self, Self::Error> {
        if value.matrix[4] != 0 {
            return Err(());
        }
        let board = PcBoard(value.matrix[0..4].try_into().unwrap());
        if !board.is_valid() {
            return Err(());
        }
        Ok(board)
    }
}
impl From<PcBoard> for Board {
    fn from(pc_board: PcBoard) -> Self {
        let mut board = Board::new();
        for (i, row) in pc_board.0.into_iter().enumerate() {
            board.set_row(i, row);
        }
        board
    }
}
impl Display for PcBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '\n' } else { '/' };
        for y in (0..4).rev() {
            for x in 0..10 {
                let bit = if self.get(x, y) { '@' } else { '.' };
                write!(f, "{}", bit)?;
            }
            if y != 0 {
                write!(f, "{}", sep)?;
            }
        }
        Ok(())
    }
}

// #[derive(Debug, Clone)]
// pub struct PcChild {
//     pub board: PcBoard,
//     pub moves: &'static [GameMove],
// }
// impl From<ChildState<'static>> for PcChild {
//     fn from(child_state: ChildState<'static>) -> Self {
//         PcChild {
//             board: child_state.game.board.into(),
//             moves: child_state.moves,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct PcGraph {
//     pub graph: HashMap<PcBoard, HashMap<PieceType, Vec<PcChild>>>,
// }
// impl PcGraph {
//     pub fn generate() -> Self {
//         type PieceMap = HashMap<PieceType, Vec<PcChild>>;
//         type Graph = HashMap<PcBoard, PieceMap>;
//         type Visited = HashSet<PcBoard>;
//         let mut graph = Graph::new();
//         let mut visited = Visited::new();
//         const INITIAL: PcBoard = PcBoard::new();
//         fn dfs(board: PcBoard, graph: &mut Graph, visited: &mut Visited) -> bool {
//             if visited.contains(&board) {
//                 return graph.contains_key(&board);
//             }
//             visited.insert(board);

//             let mut found = false;
//             let mut piece_map = PieceMap::new();
//             for piece_type in PieceType::all() {
//                 let mut children = Vec::<PcChild>::new();
//                 for child in board.children(piece_type) {
//                     let res = dfs(child.board, graph, visited);
//                     if res {
//                         found = true;
//                         children.push(child);
//                     }
//                 }
//                 piece_map.insert(piece_type, children);
//             }
//             found
//         }
//         dfs(INITIAL, &mut graph, &mut visited);
//         PcGraph { graph }
//     }
// }
