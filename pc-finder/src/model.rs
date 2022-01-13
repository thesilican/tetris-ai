use common::*;
use std::{
    // collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    fmt::Display,
    // io::{Read, Write},
    iter::empty,
    lazy::SyncLazy,
};

// Fragments used for generating child PcBoards
pub static FRAGMENTS: &SyncLazy<Fragments> = &MOVES_2F;
// // Look up table to get index of an unknown &[GameMove]
// static FRAGMENTS_IDX: SyncLazy<HashMap<&[GameMove], usize>> = SyncLazy::new(|| {
//     let mut hash_map = HashMap::new();
//     for (i, moves) in FRAGMENTS.perms.iter().enumerate() {
//         hash_map.insert(&**moves, i);
//     }
//     hash_map
// });

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
        // Ensure that all contiguous regions of empty space
        // have a size that is a multiple of 4,
        // Also ensure that the total number of contiguous
        // regions ('islands') is 5 or less
        let mut stack = Vec::with_capacity(40);
        let mut visited = [[false; 4]; 10];
        // let mut islands = 0;
        for x in 0..10 {
            for y in 0..4 {
                if visited[x as usize][y as usize] {
                    continue;
                }

                // Found new contiguous region
                // islands += 1;

                // DFS through contiguous regions
                let mut count = 0;
                stack.push((x, y));
                visited[x as usize][y as usize] = true;
                while let Some((x, y)) = stack.pop() {
                    count += 1;
                    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let (nx, ny) = (x + dx, y + dy);
                        if !(0..10).contains(&nx) || !(0..4).contains(&ny) {
                            continue;
                        }
                        if visited[nx as usize][ny as usize] {
                            continue;
                        }
                        if self.get(nx, ny) != self.get(x, y) {
                            continue;
                        }
                        stack.push((nx, ny));
                        visited[nx as usize][ny as usize] = true;
                    }
                }
                if self.get(x, y) == false && count % 4 != 0 {
                    return false;
                }
            }
        }
        // if islands > 5 {
        //     return false;
        // }
        true
    }

    // // Similar to Game::child_states()
    // // except for PcBoard
    // pub fn children(&self, piece_type: PieceType) -> Vec<PcChild> {
    //     let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
    //     game.board = Board::from(*self);
    //     game.child_states(FRAGMENTS)
    //         .into_iter()
    //         .filter_map(|child_state| {
    //             // Convert ChildState to Board
    //             let board = child_state.game.board.try_into();
    //             let moves = child_state.moves;
    //             match board {
    //                 Ok(board) => Some(PcChild { board, moves }),
    //                 Err(_) => None,
    //             }
    //         })
    //         .collect()
    // }

    pub fn child_boards(&self) -> impl Iterator<Item = PcBoard> {
        let mut iter: Box<dyn Iterator<Item = PcBoard>> = Box::new(empty());
        for piece_type in PieceType::all() {
            let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
            game.board = Board::from(*self);
            let piece_iter = game
                .child_states(FRAGMENTS)
                .into_iter()
                .filter_map(|child_state| {
                    // Convert ChildState to Board
                    PcBoard::try_from(child_state.game.board).ok()
                });
            iter = Box::new(iter.chain(piece_iter));
        }
        iter
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct PcChild {
//     pub board: PcBoard,
//     pub moves: &'static [GameMove],
// }
// impl PcChild {
//     pub fn new(board: PcBoard, moves: &'static [GameMove]) -> Self {
//         PcChild { board, moves }
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct PcGraph {
//     pub graph: HashMap<PcBoard, HashMap<PieceType, Vec<PcChild>>>,
// }
// impl PcGraph {
//     pub fn create() -> Self {
//         PcGraph::generate().prune()
//     }
//     // From the initial position
//     // generate a tree of all possible states
//     pub fn generate() -> Self {
//         type PieceMap = HashMap<PieceType, Vec<PcChild>>;
//         type Graph = HashMap<PcBoard, PieceMap>;
//         type Visited = HashSet<PcBoard>;

//         // From the empty PcBoard state,
//         // create a graph of all
//         //  PcBoard => PieceType => PcChild
//         // transitions
//         const INITIAL: PcBoard = PcBoard::new();
//         let mut graph = Graph::new();
//         let mut visited = Visited::new();
//         let mut queue = VecDeque::new();
//         queue.push_back(INITIAL);
//         visited.insert(INITIAL);

//         let mut i = 0;
//         while let Some(board) = queue.pop_front() {
//             println!("{:#}", board);
//             println!("{} {}", i, queue.len());
//             i += 1;
//             let mut piece_map = PieceMap::new();
//             for piece_type in PieceType::all() {
//                 let children = board.children(piece_type);
//                 for child in children.iter() {
//                     if !visited.contains(&child.board) {
//                         queue.push_back(child.board);
//                         visited.insert(child.board);
//                     }
//                 }
//                 piece_map.insert(piece_type, children);
//             }
//             graph.insert(board, piece_map);
//         }
//         PcGraph { graph }
//     }

//     // Given a PcGraph
//     // Remove all nodes that do not have a path back to the initial state
//     pub fn prune(self) -> Self {
//         //
//         todo!()
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// A more compact version of PcBoard for serialization
/// LSB -------> MSB     LSB  MSB
/// aaaaaaaabb000000     aaaaaaaa
/// bbbbbbcccc000000     bbbbbbbb
/// ccccdddddd000000 <=> cccccccc
/// ddeeeeeeee000000     dddddddd
///                      eeeeeeee
pub struct PcBoardSer([u8; 5]);
impl PcBoardSer {
    pub fn new(value: [u8; 5]) -> Self {
        PcBoardSer(value)
    }
    pub fn from_u64(value: u64) -> Self {
        let mut bytes = [0; 5];
        for i in 0..5 {
            bytes[i] = value.to_le_bytes()[i];
        }
        PcBoardSer::new(bytes)
    }
    pub fn to_u64(self) -> u64 {
        let mut bytes = [0; 8];
        for i in 0..5 {
            bytes[i] = self.0[i];
        }
        u64::from_le_bytes(bytes)
    }
}
impl From<PcBoard> for PcBoardSer {
    fn from(val: PcBoard) -> Self {
        let val = val.0;
        let mut rows = [0; 5];
        rows[0] = (val[0] & 0b11111111) as u8;
        rows[1] = ((val[0] >> 8 & 0b11) | (val[1] & 0b111111) << 2) as u8;
        rows[2] = ((val[1] >> 6 & 0b1111) | (val[2] & 0b1111) << 4) as u8;
        rows[3] = ((val[2] >> 4 & 0b111111) | (val[3] & 0b11) << 6) as u8;
        rows[4] = (val[3] >> 2 & 0b11111111) as u8;
        PcBoardSer(rows)
    }
}
impl From<PcBoardSer> for PcBoard {
    fn from(val: PcBoardSer) -> Self {
        let val = val.0.map(|x| x as u16);
        let mut rows = [0; 4];
        rows[0] = val[0] >> 0 | (val[1] & 0b11) << 8;
        rows[1] = val[1] >> 2 | (val[2] & 0b1111) << 6;
        rows[2] = val[2] >> 4 | (val[3] & 0b111111) << 4;
        rows[3] = val[3] >> 6 | (val[4] & 0b11111111) << 2;
        PcBoard(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_board_ser_to_u64_works() {
        for i in 0..10000 {
            let board = PcBoardSer::from_u64(i);
            let num = board.to_u64();
            assert_eq!(i, num);
        }
    }
}

// #[derive(Debug, Clone)]
// // Important! FRAGMENTS.perms.len() must be small enough to fit within a u16
// // That means you cannot use MOVES_4F as fragments
// pub struct PcChildSer(PcBoardSer, u16);

// #[derive(Debug, Clone)]
// pub struct GameMoveSer(u8);
// impl GameMoveSer {
//     pub fn new(val: u8) -> Self {
//         GameMoveSer(val)
//     }
// }
// impl From<GameMove> for GameMoveSer {
//     fn from(value: GameMove) -> Self {
//         let num = match value {
//             GameMove::ShiftLeft => 0,
//             GameMove::ShiftRight => 1,
//             GameMove::RotateCW => 2,
//             GameMove::Rotate180 => 3,
//             GameMove::RotateCCW => 4,
//             GameMove::SoftDrop => 5,
//             GameMove::Hold => 6,
//             GameMove::HardDrop => 7,
//         };
//         GameMoveSer(num)
//     }
// }
// impl TryFrom<GameMoveSer> for GameMove {
//     type Error = GenericErr;
//     fn try_from(value: GameMoveSer) -> Result<Self, Self::Error> {
//         match value.0 {
//             0 => Ok(GameMove::ShiftLeft),
//             1 => Ok(GameMove::ShiftRight),
//             2 => Ok(GameMove::RotateCW),
//             3 => Ok(GameMove::Rotate180),
//             4 => Ok(GameMove::RotateCCW),
//             5 => Ok(GameMove::SoftDrop),
//             6 => Ok(GameMove::Hold),
//             7 => Ok(GameMove::HardDrop),
//             value => Err(format!("Invalid GameMoveSer value: {}", value).into()),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct PieceTypeSer(u8);
// impl PieceTypeSer {
//     pub fn new(val: u8) -> Self {
//         PieceTypeSer(val)
//     }
// }
// impl From<PieceType> for PieceTypeSer {
//     fn from(value: PieceType) -> Self {
//         let num = match value {
//             PieceType::O => 0,
//             PieceType::I => 1,
//             PieceType::T => 2,
//             PieceType::L => 3,
//             PieceType::J => 4,
//             PieceType::S => 5,
//             PieceType::Z => 6,
//         };
//         PieceTypeSer(num)
//     }
// }
// impl TryFrom<PieceTypeSer> for PieceType {
//     type Error = GenericErr;

//     fn try_from(value: PieceTypeSer) -> Result<Self, Self::Error> {
//         match value.0 {
//             0 => Ok(PieceType::O),
//             1 => Ok(PieceType::I),
//             2 => Ok(PieceType::T),
//             3 => Ok(PieceType::L),
//             4 => Ok(PieceType::J),
//             5 => Ok(PieceType::S),
//             6 => Ok(PieceType::Z),
//             value => Err(format!("Unexpected PieceTypeSer value: {}", value).into()),
//         }
//     }
// }

// impl PcGraph {
//     pub fn serialize(&self) -> Vec<u8> {
//         // Helper Functions
//         fn write_board(board: &PcBoard, bytes: &mut Vec<u8>) {
//             bytes.extend(PcBoardSer::from(*board).0);
//         }
//         fn write_piece_type(piece_type: &PieceType, bytes: &mut Vec<u8>) {
//             bytes.push(PieceTypeSer::from(*piece_type).0);
//         }
//         fn write_moves(moves: &[GameMove], bytes: &mut Vec<u8>) {
//             let idx = FRAGMENTS_IDX.get(moves).unwrap();
//             let idx = *idx as u16;
//             write_u16(idx, bytes);
//         }
//         fn write_u8(value: u8, bytes: &mut Vec<u8>) {
//             bytes.push(value);
//         }
//         fn write_u16(value: u16, bytes: &mut Vec<u8>) {
//             bytes.extend(value.to_le_bytes());
//         }
//         fn write_u32(value: u32, bytes: &mut Vec<u8>) {
//             bytes.extend(value.to_le_bytes());
//         }

//         let mut bytes_vec = Vec::new();
//         // cuz i'm lazy
//         let bytes = &mut bytes_vec;

//         write_u32(self.graph.len() as u32, bytes);
//         for (board, piece_map) in self.graph.iter() {
//             write_board(board, bytes);
//             write_u8(piece_map.len() as u8, bytes);
//             for (piece_type, children) in piece_map.iter() {
//                 write_piece_type(piece_type, bytes);
//                 write_u16(children.len() as u16, bytes);
//                 for child in children.iter() {
//                     write_board(&child.board, bytes);
//                     write_moves(child.moves, bytes);
//                 }
//             }
//         }
//         bytes_vec
//     }
//     pub fn deserialize(value: &[u8]) -> Result<Self, GenericErr> {
//         fn read_byte(bytes: &mut impl Iterator<Item = u8>) -> Result<u8, GenericErr> {
//             match bytes.next() {
//                 Some(val) => Ok(val),
//                 None => Err("unexpected end of input".into()),
//             }
//         }
//         fn read_bytes<const N: usize>(
//             bytes: &mut impl Iterator<Item = u8>,
//         ) -> Result<[u8; N], GenericErr> {
//             let mut arr = [0; N];
//             for i in 0..N {
//                 arr[i] = read_byte(bytes)?;
//             }
//             Ok(arr)
//         }
//         fn read_u8(bytes: &mut impl Iterator<Item = u8>) -> Result<u8, GenericErr> {
//             read_byte(bytes)
//         }
//         fn read_u16(bytes: &mut impl Iterator<Item = u8>) -> Result<u16, GenericErr> {
//             let arr: [u8; 2] = read_bytes(bytes)?;
//             Ok(u16::from_le_bytes(arr))
//         }
//         fn read_u32(bytes: &mut impl Iterator<Item = u8>) -> Result<u32, GenericErr> {
//             let arr: [u8; 4] = read_bytes(bytes)?;
//             Ok(u32::from_le_bytes(arr))
//         }
//         fn read_board(bytes: &mut impl Iterator<Item = u8>) -> Result<PcBoard, GenericErr> {
//             let arr: [u8; 5] = read_bytes(bytes)?;
//             Ok(PcBoardSer::new(arr).into())
//         }
//         fn read_piece_type(bytes: &mut impl Iterator<Item = u8>) -> Result<PieceType, GenericErr> {
//             let val = read_byte(bytes)?;
//             PieceTypeSer::new(val).try_into()
//         }
//         fn read_moves(
//             bytes: &mut impl Iterator<Item = u8>,
//         ) -> Result<&'static [GameMove], GenericErr> {
//             let idx = read_u32(bytes)?;
//             match &FRAGMENTS.perms.get(idx as usize) {
//                 Some(moves) => Ok(moves),
//                 None => Err("moves out of bounds".into()),
//             }
//         }

//         let bytes = &mut value.iter().map(|x| *x);
//         let mut graph = HashMap::new();
//         let graph_len = read_u32(bytes)?;
//         for _ in 0..graph_len {
//             let board = read_board(bytes)?;
//             let mut piece_map = HashMap::new();
//             let piece_map_len = read_u8(bytes)?;
//             for _ in 0..piece_map_len {
//                 let piece_type = read_piece_type(bytes)?;
//                 let mut children = Vec::new();
//                 let children_len = read_u16(bytes)?;
//                 for _ in 0..children_len {
//                     let board = read_board(bytes)?;
//                     let moves = read_moves(bytes)?;
//                     let pc_child = PcChild::new(board, moves);
//                     children.push(pc_child);
//                 }
//                 if children.len() != children_len as usize {
//                     return Err("children.len() does not match children_len".into());
//                 }
//                 piece_map.insert(piece_type, children);
//             }
//             if piece_map.len() != piece_map_len as usize {
//                 return Err("piece_map.len() does not match piece_map_len".into());
//             }
//             graph.insert(board, piece_map);
//         }
//         if graph.len() != graph_len as usize {
//             return Err("graph.len() does not match graph_len".into());
//         }

//         Ok(PcGraph { graph })
//     }
//     pub fn save(&self, file_name: &str) -> Result<(), GenericErr> {
//         let mut file = std::fs::File::create(file_name)?;
//         let serialized = self.serialize();
//         file.write(&serialized)?;
//         Ok(())
//     }
//     pub fn load(file_name: &str) -> Result<Self, GenericErr> {
//         let mut file = std::fs::File::open(file_name)?;
//         let mut buffer = Vec::new();
//         file.read_to_end(&mut buffer)?;
//         PcGraph::deserialize(&buffer)
//     }
// }
