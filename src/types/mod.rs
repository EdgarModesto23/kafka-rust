pub mod array;
pub mod cstring;
pub mod integers;
pub mod kafkastring;
pub mod record;
pub mod uuid;
pub mod uvarint;
pub mod varint;

pub fn decode_unsigned_varint(data: &[u8], offset: &mut usize) -> (u64, usize) {
    let mut value = 0u64;
    let mut shift = 0;
    let mut i = *offset;
    let mut bytes_read = 0;

    while i < data.len() {
        let byte = data[i];
        value |= u64::from(byte & 0x7F) << shift;
        shift += 7;
        i += 1;
        bytes_read += 1;

        if byte & 0x80 == 0 {
            *offset = i;
            return (value, bytes_read);
        }

        if shift >= 64 {
            panic!("Varint is too large to decode properly");
        }
    }

    panic!("Varint is not able to be parsed");
}

pub fn decode_signed_varint(data: &[u8], offset: &mut usize) -> (i64, usize) {
    let (zigzag_value, bytes_read) = decode_unsigned_varint(data, offset);

    let original_value = zigzag_value >> 1;

    let decoded_value = if zigzag_value & 1 == 1 {
        -(original_value as i64)
    } else {
        original_value as i64
    };

    (decoded_value, bytes_read)
}

pub fn encode_unsigned_varint(mut value: u64) -> Vec<u8> {
    let mut result = Vec::new();

    while value >= 0x80 {
        result.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }

    result.push(value as u8);
    result
}

pub fn encode_signed_varint(value: i64) -> Vec<u8> {
    let zigzag_value = if value < 0 { !(value << 1) } else { value << 1 };

    encode_unsigned_varint(zigzag_value as u64)
}

pub fn signed_varint_bytes_wide(value: usize) -> usize {
    let zigzag_value = if value == 0 {
        0
    } else {
        (value << 1) ^ ((value >> (usize::BITS - 1)) as usize)
    };

    let mut bytes = 0;
    let mut temp_value = zigzag_value;

    while temp_value > 0 {
        bytes += 1;
        temp_value >>= 7;
    }

    if value == 0 {
        bytes = 1;
    }

    bytes
}

pub fn unsigned_varint_bytes_wide(value: usize) -> usize {
    if value == 0 {
        return 1; // Zero takes exactly 1 byte
    }

    let mut bytes = 0;
    let mut temp_value = value;

    while temp_value > 0 {
        bytes += 1;
        temp_value >>= 7; // Divide by 128 (or shift right by 7 bits)
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_varint_bytes_wide() {
        let test_cases = vec![
            (0usize, 1),      // 0 takes 1 byte
            (1usize, 1),      // 1 takes 1 byte
            (127usize, 1),    // 127 takes 1 byte
            (128usize, 2),    // 128 takes 2 bytes
            (1024usize, 2),   // 1024 takes 2 bytes
            (usize::MAX, 10), // Largest possible value takes 10 bytes
        ];

        for (value, expected_bytes_wide) in test_cases {
            let result = unsigned_varint_bytes_wide(value);
            assert_eq!(
                result, expected_bytes_wide,
                "Expected {} bytes, but got {} for value {}",
                expected_bytes_wide, result, value
            );
        }
    }
}
