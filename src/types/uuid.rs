use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UUID(pub [u8; 16]);

impl Decode for UUID {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let mut array = [0x00; 16];
        array.copy_from_slice(&bytes[*offset..*offset + 16]);
        let uuid = UUID(array);
        *offset += 16;
        uuid
    }
}

impl UUID {
    pub fn to_string(&self) -> String {
        Uuid::from_bytes(self.0).to_string()
    }
}

impl Encode for UUID {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.0)
    }
}

impl Size for UUID {
    fn size_in_bytes(&self) -> usize {
        15
    }
}

impl Offset for UUID {
    fn size(&self) -> usize {
        16
    }
}
