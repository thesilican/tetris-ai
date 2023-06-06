use crate::{ArrDeque, Board, Game, Pack, PackBuffer, PackCursor, Piece, PieceType};
use anyhow::{bail, Result};

impl<T, const N: usize> Pack for ArrDeque<T, N>
where
    T: Pack,
{
    fn pack(&self, buf: &mut PackBuffer) {
        if N < 256 {
            buf.write_u8(self.len() as u8);
        } else {
            buf.write_u64(self.len() as u64);
        }
        for x in self.iter() {
            x.pack(buf);
        }
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let len = if N < 256 {
            cur.read_u8()? as usize
        } else {
            cur.read_u64()? as usize
        };
        let mut arr = ArrDeque::new();
        for _ in 0..len {
            let x = T::unpack(cur)?;
            arr.push_back(x);
        }
        Ok(arr)
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
        buf.write_u8(self.location.0 as u8);
        buf.write_u8(self.location.1 as u8);
    }

    fn unpack(cur: &mut PackCursor) -> Result<Self> {
        let piece_type = PieceType::unpack(cur)?;
        let rotation = cur.read_u8()? as i8;
        let location = (cur.read_u8()? as i8, cur.read_u8()? as i8);
        Ok(Piece {
            piece_type,
            rotation,
            location,
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
        let queue = ArrDeque::unpack(cur)?;
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
