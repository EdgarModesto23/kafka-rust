use std::fmt::Debug;

use crate::*;

use super::{unsigned_varint_bytes_wide, uvarint::UVarint};

impl<T> Encode for Vec<T>
where
    T: Encode + Debug,
{
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        println!("{:?}", self);

        let size = UVarint::new(
            (self.len() - 1) as u64,
            unsigned_varint_bytes_wide(self.len()),
        );

        v.extend_from_slice(&size.encode());

        for value in self {
            v.extend_from_slice(&value.encode());
        }
        v
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let size = UVarint::decode(bytes, offset);

        println!("{size:?}");

        (0..size.0 - 1).map(|_| T::decode(bytes, offset)).collect()
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
        let size_prefix = unsigned_varint_bytes_wide(self.len() + 1);
        let elements_size: usize = self.iter().map(|e| e.size_in_bytes()).sum();

        size_prefix + elements_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_encode_decode_i32() {
        let test_cases = vec![
            (vec![], vec![1]),
            (vec![1i32], vec![2, 0, 0, 0, 1]),
            (
                vec![1i32, 2i32, 3i32],
                vec![4, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3],
            ),
            (
                vec![-1i32, i32::MAX, i32::MIN],
                vec![4, 255, 255, 255, 255, 127, 255, 255, 255, 128, 0, 0, 0],
            ),
        ];

        for (input, expected_encoded) in test_cases {
            let encoded = input.encode();
            let mut offset = 0;
            let decoded: Vec<i32> = Vec::decode(&encoded, &mut offset);

            assert_eq!(decoded, input);
            assert_eq!(encoded, expected_encoded);
        }
    }

    #[test]
    fn test_vec_size_in_bytes_i32() {
        let test_cases = vec![
            (vec![], 1),
            (vec![1i32], 6),
            (vec![1i32, 2i32], 10),
            (vec![1i32; 128], 513),
            (vec![1i32; 256], 1027),
        ];

        for (input, expected_size) in test_cases {
            assert_eq!(input.size_in_bytes(), expected_size);
        }
    }
}
