use std::fmt::Debug;

use crate::*;

use super::{unsigned_varint_bytes_wide, uvarint::UVarint};

pub struct CVec<T>
where
    T: Encode + Decode + Debug,
{
    pub data: Vec<T>,
}

impl<T> Encode for CVec<T>
where
    T: Encode + Debug + Decode,
{
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size = UVarint::new(
            (self.data.len() - 1) as u64,
            unsigned_varint_bytes_wide(self.data.len()),
        );

        v.extend_from_slice(&size.encode());

        for value in &self.data {
            v.extend_from_slice(&value.encode());
        }
        v
    }
}

impl<T> Decode for CVec<T>
where
    T: Decode + Encode + Debug,
{
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let size = UVarint::decode(bytes, offset);

        let data = (0..size.0 - 1).map(|_| T::decode(bytes, offset)).collect();

        Self { data }
    }
}

impl<T> Offset for CVec<T>
where
    T: Decode + Encode + Debug,
{
    fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Size for CVec<T>
where
    T: Decode + Encode + Debug + Size,
{
    fn size_in_bytes(&self) -> usize {
        let size_prefix = unsigned_varint_bytes_wide(self.data.len() + 1);
        let elements_size: usize = self.data.iter().map(|e| e.size_in_bytes()).sum();

        size_prefix + elements_size
    }
}
