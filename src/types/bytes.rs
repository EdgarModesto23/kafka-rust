use std::vec;

use crate::{Decode, Encode, Offset, Size};

use super::{unsigned_varint_bytes_wide, uvarint::UVarint};

#[derive(Debug)]
pub struct ByteBuf(pub Vec<u8>, pub UVarint);

impl Decode for ByteBuf {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let slice = &bytes[*offset..];
        *offset = bytes.len();
        Self(
            Vec::from(slice),
            UVarint::new(slice.len() as u64, unsigned_varint_bytes_wide(slice.len())),
        )
    }
}

impl ByteBuf {
    pub fn empty() -> Self {
        ByteBuf(vec![], UVarint(0, 1))
    }
}

impl Encode for ByteBuf {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        v.extend(self.1.encode());
        v.extend(self.0.clone());

        v
    }
}

impl Size for ByteBuf {
    fn size_in_bytes(&self) -> usize {
        self.1 .1 + self.0.len()
    }
}

impl Offset for ByteBuf {
    fn size(&self) -> usize {
        self.1 .1 + self.0.len()
    }
}
