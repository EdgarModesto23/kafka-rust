use crate::*;

use super::uvarint::UVarint;

#[derive(Debug, PartialEq, Clone)]
pub struct CString(pub String, pub usize);

impl Decode for CString {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let len = UVarint::decode(bytes, offset);
        let value =
            String::from_utf8(bytes[*offset..*offset + (len.0 - 1) as usize].to_vec()).unwrap();
        *offset += value.len();

        Self(value, len.1)
    }
}

impl Encode for CString {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size = UVarint::new((self.0.len() - 1) as u64, self.1);
        v.extend_from_slice(&size.encode());
        v.extend_from_slice(self.0.as_bytes());
        v
    }
}

impl Offset for CString {
    fn size(&self) -> usize {
        self.1 + self.0.len()
    }
}

impl Size for CString {
    fn size_in_bytes(&self) -> usize {
        self.1 + self.0.len()
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_cstring_offset_size() {
//        let test_cases = vec![
//            ("".to_string(), 1, 1),
//            ("hello".to_string(), 6, 1),
//            ("world!".to_string(), 7, 1),
//            ("A".repeat(128), 130, 2),
//        ];
//
//        for (input, expected_size, bytes_wide) in test_cases {
//            let cstring = CString(input, bytes_wide);
//            assert_eq!(cstring.size(), expected_size);
//            assert_eq!(cstring.size_in_bytes(), expected_size);
//            assert_eq!(cstring.1, bytes_wide);
//        }
//    }
//
//    #[test]
//    fn test_decode_encode_cstring() {
//        let test_cases = vec![
//            "hey que pasa chavalones".to_string(),
//            "heyheyheyhey".to_string(),
//            "Long string with spaces and special chars: !@#$%^&*()_+=-`~[]\\{}|;':\",./<>?"
//                .to_string(),
//        ];
//
//        for value in test_cases {
//            let encoded = CString(value.clone(), value.len());
//
//            let encoded_value = CString::encode(&encoded);
//
//            let decoded = CString::decode(&encoded_value[..], &mut 0);
//
//            assert_eq!(decoded.0, value);
//        }
//    }
//
//    #[test]
//    fn test_decode_from_byte_stream() {
//        let test_cases = vec![
//            (vec![0], "".to_string(), 1),
//            (vec![5, 104, 101, 108, 108, 111], "hello".to_string(), 6),
//            (
//                vec![6, 119, 111, 114, 108, 100, 33],
//                "world!".to_string(),
//                7,
//            ),
//            (
//                {
//                    let mut v = vec![128, 1];
//                    v.extend(vec![65; 128]);
//                    v
//                },
//                "A".repeat(128),
//                130,
//            ),
//        ];
//
//        for (bytes, expected_string, expected_size) in test_cases {
//            let mut offset = 0;
//            let decoded = CString::decode(&bytes, &mut offset);
//
//            assert_eq!(decoded.0, expected_string);
//            assert_eq!(decoded.size(), expected_size);
//            assert_eq!(decoded.size_in_bytes(), expected_size);
//        }
//    }
//}
