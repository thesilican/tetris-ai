use anyhow::{anyhow, Error, Result};
use common::*;
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
    convert::TryInto,
    fmt::{self, Display, Formatter},
    fs::File,
    hash::{Hash, Hasher},
    io::{Read, Write},
};
use tinyvec::{ArrayVec, TinyVec};

/// Represents the bottom 4 rows of a tetris board
/// Invariant: must be valid (see PcBoard::is_valid())
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcBoard {
    pub rows: [u16; 4],
}
impl PcBoard {
    pub const fn new() -> Self {
        PcBoard { rows: [0; 4] }
    }
    pub const fn from_rows(rows: [u16; 4]) -> Self {
        PcBoard { rows }
    }
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.rows[y as usize] >> x & 1 == 1
    }
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, on: bool) {
        if on {
            self.rows[y as usize] |= 1 << x;
        } else {
            self.rows[y as usize] &= !(1 << x);
        }
    }

    #[inline]
    pub fn count_filled_squares(&self) -> i8 {
        let mut count = 0;
        for y in 0..4 {
            for x in 0..10 {
                if self.get(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn child_boards(&self) -> Vec<PcBoard> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        for piece_type in PieceType::ALL {
            let game = Game::from_parts(
                Board::from(*self),
                Piece::from_piece_type(piece_type),
                None,
                &[PieceType::O],
                true,
            );
            let child_states = game.children().unwrap();
            for child in child_states {
                if let Ok(board) = PcBoard::try_from(child.game.board) {
                    if visited.insert(board) {
                        result.push(board);
                    }
                }
            }
        }
        result
    }

    #[inline]
    pub fn intersects(&self, piece: &CanPiece) -> bool {
        self.rows
            .iter()
            .zip(piece.rows.iter())
            .any(|(&a, &b)| a & b != 0)
    }

    #[inline]
    pub fn lock(&mut self, piece: &CanPiece) {
        for (b, p) in self.rows.iter_mut().zip(piece.rows.iter()) {
            *b |= *p;
        }
    }
}
impl TryFrom<Board> for PcBoard {
    type Error = Error;

    /// Fails if the height of the board is greater than 4
    fn try_from(value: Board) -> Result<Self> {
        if value.matrix[4] != 0 {
            return Err(anyhow!("Uh oh"));
        }
        let board = PcBoard {
            rows: value.matrix[0..4].try_into().unwrap(),
        };
        Ok(board)
    }
}
impl From<PcBoard> for Board {
    fn from(pc_board: PcBoard) -> Self {
        let mut board = Board::new();
        for (i, row) in pc_board.rows.into_iter().enumerate() {
            board.set_row(i, row);
        }
        board
    }
}
impl Display for PcBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '/' } else { '\n' };
        for y in (0..4).rev() {
            for x in 0..10 {
                let bit = if self.get(x, y) { "[]" } else { "▒▒" };
                write!(f, "{}", bit)?;
            }
            if y != 0 {
                write!(f, "{}", sep)?;
            }
        }
        Ok(())
    }
}
impl Pack for PcBoard {
    // Serialization format:
    // packed (5 bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        let num = ((self.rows[0] as u64) << 0)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30);
        buf.write_packed(num, 5);
    }
    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let num = cur.read_packed(5)?;
        let bitmask: u64 = (1 << 10) - 1;
        let rows = [
            ((num >> 0) & bitmask) as u16,
            ((num >> 10) & bitmask) as u16,
            ((num >> 20) & bitmask) as u16,
            ((num >> 30) & bitmask) as u16,
        ];
        Ok(PcBoard { rows })
    }
}
impl Default for PcBoard {
    fn default() -> Self {
        PcBoard::new()
    }
}

/// Canonical representation of a piece that has
/// been placed on a PcBoard
#[derive(Debug, Clone, Copy)]
pub struct CanPiece {
    pub piece_type: PieceType,
    pub rotation: i8,
    pub rows: [u16; 4],
}
impl CanPiece {
    pub fn new(piece: Piece) -> Result<Self> {
        piece.try_into()
    }
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.rows[y as usize] >> x & 1 == 1
    }
}
impl TryFrom<Piece> for CanPiece {
    type Error = Error;

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        let bit_shape = piece.get_bit_shape(None, None);
        let (min_x, max_x, min_y, max_y) = piece.get_location_bounds(None);
        if piece.location.0 < min_x
            || piece.location.0 > max_x
            || piece.location.1 < min_y
            || piece.location.1 > max_y - 20
        {
            return Err(anyhow!(""));
        }

