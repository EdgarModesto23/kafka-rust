use crate::*;

use super::{decode_signed_varint, encode_signed_varint};

#[derive(Debug)]
pub struct Varint(pub i64);

impl Varint {
    pub fn get_size(&self) -> usize {
        let v = encode_signed_varint(self.0);

        v.len()
    }
    pub fn new(value: i64) -> Self {
        Self(value + 1)
    }
}

impl Encode for Varint {
    fn encode(&self) -> Vec<u8> {
        encode_signed_varint(self.0)
    }
}

impl Decode for Varint {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let (value, _) = decode_signed_varint(bytes, offset);

        Varint(value)
    }
}

impl Offset for Varint {
    fn size(&self) -> usize {
        self.get_size()
    }
}

impl Size for Varint {
    fn size_in_bytes(&self) -> usize {
        self.get_size()
    }
}
