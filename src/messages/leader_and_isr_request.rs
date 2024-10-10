//! LeaderAndIsrRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/LeaderAndIsrRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, Decoder,
    Encodable, Encoder, HeaderVersion, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrLiveLeader {
    /// The leader's broker ID.
    ///
    /// Supported API versions: 0-7
    pub broker_id: super::BrokerId,

    /// The leader's hostname.
    ///
    /// Supported API versions: 0-7
    pub host_name: StrBytes,

    /// The leader's port.
    ///
    /// Supported API versions: 0-7
    pub port: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl LeaderAndIsrLiveLeader {
    /// Sets `broker_id` to the passed value.
    ///
    /// The leader's broker ID.
    ///
    /// Supported API versions: 0-7
    pub fn with_broker_id(mut self, value: super::BrokerId) -> Self {
        self.broker_id = value;
        self
    }
    /// Sets `host_name` to the passed value.
    ///
    /// The leader's hostname.
    ///
    /// Supported API versions: 0-7
    pub fn with_host_name(mut self, value: StrBytes) -> Self {
        self.host_name = value;
        self
    }
    /// Sets `port` to the passed value.
    ///
    /// The leader's port.
    ///
    /// Supported API versions: 0-7
    pub fn with_port(mut self, value: i32) -> Self {
        self.port = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "client")]
impl Encodable for LeaderAndIsrLiveLeader {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.broker_id)?;
        if version >= 4 {
            types::CompactString.encode(buf, &self.host_name)?;
        } else {
            types::String.encode(buf, &self.host_name)?;
        }
        types::Int32.encode(buf, &self.port)?;
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.broker_id)?;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.host_name)?;
        } else {
            total_size += types::String.compute_size(&self.host_name)?;
        }
        total_size += types::Int32.compute_size(&self.port)?;
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for LeaderAndIsrLiveLeader {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let broker_id = types::Int32.decode(buf)?;
        let host_name = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let port = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            broker_id,
            host_name,
            port,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrLiveLeader {
    fn default() -> Self {
        Self {
            broker_id: (0).into(),
            host_name: Default::default(),
            port: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrLiveLeader {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrPartitionState {
    /// The topic name.  This is only present in v0 or v1.
    ///
    /// Supported API versions: 0-1
    pub topic_name: super::TopicName,

    /// The partition index.
    ///
    /// Supported API versions: 0-7
    pub partition_index: i32,

    /// The controller epoch.
    ///
    /// Supported API versions: 0-7
    pub controller_epoch: i32,

    /// The broker ID of the leader.
    ///
    /// Supported API versions: 0-7
    pub leader: super::BrokerId,

    /// The leader epoch.
    ///
    /// Supported API versions: 0-7
    pub leader_epoch: i32,

    /// The in-sync replica IDs.
    ///
    /// Supported API versions: 0-7
    pub isr: Vec<super::BrokerId>,

    /// The current epoch for the partition. The epoch is a monotonically increasing value which is incremented after every partition change. (Since the LeaderAndIsr request is only used by the legacy controller, this corresponds to the zkVersion)
    ///
    /// Supported API versions: 0-7
    pub partition_epoch: i32,

    /// The replica IDs.
    ///
    /// Supported API versions: 0-7
    pub replicas: Vec<super::BrokerId>,

    /// The replica IDs that we are adding this partition to, or null if no replicas are being added.
    ///
    /// Supported API versions: 3-7
    pub adding_replicas: Vec<super::BrokerId>,

    /// The replica IDs that we are removing this partition from, or null if no replicas are being removed.
    ///
    /// Supported API versions: 3-7
    pub removing_replicas: Vec<super::BrokerId>,

    /// Whether the replica should have existed on the broker or not.
    ///
    /// Supported API versions: 1-7
    pub is_new: bool,

    /// 1 if the partition is recovering from an unclean leader election; 0 otherwise.
    ///
    /// Supported API versions: 6-7
    pub leader_recovery_state: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl LeaderAndIsrPartitionState {
    /// Sets `topic_name` to the passed value.
    ///
    /// The topic name.  This is only present in v0 or v1.
    ///
    /// Supported API versions: 0-1
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `partition_index` to the passed value.
    ///
    /// The partition index.
    ///
    /// Supported API versions: 0-7
    pub fn with_partition_index(mut self, value: i32) -> Self {
        self.partition_index = value;
        self
    }
    /// Sets `controller_epoch` to the passed value.
    ///
    /// The controller epoch.
    ///
    /// Supported API versions: 0-7
    pub fn with_controller_epoch(mut self, value: i32) -> Self {
        self.controller_epoch = value;
        self
    }
    /// Sets `leader` to the passed value.
    ///
    /// The broker ID of the leader.
    ///
    /// Supported API versions: 0-7
    pub fn with_leader(mut self, value: super::BrokerId) -> Self {
        self.leader = value;
        self
    }
    /// Sets `leader_epoch` to the passed value.
    ///
    /// The leader epoch.
    ///
    /// Supported API versions: 0-7
    pub fn with_leader_epoch(mut self, value: i32) -> Self {
        self.leader_epoch = value;
        self
    }
    /// Sets `isr` to the passed value.
    ///
    /// The in-sync replica IDs.
    ///
    /// Supported API versions: 0-7
    pub fn with_isr(mut self, value: Vec<super::BrokerId>) -> Self {
        self.isr = value;
        self
    }
    /// Sets `partition_epoch` to the passed value.
    ///
    /// The current epoch for the partition. The epoch is a monotonically increasing value which is incremented after every partition change. (Since the LeaderAndIsr request is only used by the legacy controller, this corresponds to the zkVersion)
    ///
    /// Supported API versions: 0-7
    pub fn with_partition_epoch(mut self, value: i32) -> Self {
        self.partition_epoch = value;
        self
    }
    /// Sets `replicas` to the passed value.
    ///
    /// The replica IDs.
    ///
    /// Supported API versions: 0-7
    pub fn with_replicas(mut self, value: Vec<super::BrokerId>) -> Self {
        self.replicas = value;
        self
    }
    /// Sets `adding_replicas` to the passed value.
    ///
    /// The replica IDs that we are adding this partition to, or null if no replicas are being added.
    ///
    /// Supported API versions: 3-7
    pub fn with_adding_replicas(mut self, value: Vec<super::BrokerId>) -> Self {
        self.adding_replicas = value;
        self
    }
    /// Sets `removing_replicas` to the passed value.
    ///
    /// The replica IDs that we are removing this partition from, or null if no replicas are being removed.
    ///
    /// Supported API versions: 3-7
    pub fn with_removing_replicas(mut self, value: Vec<super::BrokerId>) -> Self {
        self.removing_replicas = value;
        self
    }
    /// Sets `is_new` to the passed value.
    ///
    /// Whether the replica should have existed on the broker or not.
    ///
    /// Supported API versions: 1-7
    pub fn with_is_new(mut self, value: bool) -> Self {
        self.is_new = value;
        self
    }
    /// Sets `leader_recovery_state` to the passed value.
    ///
    /// 1 if the partition is recovering from an unclean leader election; 0 otherwise.
    ///
    /// Supported API versions: 6-7
    pub fn with_leader_recovery_state(mut self, value: i8) -> Self {
        self.leader_recovery_state = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "client")]
impl Encodable for LeaderAndIsrPartitionState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version <= 1 {
            types::String.encode(buf, &self.topic_name)?;
        }
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.controller_epoch)?;
        types::Int32.encode(buf, &self.leader)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        if version >= 4 {
            types::CompactArray(types::Int32).encode(buf, &self.isr)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.isr)?;
        }
        types::Int32.encode(buf, &self.partition_epoch)?;
        if version >= 4 {
            types::CompactArray(types::Int32).encode(buf, &self.replicas)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.replicas)?;
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Int32).encode(buf, &self.adding_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.adding_replicas)?;
            }
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Int32).encode(buf, &self.removing_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.removing_replicas)?;
            }
        }
        if version >= 1 {
            types::Boolean.encode(buf, &self.is_new)?;
        }
        if version >= 6 {
            types::Int8.encode(buf, &self.leader_recovery_state)?;
        } else {
            if self.leader_recovery_state != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version <= 1 {
            total_size += types::String.compute_size(&self.topic_name)?;
        }
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        total_size += types::Int32.compute_size(&self.leader)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        if version >= 4 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.isr)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.isr)?;
        }
        total_size += types::Int32.compute_size(&self.partition_epoch)?;
        if version >= 4 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.replicas)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.replicas)?;
        }
        if version >= 3 {
            if version >= 4 {
                total_size +=
                    types::CompactArray(types::Int32).compute_size(&self.adding_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.adding_replicas)?;
            }
        }
        if version >= 3 {
            if version >= 4 {
                total_size +=
                    types::CompactArray(types::Int32).compute_size(&self.removing_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.removing_replicas)?;
            }
        }
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.is_new)?;
        }
        if version >= 6 {
            total_size += types::Int8.compute_size(&self.leader_recovery_state)?;
        } else {
            if self.leader_recovery_state != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for LeaderAndIsrPartitionState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_name = if version <= 1 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let partition_index = types::Int32.decode(buf)?;
        let controller_epoch = types::Int32.decode(buf)?;
        let leader = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
        let isr = if version >= 4 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let partition_epoch = types::Int32.decode(buf)?;
        let replicas = if version >= 4 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let adding_replicas = if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let removing_replicas = if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let is_new = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let leader_recovery_state = if version >= 6 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            topic_name,
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            partition_epoch,
            replicas,
            adding_replicas,
            removing_replicas,
            is_new,
            leader_recovery_state,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrPartitionState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: 0,
            controller_epoch: 0,
            leader: (0).into(),
            leader_epoch: 0,
            isr: Default::default(),
            partition_epoch: 0,
            replicas: Default::default(),
            adding_replicas: Default::default(),
            removing_replicas: Default::default(),
            is_new: false,
            leader_recovery_state: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrPartitionState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrRequest {
    /// The current controller ID.
    ///
    /// Supported API versions: 0-7
    pub controller_id: super::BrokerId,

    /// If KRaft controller id is used during migration. See KIP-866
    ///
    /// Supported API versions: 7
    pub is_k_raft_controller: bool,

    /// The current controller epoch.
    ///
    /// Supported API versions: 0-7
    pub controller_epoch: i32,

    /// The current broker epoch.
    ///
    /// Supported API versions: 2-7
    pub broker_epoch: i64,

    /// The type that indicates whether all topics are included in the request
    ///
    /// Supported API versions: 5-7
    pub _type: i8,

    /// The state of each partition, in a v0 or v1 message.
    ///
    /// Supported API versions: 0-1
    pub ungrouped_partition_states: Vec<LeaderAndIsrPartitionState>,

    /// Each topic.
    ///
    /// Supported API versions: 2-7
    pub topic_states: Vec<LeaderAndIsrTopicState>,

    /// The current live leaders.
    ///
    /// Supported API versions: 0-7
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl LeaderAndIsrRequest {
    /// Sets `controller_id` to the passed value.
    ///
    /// The current controller ID.
    ///
    /// Supported API versions: 0-7
    pub fn with_controller_id(mut self, value: super::BrokerId) -> Self {
        self.controller_id = value;
        self
    }
    /// Sets `is_k_raft_controller` to the passed value.
    ///
    /// If KRaft controller id is used during migration. See KIP-866
    ///
    /// Supported API versions: 7
    pub fn with_is_k_raft_controller(mut self, value: bool) -> Self {
        self.is_k_raft_controller = value;
        self
    }
    /// Sets `controller_epoch` to the passed value.
    ///
    /// The current controller epoch.
    ///
    /// Supported API versions: 0-7
    pub fn with_controller_epoch(mut self, value: i32) -> Self {
        self.controller_epoch = value;
        self
    }
    /// Sets `broker_epoch` to the passed value.
    ///
    /// The current broker epoch.
    ///
    /// Supported API versions: 2-7
    pub fn with_broker_epoch(mut self, value: i64) -> Self {
        self.broker_epoch = value;
        self
    }
    /// Sets `_type` to the passed value.
    ///
    /// The type that indicates whether all topics are included in the request
    ///
    /// Supported API versions: 5-7
    pub fn with_type(mut self, value: i8) -> Self {
        self._type = value;
        self
    }
    /// Sets `ungrouped_partition_states` to the passed value.
    ///
    /// The state of each partition, in a v0 or v1 message.
    ///
    /// Supported API versions: 0-1
    pub fn with_ungrouped_partition_states(
        mut self,
        value: Vec<LeaderAndIsrPartitionState>,
    ) -> Self {
        self.ungrouped_partition_states = value;
        self
    }
    /// Sets `topic_states` to the passed value.
    ///
    /// Each topic.
    ///
    /// Supported API versions: 2-7
    pub fn with_topic_states(mut self, value: Vec<LeaderAndIsrTopicState>) -> Self {
        self.topic_states = value;
        self
    }
    /// Sets `live_leaders` to the passed value.
    ///
    /// The current live leaders.
    ///
    /// Supported API versions: 0-7
    pub fn with_live_leaders(mut self, value: Vec<LeaderAndIsrLiveLeader>) -> Self {
        self.live_leaders = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "client")]
impl Encodable for LeaderAndIsrRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.controller_id)?;
        if version >= 7 {
            types::Boolean.encode(buf, &self.is_k_raft_controller)?;
        } else {
            if self.is_k_raft_controller {
                bail!("failed to encode");
            }
        }
        types::Int32.encode(buf, &self.controller_epoch)?;
        if version >= 2 {
            types::Int64.encode(buf, &self.broker_epoch)?;
        }
        if version >= 5 {
            types::Int8.encode(buf, &self._type)?;
        } else {
            if self._type != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 1 {
            types::Array(types::Struct { version })
                .encode(buf, &self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topic_states)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topic_states)?;
            }
        } else {
            if !self.topic_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.live_leaders)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.live_leaders)?;
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.controller_id)?;
        if version >= 7 {
            total_size += types::Boolean.compute_size(&self.is_k_raft_controller)?;
        } else {
            if self.is_k_raft_controller {
                bail!("failed to encode");
            }
        }
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        if version >= 2 {
            total_size += types::Int64.compute_size(&self.broker_epoch)?;
        }
        if version >= 5 {
            total_size += types::Int8.compute_size(&self._type)?;
        } else {
            if self._type != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 1 {
            total_size += types::Array(types::Struct { version })
                .compute_size(&self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            if version >= 4 {
                total_size += types::CompactArray(types::Struct { version })
                    .compute_size(&self.topic_states)?;
            } else {
                total_size +=
                    types::Array(types::Struct { version }).compute_size(&self.topic_states)?;
            }
        } else {
            if !self.topic_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.live_leaders)?;
        } else {
            total_size +=
                types::Array(types::Struct { version }).compute_size(&self.live_leaders)?;
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for LeaderAndIsrRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let controller_id = types::Int32.decode(buf)?;
        let is_k_raft_controller = if version >= 7 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let controller_epoch = types::Int32.decode(buf)?;
        let broker_epoch = if version >= 2 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let _type = if version >= 5 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let ungrouped_partition_states = if version <= 1 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let topic_states = if version >= 2 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let live_leaders = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            controller_id,
            is_k_raft_controller,
            controller_epoch,
            broker_epoch,
            _type,
            ungrouped_partition_states,
            topic_states,
            live_leaders,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrRequest {
    fn default() -> Self {
        Self {
            controller_id: (0).into(),
            is_k_raft_controller: false,
            controller_epoch: 0,
            broker_epoch: -1,
            _type: 0,
            ungrouped_partition_states: Default::default(),
            topic_states: Default::default(),
            live_leaders: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrTopicState {
    /// The topic name.
    ///
    /// Supported API versions: 2-7
    pub topic_name: super::TopicName,

    /// The unique topic ID.
    ///
    /// Supported API versions: 5-7
    pub topic_id: Uuid,

    /// The state of each partition
    ///
    /// Supported API versions: 2-7
    pub partition_states: Vec<LeaderAndIsrPartitionState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl LeaderAndIsrTopicState {
    /// Sets `topic_name` to the passed value.
    ///
    /// The topic name.
    ///
    /// Supported API versions: 2-7
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `topic_id` to the passed value.
    ///
    /// The unique topic ID.
    ///
    /// Supported API versions: 5-7
    pub fn with_topic_id(mut self, value: Uuid) -> Self {
        self.topic_id = value;
        self
    }
    /// Sets `partition_states` to the passed value.
    ///
    /// The state of each partition
    ///
    /// Supported API versions: 2-7
    pub fn with_partition_states(mut self, value: Vec<LeaderAndIsrPartitionState>) -> Self {
        self.partition_states = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

#[cfg(feature = "client")]
impl Encodable for LeaderAndIsrTopicState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 2 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.topic_name)?;
            } else {
                types::String.encode(buf, &self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 5 {
            types::Uuid.encode(buf, &self.topic_id)?;
        }
        if version >= 2 {
            if version >= 4 {
                types::CompactArray(types::Struct { version })
                    .encode(buf, &self.partition_states)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.partition_states)?;
            }
        } else {
            if !self.partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 2 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.topic_name)?;
            } else {
                total_size += types::String.compute_size(&self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 5 {
            total_size += types::Uuid.compute_size(&self.topic_id)?;
        }
        if version >= 2 {
            if version >= 4 {
                total_size += types::CompactArray(types::Struct { version })
                    .compute_size(&self.partition_states)?;
            } else {
                total_size +=
                    types::Array(types::Struct { version }).compute_size(&self.partition_states)?;
            }
        } else {
            if !self.partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for LeaderAndIsrTopicState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_name = if version >= 2 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let topic_id = if version >= 5 {
            types::Uuid.decode(buf)?
        } else {
            Uuid::nil()
        };
        let partition_states = if version >= 2 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            topic_name,
            topic_id,
            partition_states,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrTopicState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            topic_id: Uuid::nil(),
            partition_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrTopicState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for LeaderAndIsrRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            2
        } else {
            1
        }
    }
}
