use anyhow::Error;
use anyhow::Result;
use crc32c::crc32c;
use encode_derive::{Decode, Size};
use partition_record::PartitionRecord;
#[allow(non_camel_case_types, non_upper_case_globals, unreachable_patterns)]
use std::collections::{HashMap, HashSet};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use topic_log::TopicRecord;

use crate::{
    types::{
        array::{CSignedVec, CVec},
        bytes::ByteBuf,
        cstring::{CSignedString, CString},
        record::GenericRecord,
        uvarint::UVarint,
        varint::Varint,
    },
    Decode, Encode, Size,
};

use super::listpartitions::{PartitionResponse, TopicResponse};

pub mod partition_record;
pub mod topic_log;

static CLUSTER_METADATA: &str =
    "/tmp/kraft-combined-logs/__cluster_metadata-0/00000000000000000000.log";

pub async fn get_topics() -> Result<HashMap<String, TopicResponse>, Error> {
    let records = get_records_from_disk().await?;

    let mut topics_w_partitions: HashMap<String, Vec<PartitionResponse>> = HashMap::new();

    let filtered_topics: HashSet<String> = records
        .iter()
        .flat_map(|batch| &batch.records)
        .filter_map(|record| {
            if let RecordValue::Topic(topic) = &record.value.r_record {
                Some(topic.id.to_string())
            } else {
                None
            }
        })
        .collect();

    records
        .iter()
        .flat_map(|batch| &batch.records)
        .filter_map(|record| {
            if let RecordValue::Partition(partition) = &record.value.r_record {
                Some(partition)
            } else {
                None
            }
        })
        .filter(|partition| filtered_topics.contains(&partition.topic_id.to_string()))
        .for_each(|partition| {
            let entry = topics_w_partitions
                .entry(partition.topic_id.to_string())
                .or_insert_with(Vec::new);

            let idx = entry.len();

            entry.push(PartitionResponse {
                error_code: 0,
                partition_idx: idx as i32,
                leader_id: partition.leader,
                leader_epoch: partition.leader_epoch,
                replica_nodes: partition.replicas.clone(),
                in_sync_replicas: partition.sync_replicas.clone(),
                eligible_leader_replicas: CVec { data: vec![] },
                last_known_elr: CVec { data: vec![] },
                offline_replica: CVec { data: vec![] },
                tag_buffer: 0,
            });
        });

    let topic_lookup: HashMap<String, &TopicRecord> = records
        .iter()
        .flat_map(|batch| &batch.records)
        .filter_map(|record| {
            if let RecordValue::Topic(topic) = &record.value.r_record {
                Some((topic.id.to_string(), topic))
            } else {
                None
            }
        })
        .collect();

    let mut topic_map: HashMap<String, TopicResponse> = HashMap::new();

    for (topic_id, partitions) in topics_w_partitions {
        if let Some(topic) = topic_lookup.get(&topic_id) {
            let name = topic.name.0.to_string();
            topic_map.insert(
                name.clone(),
                TopicResponse {
                    error_code: 0,
                    name: topic.name.clone(),
                    id: topic.id.clone(),
                    is_internal: 0,
                    partitions_array: CVec { data: partitions },
                    authorized_ops: 0x00000df8,
                    tag_buffer: topic.tagged_fields,
                },
            );
        }
    }

    Ok(topic_map)
}

fn calculate_crc(batch: &TopicRecordBatch) -> u32 {
    let mut data: Vec<u8> = Vec::new();

    data.extend(batch.attributes.encode());
    data.extend(batch.last_offset_delta.encode());
    data.extend(batch.base_timestamp.encode());
    data.extend(batch.max_timestamp.encode());
    data.extend(batch.producer_id.encode());
    data.extend(batch.producer_epoch.encode());
    data.extend(batch.base_sequence.encode());
    data.extend(batch.records.encode());

    let crc = crc32c(&data);
    crc
}

pub async fn get_topic_records_from_disk(
    name: &str,
    partition: i32,
    _idx: i64,
) -> Result<ByteBuf, Error> {
    let mut file = File::open(format!(
        "/tmp/kraft-combined-logs/{}-{}/00000000000000000000.log",
        name, partition
    ))
    .await?;

    let metadata = file.metadata().await?;

    let mut buf = Vec::with_capacity(metadata.len().try_into()?);

    file.read_to_end(&mut buf).await?;

    if buf.is_empty() {
        return Ok(ByteBuf(vec![], UVarint::new(0, 1)));
    }

    println!("Contents: {buf:?}");

    let mut another_offset = 0;

    let data_as_bytes = ByteBuf::decode(&buf[..], &mut another_offset);

    Ok(data_as_bytes)
}

pub async fn get_records_from_disk() -> Result<Vec<RecordBatch>, Error> {
    let mut file = File::open(CLUSTER_METADATA).await?;

    let metadata = file.metadata().await?;

    let mut buf = Vec::with_capacity(metadata.len().try_into()?);

    file.read_to_end(&mut buf).await?;

    let mut batches: Vec<RecordBatch> = Vec::new();

    let mut offset = 0;

    while offset < buf.len() {
        let batch = RecordBatch::decode(&buf[..], &mut offset);
        batches.push(batch);
    }

    Ok(batches)
}

