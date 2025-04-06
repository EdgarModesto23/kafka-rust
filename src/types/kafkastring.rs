use crate::*;

impl Encode for String {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size: i16 = self.len() as i16;
        v.extend_from_slice(&size.encode());
        v.extend_from_slice(self.as_bytes());
        v
    }
}

impl Decode for String {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let len = i16::decode(bytes, offset);
        let value = String::from_utf8(bytes[*offset..*offset + len as usize].to_vec()).unwrap();
        *offset += len as usize;
        value
    }
}

impl Offset for String {
    fn size(&self) -> usize {
        self.len() + 2
    }
}

impl Size for String {
    fn size_in_bytes(&self) -> usize {
        self.len() + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_encode_decode() {
        let test_cases = vec![
            ("".to_string(), vec![0, 0]),
            ("hello".to_string(), vec![0, 5, 104, 101, 108, 108, 111]),
            (
                "world!".to_string(),
                vec![0, 6, 119, 111, 114, 108, 100, 33],
            ),
            (
                "Long string with spaces and special chars: !@#$%^&*()_+=-`~[]\\{}|;':\",./<>?"
                    .to_string(),
                {
                    let mut v = vec![0, 75];
                    v.extend_from_slice("Long string with spaces and special chars: !@#$%^&*()_+=-`~[]\\{}|;':\",./<>?".as_bytes());
                    v
                },
            ),
            (
                "1234567890".to_string(),
                vec![0, 10, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48],
            ),
            ("very very very very very very long string".to_string(), {
                let mut v = vec![0, 41];
                v.extend_from_slice("very very very very very very long string".as_bytes());
                v
            }),
            ("A".repeat(256), {
                let mut v = vec![1, 0];
                v.extend_from_slice("A".repeat(256).as_bytes());
                v
            }),
            ("B".repeat(i16::MAX as usize), {
                let mut v = vec![127, 255];
                v.extend_from_slice("B".repeat(i16::MAX as usize).as_bytes());
                v
            }),
        ];

        for (input, expected_encoded) in test_cases {
            println!("{input:?}");
            let encoded = input.encode();
            let mut offset = 0;
            let decoded = String::decode(&encoded, &mut offset);

            assert_eq!(decoded, input);
            assert_eq!(encoded, expected_encoded);
        }
    }

    #[test]
    fn test_string_offset_size() {
        let test_cases = vec![
            ("".to_string(), 2),
            ("hello".to_string(), 7),
            ("world!".to_string(), 8),
            ("A".repeat(100), 102),
            ("B".repeat(200), 202),
        ];

        for (input, expected_size) in test_cases {
            let string = input;
            assert_eq!(string.size(), expected_size);
            assert_eq!(string.size_in_bytes(), expected_size);
        }
    }
}
