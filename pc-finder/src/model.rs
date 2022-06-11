use base64::{CharacterSet, Config};
use common::*;
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
    convert::TryInto,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    lazy::SyncLazy,
};
use tinyvec::{ArrayVec, TinyVec};

// Fragments used for generating child PcBoards
pub static FRAGMENTS: &SyncLazy<Fragments> = &MOVES_2F;

const BASE64_CONFIG: Config = Config::new(CharacterSet::UrlSafe, true);
pub trait SerdeBytes: Sized {
    fn serialize(&self, buffer: &mut Vec<u8>);
    fn deserialize(bytes: &[u8]) -> GenericResult<Self>;
    fn b64_serialize(&self) -> String {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        base64::encode_config(buffer, BASE64_CONFIG)
    }
    fn b64_deserialize(b64: &str) -> GenericResult<Self> {
        Self::deserialize(&base64::decode_config(b64, BASE64_CONFIG)?)
    }
}

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
        for piece_type in PieceType::all() {
            let game = Game::from_parts(
                Board::from(*self),
                Piece::from(piece_type),
                None,
                &[PieceType::O],
                true,
            );
            let child_states = game.child_states(FRAGMENTS);
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
    type Error = GenericErr;

    /// Fails if the height of the board is greater than 4
    fn try_from(value: Board) -> Result<Self, Self::Error> {
        if value.matrix[4] != 0 {
            return generic_err!();
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
impl SerdeBytes for PcBoard {
    // Serializes to 5 bytes
    fn serialize(&self, buffer: &mut Vec<u8>) {
        let num = ((self.rows[0] as u64) << 0)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30);
        buffer.extend(&num.to_le_bytes()[0..5]);
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, GenericErr> {
        if bytes.len() != 5 {
            return Err(Default::default());
        }
        let mut buffer = [0; 8];
        for i in 0..5 {
            buffer[i] = bytes[i];
        }
        let num = u64::from_le_bytes(buffer);
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
    pub fn new(piece: Piece) -> GenericResult<Self> {
        piece.try_into()
    }
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.rows[y as usize] >> x & 1 == 1
    }
}
impl TryFrom<Piece> for CanPiece {
    type Error = GenericErr;

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        let bit_shape = piece.get_bit_shape(None, None);
        let (min_x, max_x, min_y, max_y) = piece.get_location_bounds(None);
        if piece.location.0 < min_x
            || piece.location.0 > max_x
            || piece.location.1 < min_y
            || piece.location.1 > max_y - 20
        {
            return Err(Default::default());
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
            .to_i8()
            .partial_cmp(&other.piece_type.to_i8())
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
impl SerdeBytes for CanPiece {
    // Serializes to 6 bytes

    fn serialize(&self, buffer: &mut Vec<u8>) {
        let num = ((self.rows[0] as u64) << 0)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30)
            + ((self.piece_type.to_i8() as u64) << 40)
            + ((self.rotation as u64) << 43);
        buffer.extend(&num.to_le_bytes()[0..6]);
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, GenericErr> {
        if bytes.len() != 6 {
            return generic_err!();
        }
        let mut buffer = [0; 8];
        for i in 0..6 {
            buffer[i] = bytes[i];
        }
        let num = u64::from_le_bytes(buffer);
        let bitmask: u64 = (1 << 10) - 1;
        let rows = [
            ((num >> 0) & bitmask) as u16,
            ((num >> 10) & bitmask) as u16,
            ((num >> 20) & bitmask) as u16,
            ((num >> 30) & bitmask) as u16,
        ];
        let piece_type = PieceType::try_from(((num >> 40) & 0b111) as i8)?;
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
impl SerdeBytes for Tess {
    // Serializes to 60 bytes (10 * 6 per CanPiece)
    fn serialize(&self, buffer: &mut Vec<u8>) {
        for piece in self.pieces {
            piece.serialize(buffer);
        }
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, GenericErr> {
        if bytes.len() != 60 {
            return generic_err!();
        }
        let mut pieces = [Default::default(); 10];
        for (piece, win) in pieces.iter_mut().zip(bytes.chunks(6)) {
            *piece = CanPiece::deserialize(win)?;
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
impl SerdeBytes for PcTableKey {
    // Serializes to 6 bytes (5 from PcBoard + 1)
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.board.serialize(buffer);
        buffer.push(self.piece.to_i8() as u8);
    }

    fn deserialize(bytes: &[u8]) -> GenericResult<Self> {
        if bytes.len() != 6 {
            return generic_err!();
        }
        let board = PcBoard::deserialize(&bytes[0..5])?;
        let piece = PieceType::try_from(bytes[5] as i8)?;
        Ok(PcTableKey { board, piece })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PcTableVal {
    board: PcBoard,
    moves: ArrayVec<[GameMove; 12]>,
}
impl PcTableVal {
    pub fn new(board: PcBoard, moves: &[GameMove]) -> Self {
        PcTableVal {
            board,
            moves: moves.into_iter().map(|&x| x).collect(),
        }
    }
}
impl SerdeBytes for PcTableVal {
    // Serializes to 10 bytes
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.board.serialize(buffer);
        let mut num: u64 = 0;
        for i in 0..12 {
            let bits = self.moves[i].to_u8() as u64;
            num |= bits << (i * 3);
        }
        buffer.extend(&num.to_le_bytes()[0..5])
    }

    fn deserialize(bytes: &[u8]) -> GenericResult<Self> {
        if bytes.len() != 10 {
            return generic_err!();
        }
        let board = PcBoard::deserialize(&bytes[0..5])?;
        let mut buffer = [0; 8];
        for i in 0..5 {
            buffer[i] = bytes[i + 5];
        }
        let num = u64::from_le_bytes(buffer);
        let mut moves = ArrayVec::new();
        const MASK: u64 = 0b111;
        for i in 0..12 {
            let bits = (num >> (i * 3)) & MASK;
            let val = GameMove::try_from(bits as u8)?;
            moves.push(val);
        }
        Ok(PcTableVal { board, moves })
    }
}

#[derive(Debug, Clone)]
pub struct PcTable {
    map: HashMap<PcTableKey, TinyVec<[PcTableVal; 16]>>,
}
impl PcTable {
    pub fn new() -> Self {
        PcTable {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: PcTableKey, val: PcTableVal) {
        match self.map.entry(key) {
            Entry::Occupied(mut o) => {
                o.get_mut().push(val);
            }
            Entry::Vacant(v) => {
                let mut arr = TinyVec::new();
                arr.push(val);
                v.insert(arr);
            }
        }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn lookup(
        &self,
        board: PcBoard,
        piece: PieceType,
    ) -> impl Iterator<Item = &PcTableVal> + '_ {
        self.map
            .get(&PcTableKey { board, piece })
            .map(|x| x.as_slice())
            .unwrap_or(&[])
            .iter()
    }
    pub fn lookup_all(&self, board: PcBoard) -> impl Iterator<Item = &PcTableVal> + '_ {
        self.lookup(board, PieceType::O)
            .chain(self.lookup(board, PieceType::I))
            .chain(self.lookup(board, PieceType::T))
            .chain(self.lookup(board, PieceType::L))
            .chain(self.lookup(board, PieceType::J))
            .chain(self.lookup(board, PieceType::S))
            .chain(self.lookup(board, PieceType::Z))
    }
}
impl SerdeBytes for PcTable {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        // Serialization format:
        // PcTable: len (u64, 4 bytes) + Entry (* len)
        // Entry: PcTableKey (6 bytes) + PcTableValList
        // PcTableValList: len (1 byte) + PcTableVal (10 bytes) (* len)
        buffer.extend((self.map.len() as u64).to_le_bytes());
        for (key, vals) in self.map.iter() {
            key.serialize(buffer);
            assert!(vals.len() < u8::MAX as usize);
            buffer.push(vals.len() as u8);
            for val in vals {
                val.serialize(buffer);
            }
        }
    }

    fn deserialize(bytes: &[u8]) -> GenericResult<Self> {
        let mut cursor = 0;
        let mut read = |amount: usize| {
            if cursor + amount > bytes.len() {
                return generic_err!("reached end of byte stream");
            }
            let slice = &bytes[cursor..][..amount];
            cursor += amount;
            Ok(slice)
        };
        let len = u64::from_le_bytes(read(4)?.try_into()?);
        let mut table = PcTable::new();
        for _ in 0..len {
            let key = PcTableKey::deserialize(read(6)?)?;
            let len = read(1)?[0];
            for _ in 0..len {
                let val = PcTableVal::deserialize(read(10)?)?;
                table.insert(key, val);
            }
        }
        if cursor != bytes.len() {
            return generic_err!("unexpected extra bytes remaining");
        }
        Ok(table)
    }
}
