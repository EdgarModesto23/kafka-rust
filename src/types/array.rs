use std::{fmt::Debug, u8};

use crate::*;

use super::{
    signed_varint_bytes_wide, unsigned_varint_bytes_wide, uvarint::UVarint, varint::Varint,
};

#[derive(Debug)]
pub struct CSignedVec<T>
where
    T: Encode + Decode + Debug,
{
    pub data: Vec<T>,
}

impl<T> Encode for CSignedVec<T>
where
    T: Encode + Debug + Decode,
{
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        if self.data.is_empty() {
            return vec![0];
        }

        let size = Varint::new(self.data.len() as i64);

        v.extend_from_slice(&size.encode());

        for value in &self.data {
            v.extend_from_slice(&value.encode());
        }
        v
    }
}

impl<T> Decode for CSignedVec<T>
where
    T: Decode + Encode + Debug,
{
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let size = Varint::decode(bytes, offset);

        if size.0 == 0 || size.0 == -1 {
            return Self { data: vec![] };
        }

        let data = (0..size.0).map(|_| T::decode(bytes, offset)).collect();

        Self { data }
    }
}

impl<T> Offset for CSignedVec<T>
where
    T: Decode + Encode + Debug,
{
    fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Size for CSignedVec<T>
where
    T: Decode + Encode + Debug + Size,
{
    fn size_in_bytes(&self) -> usize {
        let size_prefix = signed_varint_bytes_wide(self.data.len());
        let elements_size: usize = self.data.iter().map(|e| e.size_in_bytes()).sum();

        size_prefix + elements_size
    }
}

impl<T> From<Vec<T>> for CVec<T>
where
    T: Encode + Decode + Debug,
{
    fn from(vec: Vec<T>) -> Self {
        CVec { data: vec }
    }
}

#[derive(Debug, Clone)]
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

        if self.data.is_empty() {
            return vec![0];
        }

        println!("Data on CVec: {:?}", self.data);

        let size = UVarint::new(
            (self.data.len() - 1) as u64,
            unsigned_varint_bytes_wide(self.data.len()),
        );
        println!("Size {:?}", size);

        v.extend_from_slice(&size.encode());

        println!("On buffer: {v:?}");

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

        if size.0 == 0 {
            *offset += 1;
            return Self { data: vec![] };
        }

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

impl<T> Encode for Vec<T>
where
    T: Encode + Debug,
{
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size: i32 = self.len() as i32;

        if size == 0 {
            return vec![u8::MAX];
        }

        println!("Value for vec: {:?}", self);

        v.extend_from_slice(&size.encode());

        for value in self {
            v.extend_from_slice(&value.encode());
        }
        v
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode + Debug,
{
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let size = i32::decode(bytes, offset);

        if size == -1 || size == 0 {
            *offset += 1;
            return vec![];
        }

        let r = (0..size).map(|_| T::decode(bytes, offset)).collect();
        r
    }
}

impl<T> Offset for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for Vec<T>
where
    T: Size,
{
    fn size_in_bytes(&self) -> usize {
        let size_prefix = std::mem::size_of::<i32>();
        let elements_size: usize = self.iter().map(|e| e.size_in_bytes()).sum();

        size_prefix + elements_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvec_encode() {
        let data = CVec {
            data: vec![0x01 as u8, 0x02 as u8, 0x03 as u8],
        };
        let encoded = data.encode();

        let expected: Vec<u8> = vec![4, 1, 2, 3];

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_cvec_decode() {
        let bytes: &[u8] = &[4, 1, 2, 3];
        let mut offset = 0;

        let decoded: CVec<u8> = CVec::decode(bytes, &mut offset);

        let expected = CVec {
            data: vec![1, 2, 3],
        };

        assert_eq!(decoded.data, expected.data);
        assert_eq!(offset, bytes.len());
    }

    #[test]
    fn test_cvec_size_in_bytes() {
        let data = CVec {
            data: vec![1, 2, 3],
        };

        assert_eq!(data.size_in_bytes(), 13);
    }

    #[test]
    fn test_empty_cvec_encode_decode() {
        let data = CVec::<u8> { data: Vec::new() };
        let encoded = data.encode();

        let expected: Vec<u8> = vec![0];

        assert_eq!(encoded, expected);

        let mut offset = 0;
        let decoded: CVec<u8> = CVec::decode(&encoded, &mut offset);

        assert_eq!(decoded.data, Vec::<u8>::new());
        assert_eq!(offset, encoded.len() + 1);
    }

    #[test]
    fn test_empty_cvec_size_in_bytes() {
        let data = CVec::<u8> { data: Vec::new() };

        assert_eq!(data.size_in_bytes(), 1);
    }

    #[test]
    fn test_vec_encode() {
        let data = vec![1 as u8, 2 as u8, 3 as u8];
        let encoded = data.encode();

        let mut expected: Vec<u8> = vec![];
        expected.extend_from_slice(&3i32.encode());
        expected.extend_from_slice(&[1, 2, 3]);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_vec_encode() {
        let data: Vec<u8> = Vec::new();
        let encoded = data.encode();

        let expected: Vec<u8> = vec![255];

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_vec_decode() {
        let bytes: &[u8] = &[0, 0, 0, 3 as u8, 1 as u8, 2 as u8, 3 as u8];
        let mut offset = 0;

        let decoded: Vec<u8> = Vec::decode(bytes, &mut offset);

        let expected = vec![1, 2, 3];

        assert_eq!(decoded, expected);
        assert_eq!(offset, bytes.len());
    }

    #[test]
    fn test_empty_vec_decode() {
        let bytes: &[u8] = &[255, 255, 255, 255];
        let mut offset = 0;

        let decoded: Vec<u8> = Vec::decode(bytes, &mut offset);

        let expected: Vec<u8> = Vec::new();

        assert_eq!(decoded, expected);
        assert_eq!(offset, bytes.len() + 1);
    }

    #[test]
    fn test_vec_size_in_bytes() {
        let data = vec![1, 2, 3];

        let expected_size = std::mem::size_of::<i32>() + (3 * std::mem::size_of::<i32>());
        assert_eq!(data.size_in_bytes(), expected_size);
    }

    #[test]
    fn test_empty_vec_size_in_bytes() {
        let data: Vec<u8> = Vec::new();

        let expected_size = std::mem::size_of::<i32>();
        assert_eq!(data.size_in_bytes(), expected_size);
    }
}
