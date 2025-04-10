use crate::*;

#[derive(Debug)]
pub struct UUID(pub [u8; 16]);

impl Decode for UUID {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        println!("DEBUG Offset before parsing uuid {offset:?}");
        let mut array = [0x00; 16];
        array.copy_from_slice(&bytes[*offset..*offset + 16]);
        let uuid = UUID(array);
        *offset += 16;
        println!("DEBUG Offset after parsing uuid {offset:?}");
        uuid
    }
}

impl Encode for UUID {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.0)
    }
}

impl Size for UUID {
    fn size_in_bytes(&self) -> usize {
        16
    }
}

impl Offset for UUID {
    fn size(&self) -> usize {
        16
    }
}