        let mut matrix = [0; 4];
        for y in 0..4 {
            let i = y - piece.location.1;
            if 0 <= i && i < 4 {
                matrix[y as usize] = bit_shape[i as usize]
            }
        }
        Ok(CanPiece {
            piece_type: piece.piece_type,
            rotation: piece.rotation,
            rows: matrix,
        })
    }
}
impl Display for CanPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '/' } else { '\n' };
        for y in (0..4).rev() {
            for x in 0..10 {
                let bit = if self.get(x, y) { "[]" } else { ".." };
                write!(f, "{}", bit)?;
            }
            if y != 0 {
                write!(f, "{}", sep)?;
            }
        }
        Ok(())
    }
}
impl PartialEq for CanPiece {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
    }
}
impl Eq for CanPiece {}
impl Hash for CanPiece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
    }
}
impl PartialOrd for CanPiece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self
            .piece_type
            .to_u8()
            .partial_cmp(&other.piece_type.to_u8())
        {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.rotation.partial_cmp(&other.rotation) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.rows.partial_cmp(&other.rows)
    }
}
impl Ord for CanPiece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Default for CanPiece {
    fn default() -> Self {
        Piece::new(PieceType::O, 0, (0, 0)).try_into().unwrap()
    }
}
impl Pack for CanPiece {
    // Serialization format:
    // packed (6 bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        let num = ((self.rows[0] as u64) << 0)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30)
            + ((self.piece_type.to_u8() as u64) << 40)
            + ((self.rotation as u64) << 43);
        buf.write_packed(num, 6);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let num = cur.read_packed(6)?;
        let bitmask: u64 = (1 << 10) - 1;
        let rows = [
            ((num >> 0) & bitmask) as u16,
            ((num >> 10) & bitmask) as u16,
            ((num >> 20) & bitmask) as u16,
            ((num >> 30) & bitmask) as u16,
        ];
        let piece_type = PieceType::from_u8(((num >> 40) & 0b111) as u8)?;
        let rotation = ((num >> 43) & 0b111) as i8;
        Ok(CanPiece {
            rows,
            piece_type,
            rotation,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tess {
    pub pieces: [CanPiece; 10],
}
impl Tess {
    pub fn new(pieces: [CanPiece; 10]) -> Self {
        assert!(pieces.is_sorted());
        Tess { pieces }
    }

    pub fn contains(&self, piece: CanPiece) -> bool {
        self.pieces.contains(&piece)
    }
}
impl Pack for Tess {
    // Serialization format:
    // CanPiece (6 bytes * 10)
    fn pack(&self, buf: &mut PackBuffer) {
        for piece in self.pieces {
            piece.pack(buf);
        }
    }
    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let mut pieces = [Default::default(); 10];
        for piece in pieces.iter_mut() {
            *piece = CanPiece::unpack(cur)?;
        }
        Ok(Tess { pieces })
    }
}
impl Display for Tess {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in (0..4).rev() {
            for x in 0..10 {
                for p in self.pieces {
                    let text = match p.piece_type {
                        PieceType::O => "\x1b[33m[]\x1b[0m",
                        PieceType::I => "\x1b[36m[]\x1b[0m",
                        PieceType::T => "\x1b[37m[]\x1b[0m",
                        PieceType::L => "\x1b[30m[]\x1b[0m",
                        PieceType::J => "\x1b[34m[]\x1b[0m",
                        PieceType::S => "\x1b[32m[]\x1b[0m",
                        PieceType::Z => "\x1b[31m[]\x1b[0m",
                    };
                    if p.get(x, y) {
                        write!(f, "{}", text)?;
                        break;
                    }
                }
            }
            if y != 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcTableKey {
    board: PcBoard,
    piece: PieceType,
}
impl PcTableKey {
    pub fn new(board: PcBoard, piece: PieceType) -> Self {
        PcTableKey { board, piece }
    }
}
impl Pack for PcTableKey {
    // Serialization format
    // board (5 bytes, packed) + piece (1 byte)
    fn pack(&self, buf: &mut PackBuffer) {
        self.board.pack(buf);
        buf.write_u8(self.piece.to_u8());
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let board = PcBoard::unpack(cur)?;
        let piece = PieceType::from_u8(cur.read_u8()?)?;
        Ok(PcTableKey { board, piece })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PcTableLeaf {
    board: PcBoard,
    moves: ArrayVec<[GameMove; 12]>,
}
impl PcTableLeaf {
    pub fn new(board: PcBoard, moves: &[GameMove]) -> Self {
        PcTableLeaf {
            board,
            moves: moves.into_iter().map(|&x| x).collect(),
        }
    }
    pub fn board(&self) -> PcBoard {
        self.board
    }
    pub fn moves(&self) -> &[GameMove] {
        &self.moves
    }
}
impl Pack for PcTableLeaf {
    // Serialization layout
    // board (5 bytes) + moves (5 bytes, packed)
    fn pack(&self, buf: &mut PackBuffer) {
        self.board.pack(buf);
        let mut num: u64 = 0;
        num |= self.moves.len() as u64;
        for (i, game_move) in self.moves.iter().enumerate() {
            let bits = game_move.to_u8() as u64;
            num |= bits << ((i * 3) + 4)
        }
        buf.write_packed(num, 5);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let board = PcBoard::unpack(cur)?;
        let num = cur.read_packed(5)?;
        let len = num & 0b1111;
        let mut moves = ArrayVec::new();
        for i in 0..len {
            let bits = (num >> ((i * 3) + 4)) & 0b111;
            let val = GameMove::from_u8(bits as u8)?;
            moves.push(val);
        }
        Ok(PcTableLeaf { board, moves })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcTableVal {
    leaves: TinyVec<[PcTableLeaf; 2]>,
}
impl PcTableVal {
    pub fn new(leaves: &[PcTableLeaf]) -> Self {
        PcTableVal {
            leaves: leaves.into_iter().map(|&x| x).collect(),
        }
    }
    pub fn leaves(&self) -> &[PcTableLeaf] {
        &self.leaves
    }
}
impl Pack for PcTableVal {
    // Serialization layout
    // len (1 byte) + leaves (10 bytes * len)
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u8(self.leaves.len() as u8);
        for leaf in &self.leaves {
            leaf.pack(buf);
        }
    }
    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let len = cur.read_u8()?;
        let mut leaves = TinyVec::new();
        for _ in 0..len {
            let leaf = PcTableLeaf::unpack(cur)?;
            leaves.push(leaf);
        }
        Ok(PcTableVal { leaves })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcTable {
    map: HashMap<PcTableKey, PcTableVal>,
}
impl PcTable {
    pub fn new() -> Self {
        PcTable {
            map: HashMap::new(),
        }
    }
    pub fn with_map(map: HashMap<PcTableKey, PcTableVal>) -> Self {
        PcTable { map }
    }
    pub fn map(&self) -> &HashMap<PcTableKey, PcTableVal> {
        &self.map
    }
    pub fn insert_leaf(&mut self, key: PcTableKey, leaf: PcTableLeaf) {
        match self.map.entry(key) {
            Entry::Occupied(mut o) => {
                o.get_mut().leaves.push(leaf);
            }
            Entry::Vacant(v) => {
                v.insert(PcTableVal::new(&[leaf]));
            }
        }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    #[inline]
    pub fn leaves<'a>(
        &'a self,
        board: PcBoard,
        piece: PieceType,
    ) -> impl Iterator<Item = &'a PcTableLeaf> + 'a {
        self.map
            .get(&PcTableKey { board, piece })
            .map(|x| &*x.leaves)
            .unwrap_or(&[])
            .iter()
    }
    pub fn leaves_all(&self, board: PcBoard) -> impl Iterator<Item = &PcTableLeaf> + '_ {
        self.leaves(board, PieceType::O)
            .chain(self.leaves(board, PieceType::I))
            .chain(self.leaves(board, PieceType::T))
            .chain(self.leaves(board, PieceType::L))
            .chain(self.leaves(board, PieceType::J))
            .chain(self.leaves(board, PieceType::S))
            .chain(self.leaves(board, PieceType::Z))
    }
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        println!("Saving pc-table to {}", filename);
        let mut buf = PackBuffer::new();
        self.pack(&mut buf);
        let mut file = File::create(filename)?;
        file.write_all(buf.read())?;
        Ok(())
    }
    pub fn load_from_file(filename: &str) -> Result<Self> {
        println!("Loading pc-table from {}", filename);
        let mut file = File::open(filename)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Self::unpack(&mut PackCursor::new(&buf))
    }
    pub fn load_static() -> Self {
        let bytes = &*include_bytes!("../data/pc-table.bin");
        Self::unpack(&mut PackCursor::new(bytes)).unwrap()
    }
}
impl Pack for PcTable {
    // Serialization format:
    // PcTable: len (4 bytes) + Entry (* len)
    // Entry: PcTableKey (6 bytes) + PcTableVal (? bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u32(self.len() as u32);
        for (key, val) in self.map.iter() {
            key.pack(buf);
            val.pack(buf);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let len = cur.read_u32()?;
        let mut map = HashMap::new();
        for _ in 0..len {
            let key = PcTableKey::unpack(cur)?;
            let val = PcTableVal::unpack(cur)?;
            map.insert(key, val);
        }
        Ok(PcTable { map })
    }
}
