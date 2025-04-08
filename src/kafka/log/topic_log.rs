use crate::{
    types::{cstring::CString, uuid::UUID},
    Decode, Encode, Size,
};
use encode_derive::{Decode, Size};

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicRecord {
    pub name: CString,
    pub id: UUID,
    pub tagged_fields: u8,
}
