use anyhow::{anyhow, Error, Result};
use libtetris::*;
use std::{
    collections::HashMap,
    convert::TryInto,
    fmt::{self, Display, Formatter},
};
use tinyvec::TinyVec;

/// Represents the bottom 4 rows of a tetris board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn intersects(&self, piece: &NormPiece) -> bool {
        self.rows
            .iter()
            .zip(piece.rows.iter())
            .any(|(&a, &b)| a & b != 0)
    }

    #[inline]
    pub fn lock(&mut self, piece: &NormPiece) {
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
            return Err(anyhow!("board exceeds max height of 4"));
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
        let num = (self.rows[0] as u64)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30);
        buf.write_packed(num, 5);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let num = cur.read_packed(5)?;
        let bitmask: u64 = (1 << 10) - 1;
        let rows = [
            (num & bitmask) as u16,
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

/// Normalized representation of a piece that has been placed on a PcBoard
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormPiece {
    pub piece_type: PieceType,
    pub rows: [u16; 4],
}

impl NormPiece {
    pub fn new(piece: Piece) -> Result<Self> {
        piece.try_into()
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        self.rows[y as usize] >> x & 1 == 1
    }
}

impl TryFrom<Piece> for NormPiece {
    type Error = Error;

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        let (min_x, max_x, min_y, max_y) =
            PieceInfo::location_bound(piece.piece_type, piece.rotation);
        if piece.position_x < min_x
            || piece.position_x > max_x
            || piece.position_y < min_y
            || piece.position_y > max_y - 20
        {
            return Err(anyhow!("piece out of bounds"));
        }

        let bit_shape = PieceInfo::bit_shape(piece.piece_type, piece.rotation, piece.position_x);
        let mut matrix = [0; 4];
        for y in 0..4 {
            let i = y - piece.position_y;
            if (0..4).contains(&i) {
                matrix[y as usize] = bit_shape[i as usize]
            }
        }
        Ok(NormPiece {
            piece_type: piece.piece_type,
            rows: matrix,
        })
    }
}

impl Display for NormPiece {
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

impl Default for NormPiece {
    fn default() -> Self {
        Piece::from_parts(PieceType::O, 0, 0, 0).try_into().unwrap()
    }
}

impl Pack for NormPiece {
    // Serialization format:
    // packed (6 bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        let num = (self.rows[0] as u64)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30)
            + ((self.piece_type.to_u8() as u64) << 40);
        buf.write_packed(num, 6);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let num = cur.read_packed(6)?;
        let bitmask: u64 = (1 << 10) - 1;
        let rows = [
            (num & bitmask) as u16,
            ((num >> 10) & bitmask) as u16,
            ((num >> 20) & bitmask) as u16,
            ((num >> 30) & bitmask) as u16,
        ];
        let piece_type = PieceType::from_u8(((num >> 40) & 0b111) as u8)?;
        Ok(NormPiece { rows, piece_type })
    }
}

/// A tesselation of the 4x10 area consisting of 10 pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tess {
    pub pieces: [NormPiece; 10],
}

impl Tess {
    pub fn new(pieces: [NormPiece; 10]) -> Self {
        // Check that the pieces are sorted
        // The exact ordering doesn't matter as long as it's consistent
        for window in pieces.windows(2) {
            assert!(window[0] < window[1]);
        }
        Tess { pieces }
    }

    pub fn contains(&self, piece: NormPiece) -> bool {
        self.pieces.contains(&piece)
    }
}

