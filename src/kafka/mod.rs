use crate::{Decode, Encode};
use encode_derive::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct BaseRequest {
    pub size: i32,
    pub api_key: i16,
    pub api_versions: i16,
    pub correlation_id: i32,
}

#[derive(Debug, Encode, Decode)]
pub struct BaseResponse {
    pub size: i32,
    pub correlation_id: i32,
}

pub mod apiversions;
