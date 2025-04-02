use crate::{Decode, Encode, Offset, Size};

pub fn decode_varint(data: &[u8]) -> (u64, usize) {
    let mut value = 0u64;
    let mut shift = 0;
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        value |= u64::from(byte & 0x7F) << shift;
        shift += 7;
        i += 1;

        if byte & 0x80 == 0 {
            return (value, i);
        }

        if shift >= 64 {
            panic!("Varint is not able to be parsed")
        }
    }

    panic!("Varint is not able to be parsed")
}

pub fn encode_zigzag(value: u64) -> Vec<u8> {
    let mut result = Vec::new();
    let mut value = value;

    while value >= 0x80 {
        result.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }

    result.push(value as u8);
    result
}

// ------------i32----------------------------

impl Encode for i32 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl Decode for i32 {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[*offset..*offset + 4]);
        *offset += 4;
        i32::from_be_bytes(array)
    }
}

impl Offset for i32 {
    fn size(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}

impl Size for i32 {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}

// ---------------------------------------------

// ------------i16----------------------------

impl Encode for i16 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl Decode for i16 {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0u8; 2];
        array.copy_from_slice(&bytes[*offset..*offset + 2]);
        *offset += 2;
        i16::from_be_bytes(array)
    }
}

impl Offset for i16 {
    fn size(&self) -> usize {
        std::mem::size_of::<i16>()
    }
}

impl Size for i16 {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<i16>()
    }
}

// ---------------------------------------------
//

// ------------u8----------------------------

impl Encode for u8 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl Decode for u8 {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0u8; 1];
        array.copy_from_slice(&bytes[*offset..*offset + 1]);
        *offset += 1;
        u8::from_be_bytes(array)
    }
}

impl Offset for u8 {
    fn size(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

impl Size for u8 {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

// ---------------------------------------------
//
// ------------String----------------------------
impl Encode for String {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size = Varint(self.len());
        v.extend_from_slice(&size.encode());
        v.extend_from_slice(self.as_bytes());
        v
    }
}

impl Decode for String {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let len = Varint::decode(bytes, offset).0;
        let value = String::from_utf8(bytes[*offset..*offset + len].to_vec()).unwrap();
        *offset += len;
        value
    }
}

impl Offset for String {
    fn size(&self) -> usize {
        let v = Varint(self.len());
        v.get_size() + v.0
    }
}

impl Size for String {
    fn size_in_bytes(&self) -> usize {
        let v = Varint(self.len());
        v.get_size() + v.0
    }
}

// ---------------------------------------------

// ------------Varint----------------------------

pub struct Varint(usize);

impl Varint {
    fn get_size(&self) -> usize {
        let v = encode_zigzag(self.0 as u64);

        v.len()
    }
}

impl Encode for Varint {
    fn encode(&self) -> Vec<u8> {
        encode_zigzag(self.0 as u64)
    }
}

impl Decode for Varint {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let (value, size) = decode_varint(bytes);

        *offset += size;

        Varint(value as usize)
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

// ---------------------------------------------

// ------------Vec----------------------------
impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let size = Varint(self.len() + 1);

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
        let size = Varint::decode(bytes, offset);

        (0..size.0).map(|_| T::decode(bytes, offset)).collect()
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
        let size_prefix = Varint(self.len()).size_in_bytes();
        let elements_size: usize = self.iter().map(|e| e.size_in_bytes()).sum();

        size_prefix + elements_size
    }
}

// ---------------------------------------------