impl Pack for Tess {
    // Serialization format:
    // NormPiece (6 bytes * 10)
    fn pack(&self, buf: &mut PackBuffer) {
        for piece in self.pieces {
            piece.pack(buf);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let mut pieces = [Default::default(); 10];
        for piece in pieces.iter_mut() {
            *piece = NormPiece::unpack(cur)?;
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
                        PieceType::I => "\x1b[34m[]\x1b[0m",
                        PieceType::T => "\x1b[35m[]\x1b[0m",
                        PieceType::L => "\x1b[37m[]\x1b[0m",
                        PieceType::J => "\x1b[36m[]\x1b[0m",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PcTableKey {
    pub board: PcBoard,
    pub piece: PieceType,
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PcTableChild {
    pub board: PcBoard,
    // Tiny vec because most sequences are less than 8 long, prevents heap fragmentation
    pub actions: TinyVec<[Action; 8]>,
}

impl PcTableChild {
    pub fn new(board: PcBoard, actions: impl Into<TinyVec<[Action; 8]>>) -> Self {
        PcTableChild {
            board,
            actions: actions.into(),
        }
    }

    pub fn board(&self) -> PcBoard {
        self.board
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}

impl Pack for PcTableChild {
    // Serialization layout
    // board (5 bytes) + moves (5 bytes, packed)
    fn pack(&self, buf: &mut PackBuffer) {
        self.board.pack(buf);
        let mut num: u64 = 0;
        num |= self.actions.len() as u64;
        for (i, action) in self.actions.iter().enumerate() {
            let bits = action.to_u8() as u64;
            num |= bits << ((i * 3) + 4)
        }
        buf.write_packed(num, 5);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let board = PcBoard::unpack(cur)?;
        let num = cur.read_packed(5)?;
        let len = num & 0b1111;
        let mut actions = TinyVec::new();
        for i in 0..len {
            let bits = (num >> ((i * 3) + 4)) & 0b111;
            let val = Action::from_u8(bits as u8)?;
            actions.push(val);
        }

        Ok(PcTableChild { board, actions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PcTable {
    pub map: HashMap<PcTableKey, TinyVec<[PcTableChild; 2]>>,
}

impl PcTable {
    pub fn new() -> Self {
        PcTable {
            map: HashMap::new(),
        }
    }

    pub fn insert_child(&mut self, board: PcBoard, piece: PieceType, child: PcTableChild) {
        let key = PcTableKey { board, piece };
        let val = self.map.entry(key).or_default();
        val.push(child);
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn children(
        &self,
        board: PcBoard,
        piece: PieceType,
    ) -> impl Iterator<Item = &PcTableChild> + '_ {
        self.map
            .get(&PcTableKey { board, piece })
            .map(|x| x.as_ref())
            .unwrap_or(&[])
            .iter()
    }

    pub fn all_children(&self, board: PcBoard) -> impl Iterator<Item = &PcTableChild> + '_ {
        self.children(board, PieceType::O)
            .chain(self.children(board, PieceType::I))
            .chain(self.children(board, PieceType::T))
            .chain(self.children(board, PieceType::L))
            .chain(self.children(board, PieceType::J))
            .chain(self.children(board, PieceType::S))
            .chain(self.children(board, PieceType::Z))
    }

    pub fn load(bytes: &[u8]) -> Result<Self> {
        Self::unpack(&mut PackCursor::new(bytes))
    }
}

impl Pack for PcTable {
    // Serialization format:
    // PcTable: len (4 bytes) + Entry (* len)
    // Entry: PcTableKey (6 bytes) + PcTableVal (? bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u32(self.len() as u32);

        let mut vec: Vec<(&PcTableKey, &TinyVec<[PcTableChild; 2]>)> = self.map.iter().collect();
        // Sort so that the output is deterministic
        vec.sort_by_key(|&(key, _)| key);

        for (key, val) in vec {
            key.pack(buf);
            buf.write_u32(val.len() as u32);
            for child in val {
                child.pack(buf);
            }
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let len = cur.read_u32()?;
        let mut map = HashMap::new();
        for _ in 0..len {
            let key = PcTableKey::unpack(cur)?;
            let len = cur.read_u32()?;
            let mut val = TinyVec::<[PcTableChild; 2]>::new();
            for _ in 0..len {
                let child = PcTableChild::unpack(cur)?;
                val.push(child);
            }
            map.insert(key, val);
        }
        Ok(PcTable { map })
    }
}
