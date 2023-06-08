mod impls;

use anyhow::{bail, Result};
use base64::{engine::general_purpose::URL_SAFE, Engine};

pub struct PackBuffer {
    buf: Vec<u8>,
}
impl PackBuffer {
    pub fn new() -> Self {
        PackBuffer { buf: Vec::new() }
    }
    pub fn read(&self) -> &[u8] {
        &self.buf
    }
    pub fn write(&mut self, bytes: &[u8]) {
        self.buf.extend(bytes);
    }
    pub fn write_u8(&mut self, val: u8) {
        self.buf.push(val);
    }
    pub fn write_u16(&mut self, val: u16) {
        self.buf.extend(val.to_le_bytes());
    }
    pub fn write_u32(&mut self, val: u32) {
        self.buf.extend(val.to_le_bytes());
    }
    pub fn write_u64(&mut self, val: u64) {
        self.buf.extend(val.to_le_bytes());
    }
    pub fn write_packed(&mut self, packed: u64, len: usize) {
        self.buf.extend(&packed.to_le_bytes()[..len])
    }
}

pub struct PackCursor<'a> {
    head: usize,
    bytes: &'a [u8],
}
impl<'a> PackCursor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        PackCursor { head: 0, bytes }
    }
    pub fn read(&mut self, amount: usize) -> Result<&[u8]> {
        if self.head + amount > self.bytes.len() {
            bail!("cursor read past end of bytes");
        }
        let slice = &self.bytes[self.head..][..amount];
        self.head += amount;
        Ok(slice)
    }
    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let slice = self.read(N)?;
        Ok(slice.try_into()?)
    }
    pub fn read_u8(&mut self) -> Result<u8> {
        let arr = self.read_array::<1>()?;
        Ok(arr[0])
    }
    pub fn read_u16(&mut self) -> Result<u16> {
        let arr = self.read_array::<2>()?;
        Ok(u16::from_le_bytes(arr))
    }
    pub fn read_u32(&mut self) -> Result<u32> {
        let arr = self.read_array::<4>()?;
        Ok(u32::from_le_bytes(arr))
    }
    pub fn read_u64(&mut self) -> Result<u64> {
        let arr = self.read_array::<8>()?;
        Ok(u64::from_le_bytes(arr))
    }
    pub fn read_packed(&mut self, len: usize) -> Result<u64> {
        let slice = self.read(len)?;
        let mut buffer = [0; 8];
        for (b, s) in buffer.iter_mut().zip(slice) {
            *b = *s;
        }
        Ok(u64::from_le_bytes(buffer))
    }
    pub fn read_all(&mut self) -> &[u8] {
        let amount = self.len();
        self.read(amount).unwrap()
    }
    pub fn len(&self) -> usize {
        self.bytes.len() - self.head
    }
    pub fn finished(&self) -> bool {
        self.len() == 0
    }
}

/// A faster and more compact format for serialization
pub trait Pack: Sized {
    fn pack(&self, buf: &mut PackBuffer);
    fn unpack(cur: &mut PackCursor) -> Result<Self>;
    fn pack_bytes(&self) -> Vec<u8> {
        let mut buf = PackBuffer::new();
        self.pack(&mut buf);
        buf.buf
    }
    fn pack_base64(&self) -> String {
        let mut buf = PackBuffer::new();
        self.pack(&mut buf);
        URL_SAFE.encode(buf.read())
    }
    fn unpack_bytes(bytes: &[u8]) -> Result<Self> {
        let mut cursor = PackCursor::new(bytes);
        let val = Self::unpack(&mut cursor);
        if !cursor.finished() {
            bail!("expected end of cursor");
        }
        val
    }
    fn unpack_base64(text: &str) -> Result<Self> {
        let bytes = URL_SAFE.decode(text)?;
        let mut cursor = PackCursor::new(&bytes);
        let val = Self::unpack(&mut cursor);
        if !cursor.finished() {
            bail!("expected end of cursor");
        }
        val
    }
}
