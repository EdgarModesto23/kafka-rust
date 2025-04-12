use crate::*;

impl Encode for i8 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl Decode for i8 {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0u8; 1];
        array.copy_from_slice(&bytes[*offset..*offset + 1]);
        *offset += 1;
        let r = i8::from_be_bytes(array);
        r
    }
}

impl Offset for i8 {
    fn size(&self) -> usize {
        std::mem::size_of::<i8>()
    }
}

impl Size for i8 {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<i8>()
    }
}

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
        let r = u8::from_be_bytes(array);
        r
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
        let r = i16::from_be_bytes(array);
        r
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
        let r = i32::from_be_bytes(array);
        r
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

impl Encode for i64 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl Decode for i64 {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes[*offset..*offset + 8]);
        *offset += 8;
        let r = i64::from_be_bytes(array);
        r
    }
}

impl Offset for i64 {
    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl Size for i64 {
    fn size_in_bytes(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}
