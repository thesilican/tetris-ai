use base64::{CharacterSet, Config};
use common::*;
use std::{
    cmp::Ordering,
    collections::HashSet,
    convert::TryInto,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    lazy::SyncLazy,
};

// Fragments used for generating child PcBoards
pub static FRAGMENTS: &SyncLazy<Fragments> = &MOVES_2F;

const BASE64_CONFIG: Config = Config::new(CharacterSet::UrlSafe, true);
pub trait Serializable: Sized {
    fn serialize(&self, buffer: &mut Vec<u8>);
    fn deserialize(bytes: &[u8]) -> GenericResult<Self>;
    fn serialized_len() -> usize;
    fn base64_serialize(&self) -> String {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        base64::encode_config(buffer, BASE64_CONFIG)
    }
    fn base64_deserialize(b64: &str) -> GenericResult<Self> {
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
impl Serializable for PcBoard {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        let num = ((self.rows[0] as u64) << 0)
            + ((self.rows[1] as u64) << 10)
            + ((self.rows[2] as u64) << 20)
            + ((self.rows[3] as u64) << 30);
        buffer.extend(&num.to_le_bytes()[0..5]);
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, GenericErr> {
        if bytes.len() != Self::serialized_len() {
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
    fn serialized_len() -> usize {
        5
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
impl Serializable for CanPiece {
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
        if bytes.len() != Self::serialized_len() {
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

    fn serialized_len() -> usize {
        6
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
impl Serializable for Tess {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        for piece in self.pieces {
            piece.serialize(buffer);
        }
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, GenericErr> {
        if bytes.len() != Self::serialized_len() {
            return generic_err!();
        }
        let mut pieces = [Default::default(); 10];
        for (piece, win) in pieces
            .iter_mut()
            .zip(bytes.chunks(CanPiece::serialized_len()))
        {
            *piece = CanPiece::deserialize(win)?;
        }
        Ok(Tess { pieces })
    }
    fn serialized_len() -> usize {
        CanPiece::serialized_len() * 10
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
