pub mod kafka;
pub mod types;

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode: Sized {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self;
}

pub trait Offset {
    fn size(&self) -> usize;
}
