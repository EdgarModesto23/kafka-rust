use crate::{Decode, Encode, Size};
use encode_derive::{Decode, Size};

#[derive(Debug, Encode, Decode)]
pub struct BaseRequest {
    pub size: i32,
    pub api_key: i16,
    pub api_versions: i16,
    pub correlation_id: i32,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct BaseResponse {
    pub size: i32,
    pub correlation_id: i32,
}

pub mod apiversions;
