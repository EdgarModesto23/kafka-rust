use crate::{
    types::{cstring::CString, uuid::UUID},
    Decode, Encode, Size,
};
use anyhow::Error;
use encode_derive::{Decode, Size};

use super::{BaseRequestV2, BaseResponse, BaseResponseV1};

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicsRequest {
    pub name: CString,
    pub tag_buffer: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct DescribePartitionsRequest {
    pub basev2: BaseRequestV2,
    pub topics_array: Vec<TopicsRequest>,
    pub response_partition_limit: i32,
    pub cursor: u8,
    pub tag_buffer: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicResponse {
    pub error_code: i16,
    pub name: CString,
    pub id: UUID,
    pub is_internal: u8,
    pub partitions_array: u8,
    pub authorized_ops: i32,
    pub tag_buffer: u8,
}

impl TopicResponse {
    pub fn new(name: &CString) -> Self {
        TopicResponse {
            error_code: 3,
            name: CString(name.0.clone(), name.1),
            id: UUID([0x00; 16]),
            is_internal: 0,
            partitions_array: 1,
            authorized_ops: 0x00000df8,
            tag_buffer: 0,
        }
    }
}

#[derive(Debug, Encode, Decode, Size)]
pub struct DescribePartitionsResponse {
    pub basev1: BaseResponseV1,
    pub throttle: i32,
    pub topics_array: Vec<TopicResponse>,
    pub next_cursor: u8,
    pub tag_buffer: u8,
}

impl DescribePartitionsRequest {
    pub async fn handle_request(&self) -> Result<DescribePartitionsResponse, Error> {
        let base = BaseResponse {
            size: 0,
            correlation_id: self.basev2.correlation_id,
        };
        let mut topics_array = vec![];
        let throttle = 0;
        let next_cursor = 0xff;
        let tag_buffer = 0;

        let basev1 = BaseResponseV1 { base, tag_buffer };

        for topic in &self.topics_array {
            topics_array.push(TopicResponse::new(&topic.name));
        }

        let mut response = DescribePartitionsResponse {
            basev1,
            throttle,
            topics_array,
            next_cursor,
            tag_buffer,
        };

        let res_size = response.size_in_bytes() - 3;
        println!("{response:?}");
        response.basev1.base.size = res_size as i32;

        Ok(response)
    }
}
