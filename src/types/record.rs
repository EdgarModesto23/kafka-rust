use crate::{
    kafka::log::{
        partition_record::PartitionRecord, topic_log::TopicRecord, FeatureLevelRecord, RecordValue,
        UnknownRecord,
    },
    Decode, Encode, Offset, Size,
};

#[derive(Debug)]
pub struct GenericRecord {
    pub r_record: RecordValue,
}

impl Encode for GenericRecord {
    fn encode(&self) -> Vec<u8> {
        self.r_record.encode()
    }
}

impl Decode for GenericRecord {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self {
        let t = u8::decode(bytes, offset);
        let v = u8::decode(bytes, offset);

        let r_record = match t {
            12 => RecordValue::FeatureLevel(FeatureLevelRecord::decode(bytes, offset)),
            2 => RecordValue::Topic(TopicRecord::decode(bytes, offset)),
            3 => RecordValue::Partition(PartitionRecord::decode(bytes, offset)),
            _ => RecordValue::Unknown(UnknownRecord {}),
        };

        Self { r_record }
    }
}

impl Offset for GenericRecord {
    fn size(&self) -> usize {
        self.r_record.size_in_bytes()
    }
}

impl Size for GenericRecord {
    fn size_in_bytes(&self) -> usize {
        self.r_record.size_in_bytes()
    }
}
