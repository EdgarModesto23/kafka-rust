use crate::{Decode, Encode, Offset, Size};

#[derive(Debug)]
pub struct ByteBuf(pub Vec<u8>);

impl Decode for ByteBuf {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let slice = &bytes[*offset..];
        *offset = bytes.len();
        Self(Vec::from(slice))
    }
}

impl Encode for ByteBuf {
    fn encode(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Size for ByteBuf {
    fn size_in_bytes(&self) -> usize {
        self.0.len()
    }
}

impl Offset for ByteBuf {
    fn size(&self) -> usize {
        self.0.len()
    }
}
