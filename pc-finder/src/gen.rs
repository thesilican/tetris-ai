use common::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
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
    // Similar to Game::child_states()
    // except for PcBoard
    pub fn children<'a>(
        &self,
        piece_type: PieceType,
        fragments: &'a Fragments,
    ) -> Vec<PcChild<'a>> {
        let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
        game.board = Board::from(*self);
        game.child_states(fragments)
            .into_iter()
            .filter_map(|child_state| {
                // Convert ChildState to Board
                let board = child_state.game.board.try_into();
                let moves = child_state.moves;
                match board {
                    Ok(board) => Some(PcChild { board, moves }),
                    Err(_) => None,
                }
            })
            .collect()
    }
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

#[derive(Debug, Clone)]
pub struct PcChild<'a> {
    pub board: PcBoard,
    pub moves: &'a [GameMove],
}

#[derive(Debug, Clone)]
pub struct PcGraph<'a> {
    pub graph: HashMap<PcBoard, HashMap<PieceType, Vec<PcChild<'a>>>>,
}
impl<'a> PcGraph<'a> {
    pub fn create(fragments: &'a Fragments) -> Self {
        PcGraph::generate(fragments).prune()
    }
    fn generate(fragments: &'a Fragments) -> Self {
        type PieceMap<'a> = HashMap<PieceType, Vec<PcChild<'a>>>;
        type Graph<'a> = HashMap<PcBoard, PieceMap<'a>>;
        type Visited = HashSet<PcBoard>;

        // From the empty PcBoard state,
        // create a graph of all
        //  PcBoard => PieceType => PcChild
        // transitions
        const INITIAL: PcBoard = PcBoard::new();
        let mut graph = Graph::new();
        let mut visited = Visited::new();
        let mut queue = VecDeque::new();
        queue.push_back(INITIAL);
        visited.insert(INITIAL);

        let mut i = 0;
        while let Some(board) = queue.pop_front() {
            println!("{:#}", board);
            println!("{} {}", i, queue.len());
            i += 1;
            let mut piece_map = PieceMap::new();
            for piece_type in PieceType::all() {
                let children = board.children(piece_type, fragments);
                for child in children.iter() {
                    if !visited.contains(&child.board) {
                        queue.push_back(child.board);
                        visited.insert(child.board);
                    }
                }
                piece_map.insert(piece_type, children);
            }
            graph.insert(board, piece_map);
        }
        PcGraph { graph }
    }
    // Given a PcGraph
    // Remove all nodes that do not have a path back to the initial state
    fn prune(self) -> Self {
        //
        todo!()
    }
}
