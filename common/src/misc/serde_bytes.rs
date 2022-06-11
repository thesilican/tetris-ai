use base64::{CharacterSet, Config};

use crate::{generic_err, GenericResult};

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
    pub fn write_byte(&mut self, byte: u8) {
        self.buffer.push(byte);
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
    pub fn read(&mut self, amount: usize) -> GenericResult<&[u8]> {
        if self.head + amount > self.bytes.len() {
            return generic_err!("cursor read past end of bytes");
        }
        let slice = &self.bytes[self.head..][..amount];
        self.head += amount;
        Ok(slice)
    }
    pub fn read_byte(&mut self) -> GenericResult<u8> {
        let slice = self.read(1)?;
        Ok(slice[0])
    }
    pub fn read_array<const N: usize>(&mut self) -> GenericResult<[u8; N]> {
        let slice = self.read(N)?;
        Ok(slice.try_into()?)
    }
    pub fn finished(&self) -> bool {
        self.head == self.bytes.len()
    }
}

const BASE64_CONFIG: Config = Config::new(CharacterSet::UrlSafe, true);

/// A faster and more compact format for serialization
pub trait SerdeBytes: Sized {
    fn serialize(&self, buf: &mut Buffer);
    fn deserialize(cur: Cursor) -> GenericResult<Self>;
    fn base64_serialize(&self) -> String {
        let mut buf = Buffer::new();
        self.serialize(&mut buf);
        base64::encode_config(buf.read(), BASE64_CONFIG)
    }
    fn base64_deserialize(b64: &str) -> GenericResult<Self> {
        let bytes = base64::decode_config(b64, BASE64_CONFIG)?;
        Self::deserialize(Cursor::new(&bytes))
    }
}
