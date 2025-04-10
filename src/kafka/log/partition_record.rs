use crate::{types::array::CVec, Decode, Encode, Size};
use encode_derive::{Decode, Size};

use crate::types::uuid::UUID;

#[derive(Debug, Encode, Decode, Size)]
pub struct PartitionRecord {
    pub id: i32,
    pub topic_id: UUID,
    pub replicas: CVec<i32>,
    pub sync_replicas: CVec<i32>,
    pub removing_replicas: CVec<i32>,
    pub adding_replicas: CVec<i32>,
    pub leader: i32,
    pub leader_epoch: i32,
    pub partition_epoch: i32,
    pub directories: CVec<UUID>,
    pub tagged_fields: u8,
}
