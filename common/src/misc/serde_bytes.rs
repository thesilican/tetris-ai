use anyhow::{bail, Result};
use base64::{CharacterSet, Config};

pub struct Buffer {
    buffer: Vec<u8>,
}
impl Buffer {
    pub fn new() -> Self {
        Buffer { buffer: Vec::new() }
    }
    pub fn read(&self) -> &[u8] {
        &self.buffer
    }
    pub fn write(&mut self, bytes: &[u8]) {
        self.buffer.extend(bytes);
    }
    pub fn write_u8(&mut self, val: u8) {
        self.buffer.push(val);
    }
    pub fn write_u16(&mut self, val: u16) {
        self.buffer.extend(val.to_le_bytes());
    }
    pub fn write_u32(&mut self, val: u32) {
        self.buffer.extend(val.to_le_bytes());
    }
    pub fn write_u64(&mut self, val: u64) {
        self.buffer.extend(val.to_le_bytes());
    }
    pub fn write_packed(&mut self, packed: u64, len: usize) {
        self.buffer.extend(&packed.to_le_bytes()[..len])
    }
}

pub struct Cursor<'a> {
    head: usize,
    bytes: &'a [u8],
}
impl<'a> Cursor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Cursor { head: 0, bytes }
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

const BASE64_CONFIG: Config = Config::new(CharacterSet::UrlSafe, true);

/// A faster and more compact format for serialization
pub trait SerdeBytes: Sized {
    fn serialize(&self, buf: &mut Buffer);
    fn deserialize(cur: &mut Cursor) -> Result<Self>;
    fn base64_serialize(&self) -> String {
        let mut buf = Buffer::new();
        self.serialize(&mut buf);
        base64::encode_config(buf.read(), BASE64_CONFIG)
    }
    fn base64_deserialize(b64: &str) -> Result<Self> {
        let bytes = base64::decode_config(b64, BASE64_CONFIG)?;
        let mut cursor = Cursor::new(&bytes);
        let val = Self::deserialize(&mut cursor);
        if !cursor.finished() {
            bail!("expected end of cursor");
        }
        val
    }
}
