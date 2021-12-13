use common::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcBoard([u16; 4]);

impl PcBoard {
    pub const fn new() -> Self {
        PcBoard([0; 4])
    }
    pub fn children(&self, piece_type: PieceType) -> impl Iterator<Item = PcChild> {
        let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
        game.board = Board::from(*self);
        game.child_states(&MOVES_2F)
            .into_iter()
            .filter(|child_state| child_state.game.board.matrix[2] == 0)
            .map(PcChild::from)
    }
}
impl From<Board> for PcBoard {
    fn from(board: Board) -> Self {
        PcBoard(board.matrix[0..4].try_into().unwrap())
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
        fn bits(num: u16) -> String {
            let mut res = String::new();
            for i in 0..10 {
                let s = if num & (1 << i) == 0 { "." } else { "@" };
                res.push_str(s);
            }
            res
        }
        writeln!(f, "{}", bits(self.0[3]))?;
        writeln!(f, "{}", bits(self.0[2]))?;
        writeln!(f, "{}", bits(self.0[1]))?;
        write!(f, "{}", bits(self.0[0]))
    }
}

#[derive(Debug, Clone)]
pub struct PcChild {
    pub board: PcBoard,
    pub moves: &'static [GameMove],
}
impl From<ChildState<'static>> for PcChild {
    fn from(child_state: ChildState<'static>) -> Self {
        PcChild {
            board: child_state.game.board.into(),
            moves: child_state.moves,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PcGraph {
    pub graph: HashMap<PcBoard, HashMap<PieceType, Vec<PcChild>>>,
}
impl PcGraph {
    pub fn generate() -> Self {
        type PieceMap = HashMap<PieceType, Vec<PcChild>>;
        type Graph = HashMap<PcBoard, PieceMap>;
        type Visited = HashSet<PcBoard>;
        let mut graph = Graph::new();
        let mut visited = Visited::new();
        const INITIAL: PcBoard = PcBoard::new();
        fn dfs(board: PcBoard, graph: &mut Graph, visited: &mut Visited) -> bool {
            if visited.contains(&board) {
                return graph.contains_key(&board);
            }
            visited.insert(board);

            let mut found = false;
            let mut piece_map = PieceMap::new();
            for piece_type in PieceType::all() {
                let mut children = Vec::<PcChild>::new();
                for child in board.children(piece_type) {
                    let res = dfs(child.board, graph, visited);
                    if res {
                        found = true;
                        children.push(child);
                    }
                }
                piece_map.insert(piece_type, children);
            }
            found
        }
        dfs(INITIAL, &mut graph, &mut visited);
        PcGraph { graph }
    }
}
