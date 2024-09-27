use crate::{Board, Game, Pack, PackBuffer, PackCursor, Piece, PieceQueue, PieceType};
use anyhow::{bail, Result};
use std::{collections::HashMap, hash::Hash};

impl Pack for u64 {
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u64(*self)
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        cur.read_u64()
    }
}

impl<T> Pack for Vec<T>
where
    T: Pack,
{
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u64(self.len() as u64);
        for x in self.iter() {
            x.pack(buf);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let mut output = Vec::new();
        let len = cur.read_u64()?;
        for _ in 0..len {
            let x = T::unpack(cur)?;
            output.push(x);
        }
        Ok(output)
    }
}

impl<K, V> Pack for HashMap<K, V>
where
    K: Pack + Eq + Hash + Ord,
    V: Pack,
{
    fn pack(&self, buf: &mut PackBuffer) {
        let mut vec = self.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        buf.write_u64(self.len() as u64);
        for (k, v) in vec {
            k.pack(buf);
            v.pack(buf);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let mut hash_map = HashMap::new();
        let len = cur.read_u64()?;
        for _ in 0..len {
            let k = K::unpack(cur)?;
            let v = V::unpack(cur)?;
            hash_map.insert(k, v);
        }
        Ok(hash_map)
    }
}

impl<T1, T2> Pack for (T1, T2)
where
    T1: Pack,
    T2: Pack,
{
    fn pack(&self, buf: &mut PackBuffer) {
        self.0.pack(buf);
        self.1.pack(buf);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        Ok((T1::unpack(cur)?, T2::unpack(cur)?))
    }
}

impl Pack for PieceQueue {
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u8(self.len);
        buf.write_u64(self.queue);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let len = cur.read_u8()?;
        let queue = cur.read_u64()?;
        PieceQueue::from_parts(len, queue)
    }
}

impl Pack for PieceType {
    fn pack(&self, buf: &mut PackBuffer) {
        buf.write_u8(self.to_u8());
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        PieceType::from_u8(cur.read_u8()?)
    }
}

impl Pack for Piece {
    fn pack(&self, buf: &mut PackBuffer) {
        self.piece_type.pack(buf);
        buf.write_u8(self.rotation as u8);
        buf.write_u8(self.position_x as u8);
        buf.write_u8(self.position_y as u8);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let piece_type = PieceType::unpack(cur)?;
        let rotation = cur.read_u8()? as i8;
        let position_x = cur.read_u8()? as i8;
        let position_y = cur.read_u8()? as i8;
        Ok(Piece {
            piece_type,
            rotation,
            position_x,
            position_y,
        })
    }
}

impl Pack for Board {
    // Writes board state as 24 * 10 = 240 bits (30 bytes)
    fn pack(&self, buf: &mut PackBuffer) {
        for i in 0..6 {
            // Write 4 rows -> 5 bytes
            let mut accum: u64 = 0;
            for j in 0..4 {
                let row = self.matrix[i * 4 + j] as u64;
                accum |= row << (j * 10);
            }
            buf.write_packed(accum, 5);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let mut board = Board::new();
        for i in 0..6 {
            let accum = cur.read_packed(5)?;
            for j in 0..4 {
                let row = ((accum >> (j * 10)) & ((1 << 10) - 1)) as u16;
                board.set_row(i * 4 + j, row);
            }
        }
        Ok(board)
    }
}

impl Pack for Game {
    fn pack(&self, buf: &mut PackBuffer) {
        self.board.pack(buf);
        self.active.pack(buf);
        match self.hold {
            Some(x) => buf.write_u8(x.to_u8()),
            None => buf.write_u8(255),
        }
        self.queue.pack(buf);
        match self.can_hold {
            false => buf.write_u8(0),
            true => buf.write_u8(1),
        };
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let board = Board::unpack(cur)?;
        let active = Piece::unpack(cur)?;
        let hold = match cur.read_u8()? {
            255 => None,
            x => Some(PieceType::from_u8(x)?),
        };
        let queue = PieceQueue::unpack(cur)?;
        let can_hold = match cur.read_u8()? {
            0 => false,
            1 => true,
            x => bail!("could not unpack bool from byte {x}"),
        };
        Ok(Game {
            board,
            active,
            hold,
            queue,
            can_hold,
        })
    }
}
