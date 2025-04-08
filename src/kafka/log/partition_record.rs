use crate::{Decode, Encode, Size};
use encode_derive::{Decode, Size};

use crate::types::uuid::UUID;

#[derive(Debug, Encode, Decode, Size)]
pub struct PartitionRecord {
    pub id: i32,
    pub topic_id: UUID,
    pub replicas: Vec<i32>,
    pub sync_replicas: Vec<i32>,
    pub removing_replicas: Vec<i32>,
    pub adding_replicas: Vec<i32>,
    pub leader: i32,
    pub leader_epoch: i32,
    pub partition_epoch: i32,
    pub directories: Vec<UUID>,
    pub tagged_fields: u8,
}
