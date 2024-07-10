use crate::PieceType;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub const PIECE_QUEUE_MAX_LEN: usize = 21;

/// A bitwise queue of piece types, the 3i..3i+2 LSB represent the bit value
/// for the piece type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "crate::serde::SerializedPieceQueue")]
#[serde(try_from = "crate::serde::SerializedPieceQueue")]
pub struct PieceQueue {
    pub(crate) len: u8,
    pub(crate) queue: u64,
}

impl PieceQueue {
    pub fn new() -> Self {
        PieceQueue { len: 0, queue: 0 }
    }

    pub fn from_parts(len: u8, queue: u64) -> Result<Self> {
        if len > PIECE_QUEUE_MAX_LEN as u8 {
            bail!("queue is too long");
        }

        for i in 0..len {
            let bits = (queue >> (i * 3)) as u8;
            if PieceType::from_u8(bits).is_err() {
                bail!("encountered invalid piece bits");
            }
        }

        let mut bitmask: u64 = 0;
        for i in 0..len * 3 {
            bitmask |= 1 << i;
        }
        if queue & !bitmask != 0 {
            bail!("encountered non-zero garbage bits");
        } else {
            Ok(PieceQueue { len, queue })
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> PieceType {
        if idx > self.len() as usize {
            panic!("index out of bounds");
        }
        let bits = (self.queue >> idx * 3) & 0b111;
        PieceType::from_u8(bits as u8).unwrap()
    }

    pub fn enqueue(&mut self, piece_type: PieceType) {
        if self.len == PIECE_QUEUE_MAX_LEN as u8 {
            panic!("queue capacity full");
        }
        let bits = piece_type.to_u8() as u64;
        self.queue |= bits << self.len * 3;
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<PieceType> {
        let len = self.len();
        if len == 0 {
            return None;
        }
        let bits = (self.queue & 0b111) as u8;
        let piece_type = PieceType::from_u8(bits).unwrap();
        self.queue >>= 3;
        self.len -= 1;
        Some(piece_type)
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            idx: 0,
            queue: self,
        }
    }
}

pub struct Iter<'a> {
    idx: u8,
    queue: &'a PieceQueue,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PieceType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.queue.len {
            None
        } else {
            let piece_type = self.queue.get(self.idx as usize);
            self.idx += 1;
            Some(piece_type)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_piece_queue() {
        let mut queue = PieceQueue::new();
        for piece_type in PieceType::ALL {
            queue.enqueue(piece_type);
        }
        let mut iter = queue.iter().zip(PieceType::ALL);
        let mut len = 0;
        while let Some((a, b)) = iter.next() {
            assert!(a == b);
            len += 1;
        }
        assert!(len == PieceType::ALL.len());
    }
}
