use crate::{Decode, Encode, Offset};

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

// ---------------------------------------------

// ------------String----------------------------
impl Encode for String {
    fn encode(&self) -> Vec<u8> {
        let mut encoded = (self.len() as u32).to_be_bytes().to_vec();
        encoded.extend_from_slice(self.as_bytes());
        encoded
    }
}

impl Decode for String {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let len = u32::from_be_bytes(bytes[*offset..*offset + 4].try_into().unwrap()) as usize;
        *offset += 4;
        let value = String::from_utf8(bytes[*offset..*offset + len].to_vec()).unwrap();
        *offset += len;
        value
    }
}

impl Offset for String {
    fn size(&self) -> usize {
        4 + self.len()
    }
}

// ---------------------------------------------
