use crate::{
    types::{array::CVec, cstring::CString},
    Decode, Encode, Size,
};
use anyhow::Error;
use encode_derive::{Decode, Size};

use super::{BaseRequestV2, BaseResponse, BaseResponseV1};

//pub struct TopicFetch {
//    pub topic_id:
//}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchRequest {
    pub basev2: BaseRequestV2,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub session_id: i32,
    pub session_epoch: i32,
    pub topics: CVec<u8>,
    pub forgotten_topics_data: CVec<u8>,
    pub rack_id: CString,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchResponse {
    pub basev1: BaseResponseV1,
    pub throttle_time: i32,
    pub error_code: i16,
    pub session_id: i32,
    pub responses: CVec<u8>,
    pub tagged_field: u8,
}

impl FetchRequest {
    pub async fn handle_request(&self) -> Result<FetchResponse, Error> {
        let base = BaseResponse {
            size: 0,
            correlation_id: self.basev2.correlation_id,
        };
        let tag_buffer = 0;
        let basev1 = BaseResponseV1 { base, tag_buffer };
        let mut response = FetchResponse {
            basev1,
            throttle_time: 0,
            error_code: 0,
            session_id: self.session_id,
            responses: CVec { data: vec![] },
            tagged_field: tag_buffer,
        };

        let res_size = response.size_in_bytes() - 4;

        response.basev1.base.size = res_size as i32;

        Ok(response)
    }
}
