use crate::{
    types::{array::CVec, cstring::CString, uuid::UUID},
    Decode, Encode, Size,
};
use anyhow::Error;
use encode_derive::{Decode, Size};

use super::{listpartitions::TopicsRequest, BaseRequestV2, BaseResponse, BaseResponseV1};

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchPartitionsRequest {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub fetch_offset: i64,
    pub log_start_offset: i64,
    pub pub_partition_max_bytes: i32,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicFetch {
    pub topic_id: UUID,
    pub partitions: CVec<FetchPartitionsRequest>,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct ForgottenTopicsData {
    pub topic_id: UUID,
    pub partitions: i32,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchRequest {
    pub basev2: BaseRequestV2,
    pub max_wait_ms: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub session_id: i32,
    pub session_epoch: i32,
    pub topics: CVec<TopicFetch>,
    pub forgotten_topics_data: CVec<ForgottenTopicsData>,
    pub rack_id: CString,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct AbortedTransactions {
    pub producer_id: i64,
    pub first_offset: i64,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchTopicResponse {
    pub topic_id: UUID,
    pub partitions: CVec<FetchPartitionsResponse>,
    pub tagged_field: u8,
}

impl FetchTopicResponse {
    pub fn unknown_topic(topic_id: UUID) -> Self {
        Self {
            topic_id,
            partitions: CVec {
                data: vec![FetchPartitionsResponse::unknown_topic()],
            },
            tagged_field: 0,
        }
    }
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchPartitionsResponse {
    pub partition_idx: i32,
    pub error_code: i16,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted_transactions: CVec<FetchPartitionsResponse>,
    pub preferred_read_replica: i32,
    pub records: u8,
    pub tagged_field: u8,
}

impl FetchPartitionsResponse {
    pub fn unknown_topic() -> Self {
        Self {
            partition_idx: 0,
            error_code: 100,
            high_watermark: 0,
            last_stable_offset: 0,
            log_start_offset: 0,
            aborted_transactions: CVec { data: vec![] },
            preferred_read_replica: 0,
            records: 0,
            tagged_field: 0,
        }
    }
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FetchResponse {
    pub basev1: BaseResponseV1,
    pub throttle_time: i32,
    pub error_code: i16,
    pub session_id: i32,
    pub responses: CVec<FetchTopicResponse>,
    pub tagged_field: u8,
}

impl FetchResponse {
    pub fn get_topics(correlation_id: i32, session_id: i32, topics: &Vec<TopicFetch>) -> Self {
        let base = BaseResponse::new_base(correlation_id);
        let tag_buffer = 0;
        let basev1 = BaseResponseV1 { base, tag_buffer };
        if topics.is_empty() {
            FetchResponse {
                basev1,
                throttle_time: 0,
                error_code: 0,
                session_id,
                responses: CVec { data: vec![] },
                tagged_field: tag_buffer,
            }
        } else {
            let mut ts: Vec<FetchTopicResponse> = vec![];
            for topic in topics {
                ts.push(FetchTopicResponse::unknown_topic(topic.topic_id.clone()));
            }
            FetchResponse {
                basev1,
                throttle_time: 0,
                error_code: 0,
                session_id,
                responses: CVec { data: ts },
                tagged_field: tag_buffer,
            }
        }
    }
}

impl FetchRequest {
    pub async fn handle_request(&self) -> Result<FetchResponse, Error> {
        let mut response = FetchResponse::get_topics(
            self.basev2.correlation_id,
            self.session_id,
            &self.topics.data,
        );

        let res_size = response.size_in_bytes() - 4;

        response.basev1.base.size = res_size as i32;

        Ok(response)
    }
}