#[derive(Debug, Encode, Decode, Size)]
pub struct FeatureLevelRecord {
    pub name: CString,
    pub feature_level: i16,
    pub tagged_field: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct UnknownRecord {}

#[derive(Debug, Encode, Decode, Size)]
pub enum RecordValue {
    Topic(TopicRecord),
    FeatureLevel(FeatureLevelRecord),
    Partition(PartitionRecord),
    Unknown(UnknownRecord),
}

#[derive(Debug, Encode, Decode, Size)]
pub struct LogFile {
    pub data: Vec<RecordBatch>,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicRecordBatch {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic_byte: u8,
    pub crc: u32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: ByteBuf,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicHeaders {
    pub header_key: CString,
    pub value: Vec<u8>,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct TopicRecordDisk {
    pub length: Varint,
    pub attributes: u8,
    pub timestamp: Varint,
    pub delta_offset: Varint,
    pub key: CSignedVec<i32>,
    pub value: CSignedString,
    pub headers_array: CSignedVec<TopicHeaders>,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct MessageData {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic_byte: u8,
    pub crc: i32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub message: ByteBuf,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct RecordBatch {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic_byte: u8,
    pub crc: i32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: Vec<Record>,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct Record {
    pub length: Varint,
    pub attributes: u8,
    pub timestamp: Varint,
    pub delta_offset: Varint,
    pub key: CSignedVec<i32>,
    pub value_length: Varint,
    pub frame_version: u8,
    pub value: GenericRecord,
    pub headers_array: UVarint,
}

#[cfg(test)]
mod tests {
    use crate::{
        kafka::log::{RecordBatch, RecordValue, TopicRecordBatch},
        types::cstring::CString,
        Decode,
    };

    #[test]
    fn test_record_parsing() {
        let test_case_1 = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x00, 0x00,
            0x00, 0x01, 0x02, 0xb0, 0x69, 0x45, 0x7c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x91, 0xe0, 0x5a, 0xf8, 0x18, 0x00, 0x00, 0x01, 0x91, 0xe0, 0x5a, 0xf8,
            0x18, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0x00, 0x00, 0x00, 0x01, 0x3a, 0x00, 0x00, 0x00, 0x01, 0x2e, 0x01, 0x0c, 0x00,
            0x11, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x76, 0x65, 0x72, 0x73,
            0x69, 0x6f, 0x6e, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0xe4, 0x00, 0x00, 0x00, 0x01, 0x02, 0x24, 0xdb, 0x12, 0xdd,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0x91, 0xe0, 0x5b, 0x2d, 0x15,
            0x00, 0x00, 0x01, 0x91, 0xe0, 0x5b, 0x2d, 0x15, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x03, 0x3c, 0x00,
            0x00, 0x00, 0x01, 0x30, 0x01, 0x02, 0x00, 0x04, 0x73, 0x61, 0x7a, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x00,
            0x00, 0x90, 0x01, 0x00, 0x00, 0x02, 0x01, 0x82, 0x01, 0x01, 0x03, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x91, 0x02, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01,
            0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x90, 0x01, 0x00, 0x00, 0x04, 0x01, 0x82, 0x01, 0x01,
            0x03, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00,
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x02, 0x00, 0x00, 0x00, 0x01, 0x02,
            0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let mut offset = 0;

        let decoded = RecordBatch::decode(&test_case_1[..], &mut offset);

        assert_eq!(decoded.base_offset, 0);
        assert_eq!(decoded.batch_length, 79);
        assert_eq!(decoded.partition_leader_epoch, 1);
        assert_eq!(decoded.crc, -1335278212);
        assert_eq!(decoded.attributes, 0);
        assert_eq!(decoded.last_offset_delta, 0);
        assert_eq!(decoded.base_timestamp, 1726045943832);
        assert_eq!(decoded.max_timestamp, 1726045943832);
        assert_eq!(decoded.producer_id, -1);
        assert_eq!(decoded.producer_epoch, -1);
        assert_eq!(decoded.base_sequence, -1);
        assert_eq!(decoded.records.len(), 1);
        assert_eq!(decoded.records[0].attributes, 0);
        assert_eq!(decoded.records[0].timestamp.0, 0);
        assert_eq!(decoded.records[0].delta_offset.0, 0);
        assert_eq!(decoded.records[0].value_length.0, 23);

        match &decoded.records[0].value.r_record {
            RecordValue::FeatureLevel(value) => {
                assert_eq!(value.feature_level, 20);
                assert_eq!(value.name, CString("metadata.version".to_string(), 1))
            }
            _ => panic!("Expected Feature level to be decoded"),
        }
    }

    #[test]
    fn topic_batch() {
        let test_case: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 0, 0, 0, 2, 139, 170, 135, 42, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 0, 0, 0, 1, 52, 72, 101, 108, 108, 111, 32, 82,
            101, 118, 101, 114, 115, 101, 32, 69, 110, 103, 105, 110, 101, 101, 114, 105, 110, 103,
            33, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 68, 0, 0, 0, 0, 2, 100, 97, 124, 74, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 36, 0, 0, 0, 1, 24, 72, 101, 108, 108, 111,
            32, 69, 97, 114, 116, 104, 33, 0,
        ];

        let mut offset = 0;

        let decoded = TopicRecordBatch::decode(&test_case[..], &mut offset);

        println!("{decoded:?}");

        assert_eq!(decoded.base_offset, 0);
        assert_eq!(
            decoded.records[0].value.0,
            "Hello Reverse Engineering!".to_string()
        );
    }

    #[test]
    fn test_topic_fetch() {
        let test_case: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 2, 152, 236, 24, 211, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 36, 0, 0, 0, 1, 24, 72, 101, 108, 108, 111, 32, 87,
            111, 114, 108, 100, 33, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 82, 0, 0, 0, 0, 2, 139,
            170, 135, 42, 0, 0, 0, 0, 0, 0, 0, 0, 1, 145, 224, 91, 109, 139, 0, 0, 1, 145, 224, 91,
            109, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 0, 0, 0, 1, 52, 72,
            101, 108, 108, 111, 32, 82, 101, 118, 101, 114, 115, 101, 32, 69, 110, 103, 105, 110,
            101, 101, 114, 105, 110, 103, 33, 0,
        ];

        let mut offset = 0;

        let mut data: Vec<TopicRecordBatch> = Vec::new();

        while offset < test_case.len() {
            let decoded = TopicRecordBatch::decode(&test_case[..], &mut offset);
            data.push(decoded);
        }

        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_batch_2() {
        let test_case_2 = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xe4, 0x00, 0x00,
            0x00, 0x01, 0x02, 0x24, 0xdb, 0x12, 0xdd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x01, 0x91, 0xe0, 0x5b, 0x2d, 0x15, 0x00, 0x00, 0x01, 0x91, 0xe0, 0x5b, 0x2d,
            0x15, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0x00, 0x00, 0x00, 0x03, 0x3c, 0x00, 0x00, 0x00, 0x01, 0x30, 0x01, 0x02, 0x00,
            0x04, 0x73, 0x61, 0x7a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x00, 0x00, 0x90, 0x01, 0x00, 0x00, 0x02, 0x01,
            0x82, 0x01, 0x01, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x02, 0x00, 0x00,
            0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x90, 0x01,
            0x00, 0x00, 0x04, 0x01, 0x82, 0x01, 0x01, 0x03, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x91, 0x02, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x10, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00,
        ];

        let mut offset = 0;

        let decoded = RecordBatch::decode(&test_case_2[..], &mut offset);

        assert_eq!(decoded.base_offset, 1);
        assert_eq!(decoded.batch_length, 228);
        assert_eq!(decoded.partition_leader_epoch, 1);
        assert_eq!(decoded.magic_byte, 2);
        assert_eq!(decoded.crc, 618336989);
        assert_eq!(decoded.attributes, 0);
        assert_eq!(decoded.last_offset_delta, 2);
        assert_eq!(decoded.base_timestamp, 1726045957397);
        assert_eq!(decoded.max_timestamp, 1726045957397);
        assert_eq!(decoded.producer_id, -1);
        assert_eq!(decoded.producer_epoch, -1);
        assert_eq!(decoded.base_sequence, -1);
        assert_eq!(decoded.records.len(), 3);
        assert_eq!(decoded.records[0].length.0, 30);
        assert_eq!(decoded.records[0].attributes, 0);
        assert_eq!(decoded.records[0].timestamp.0, 0);
        assert_eq!(decoded.records[0].delta_offset.0, 0);
        assert_eq!(decoded.records[0].key.data.len(), 0);
        match &decoded.records[0].value.r_record {
            RecordValue::Topic(record) => {
                assert_eq!(record.name.0, "saz");
                assert_eq!(
                    record.id.0,
                    [
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x91,
                    ]
                );
                assert_eq!(record.tagged_fields, 0);
            }
            _ => panic!("Expected record to be Topic"),
        }
        assert_eq!(decoded.records[0].headers_array.0, 0);

        assert_eq!(decoded.records[1].length.0, 72);
        match &decoded.records[1].value.r_record {
            RecordValue::Partition(record) => {
                assert_eq!(record.id, 0);
            }
            _ => panic!("Expected record to be Topic"),
        }
        assert_eq!(decoded.records[2].length.0, 72);
        match &decoded.records[2].value.r_record {
            RecordValue::Partition(record) => {
                assert_eq!(record.id, 1);
                assert_eq!(record.replicas.data[0], 1);
            }
            _ => panic!("Expected record to be Topic"),
        }
    }

    //#[tokio::test]
    //async fn test_decode_from_file() {
    //    if let Ok(batch_vec) = get_records_from_disk().await {
    //        assert_eq!(batch_vec.len(), 2);
    //    } else {
    //        panic!("Could not parse records from disk")
    //    }
    //}
}
