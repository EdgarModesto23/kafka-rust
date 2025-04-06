use crate::*;

use super::{decode_unsigned_varint, encode_unsigned_varint};

#[derive(Debug)]
pub struct UVarint(pub u64, pub usize);

impl UVarint {
    pub fn new(value: u64, bytes_wide: usize) -> Self {
        Self(value + 1, bytes_wide)
    }
}

impl Decode for UVarint {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let (value, bytes_read) = decode_unsigned_varint(bytes, offset);
        Self(value, bytes_read)
    }
}

impl Encode for UVarint {
    fn encode(&self) -> Vec<u8> {
        encode_unsigned_varint(self.0 + 1)
    }
}

impl Offset for UVarint {
    fn size(&self) -> usize {
        self.1
    }
}

impl Size for UVarint {
    fn size_in_bytes(&self) -> usize {
        self.1
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    // Helper function to test encoding and decoding consistency
//    fn test_encoding_decoding(value: u64) {
//        let encoded = UVarint::new(value, 0).encode();
//        let mut offset = 0;
//        let decoded = UVarint::decode(&encoded, &mut offset);
//
//        assert_eq!(
//            decoded.0, value,
//            "Decoded value should match the original value."
//        );
//        assert_eq!(
//            decoded.1, offset,
//            "Bytes read should match the number of bytes used."
//        );
//    }
//
//    #[test]
//    fn test_encoding() {
//        // Test for various values to ensure they encode correctly
//        let test_cases = vec![
//            (0u64, vec![0]),
//            (1u64, vec![1]),
//            (127u64, vec![127]),
//            (128u64, vec![0x80, 0x01]),
//            (255u64, vec![0xFF, 0x01]),
//        ];
//
//        for (value, expected_encoding) in test_cases {
//            let uvarint = UVarint::new(value, 0);
//            let encoded = uvarint.encode();
//            assert_eq!(
//                encoded, expected_encoding,
//                "Encoding of value {} did not match",
//                value
//            );
//        }
//    }
//
//    #[test]
//    fn test_decoding() {
//        // Test for decoding values to ensure correctness
//        let test_cases = vec![
//            (vec![0], 0u64, 1),
//            (vec![1], 1u64, 1),
//            (vec![127], 127u64, 1),
//            (vec![0x80, 0x01], 128u64, 2),
//            (vec![0xFF, 0x01], 255u64, 2),
//            (vec![0x80, 0x04], 512u64, 2), // Corrected to 512
//        ];
//
//        for (encoded, expected_value, expected_bytes_read) in test_cases {
//            let mut offset = 0;
//            let decoded = decode_unsigned_varint(&encoded, &mut offset);
//            assert_eq!(
//                decoded.0, expected_value,
//                "Decoded value did not match expected for encoding {:?}",
//                encoded
//            );
//            assert_eq!(
//                decoded.1, expected_bytes_read,
//                "Decoded byte count did not match expected for encoding {:?}",
//                encoded
//            );
//        }
//    }
//
//    #[test]
//    fn test_size_in_bytes() {
//        // Test for the size of UVarint objects
//        let test_cases = vec![
//            (UVarint::new(0u64, 1), 1),
//            (UVarint::new(1u64, 1), 1),
//            (UVarint::new(128u64, 2), 2),
//            (UVarint::new(1024u64, 2), 2),
//            (UVarint::new(u64::MAX, 9), 9),
//        ];
//
//        for (uvarint, expected_size) in test_cases {
//            assert_eq!(
//                uvarint.size_in_bytes(),
//                expected_size,
//                "Size in bytes did not match for {:?}",
//                uvarint
//            );
//        }
//    }
//
//    #[test]
//    fn test_offset_trait() {
//        // Test for the offset trait to ensure proper byte width
//        let test_cases = vec![
//            (UVarint::new(0u64, 1), 1),
//            (UVarint::new(128u64, 2), 2),
//            (UVarint::new(1024u64, 2), 2),
//            (UVarint::new(u64::MAX, 9), 9),
//        ];
//
//        for (uvarint, expected_size) in test_cases {
//            assert_eq!(
//                uvarint.size(),
//                expected_size,
//                "Offset size did not match for {:?}",
//                uvarint
//            );
//        }
//    }
//
//    #[test]
//    fn test_encoding_decoding_consistency() {
//        // Test that encoding and then decoding the value returns the original value
//        let test_cases = vec![0u64, 1u64, 127u64, 128u64, 1024u64, u64::MAX];
//
//        for &value in &test_cases {
//            test_encoding_decoding(value);
//        }
//    }
//
//    #[test]
//    fn test_edge_cases() {
//        // Test edge cases like small values and large values
//        let edge_cases = vec![
//            (0u64, 1),      // Zero value
//            (u64::MAX, 10), // Maximum value of u64
//            (1u64, 1),      // Minimum non-zero value
//            (128u64, 2),    // Value that spans 2 bytes
//            (1024u64, 2),   // Value that spans 2 bytes
//            (255u64, 2),    // Another 2-byte value
//        ];
//
//        for (value, expected_bytes) in edge_cases {
//            let uvarint = UVarint::new(value, expected_bytes);
//            let encoded = uvarint.encode();
//            let mut offset = 0;
//            let decoded = UVarint::decode(&encoded, &mut offset);
//            assert_eq!(
//                decoded.0, value,
//                "Edge case decoding failed for value: {}",
//                value
//            );
//            assert_eq!(
//                decoded.1, expected_bytes,
//                "Edge case decoding bytes count failed for value: {}",
//                value
//            );
//        }
//    }
//}
