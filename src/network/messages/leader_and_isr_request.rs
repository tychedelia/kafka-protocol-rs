//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrPartitionState {
    /// The topic name.  This is only present in v0 or v1.
    /// 
    /// Supported API versions: 0-1
    pub topic_name: super::TopicName,

    /// The partition index.
    /// 
    /// Supported API versions: 0-4
    pub partition_index: i32,

    /// The controller epoch.
    /// 
    /// Supported API versions: 0-4
    pub controller_epoch: i32,

    /// The broker ID of the leader.
    /// 
    /// Supported API versions: 0-4
    pub leader: super::BrokerId,

    /// The leader epoch.
    /// 
    /// Supported API versions: 0-4
    pub leader_epoch: i32,

    /// The in-sync replica IDs.
    /// 
    /// Supported API versions: 0-4
    pub isr: Vec<i32>,

    /// The ZooKeeper version.
    /// 
    /// Supported API versions: 0-4
    pub zk_version: i32,

    /// The replica IDs.
    /// 
    /// Supported API versions: 0-4
    pub replicas: Vec<i32>,

    /// The replica IDs that we are adding this partition to, or null if no replicas are being added.
    /// 
    /// Supported API versions: 3-4
    pub adding_replicas: Vec<i32>,

    /// The replica IDs that we are removing this partition from, or null if no replicas are being removed.
    /// 
    /// Supported API versions: 3-4
    pub removing_replicas: Vec<i32>,

    /// Whether the replica should have existed on the broker or not.
    /// 
    /// Supported API versions: 1-4
    pub is_new: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrPartitionState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 1 {
            types::String.encode(buf, &self.topic_name)?;
        }
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.controller_epoch)?;
        types::Int32.encode(buf, &self.leader)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        if version == 4 {
            types::CompactArray(types::Int32).encode(buf, &self.isr)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.isr)?;
        }
        types::Int32.encode(buf, &self.zk_version)?;
        if version == 4 {
            types::CompactArray(types::Int32).encode(buf, &self.replicas)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.replicas)?;
        }
        if version >= 3 {
            if version == 4 {
                types::CompactArray(types::Int32).encode(buf, &self.adding_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.adding_replicas)?;
            }
        }
        if version >= 3 {
            if version == 4 {
                types::CompactArray(types::Int32).encode(buf, &self.removing_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.removing_replicas)?;
            }
        }
        if version >= 1 {
            types::Boolean.encode(buf, &self.is_new)?;
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version <= 1 {
            total_size += types::String.compute_size(&self.topic_name)?;
        }
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        total_size += types::Int32.compute_size(&self.leader)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        if version == 4 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.isr)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.isr)?;
        }
        total_size += types::Int32.compute_size(&self.zk_version)?;
        if version == 4 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.replicas)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.replicas)?;
        }
        if version >= 3 {
            if version == 4 {
                total_size += types::CompactArray(types::Int32).compute_size(&self.adding_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.adding_replicas)?;
            }
        }
        if version >= 3 {
            if version == 4 {
                total_size += types::CompactArray(types::Int32).compute_size(&self.removing_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.removing_replicas)?;
            }
        }
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.is_new)?;
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for LeaderAndIsrPartitionState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version <= 1 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let partition_index = types::Int32.decode(buf)?;
        let controller_epoch = types::Int32.decode(buf)?;
        let leader = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
        let isr = if version == 4 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let zk_version = types::Int32.decode(buf)?;
        let replicas = if version == 4 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let adding_replicas = if version >= 3 {
            if version == 4 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let removing_replicas = if version >= 3 {
            if version == 4 {
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
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
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
            zk_version,
            replicas,
            adding_replicas,
            removing_replicas,
            is_new,
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
            zk_version: 0,
            replicas: Default::default(),
            adding_replicas: Default::default(),
            removing_replicas: Default::default(),
            is_new: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrPartitionState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrTopicState {
    /// The topic name.
    /// 
    /// Supported API versions: 2-4
    pub topic_name: super::TopicName,

    /// The state of each partition
    /// 
    /// Supported API versions: 2-4
    pub partition_states: Vec<LeaderAndIsrPartitionState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrTopicState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            if version == 4 {
                types::CompactString.encode(buf, &self.topic_name)?;
            } else {
                types::String.encode(buf, &self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            if version == 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.partition_states)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.partition_states)?;
            }
        } else {
            if !self.partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            if version == 4 {
                total_size += types::CompactString.compute_size(&self.topic_name)?;
            } else {
                total_size += types::String.compute_size(&self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            if version == 4 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_states)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.partition_states)?;
            }
        } else {
            if !self.partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for LeaderAndIsrTopicState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version >= 2 {
            if version == 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let partition_states = if version >= 2 {
            if version == 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            topic_name,
            partition_states,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrTopicState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrTopicState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrLiveLeader {
    /// The leader's broker ID.
    /// 
    /// Supported API versions: 0-4
    pub broker_id: super::BrokerId,

    /// The leader's hostname.
    /// 
    /// Supported API versions: 0-4
    pub host_name: StrBytes,

    /// The leader's port.
    /// 
    /// Supported API versions: 0-4
    pub port: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrLiveLeader {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.broker_id)?;
        if version == 4 {
            types::CompactString.encode(buf, &self.host_name)?;
        } else {
            types::String.encode(buf, &self.host_name)?;
        }
        types::Int32.encode(buf, &self.port)?;
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.broker_id)?;
        if version == 4 {
            total_size += types::CompactString.compute_size(&self.host_name)?;
        } else {
            total_size += types::String.compute_size(&self.host_name)?;
        }
        total_size += types::Int32.compute_size(&self.port)?;
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for LeaderAndIsrLiveLeader {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let broker_id = types::Int32.decode(buf)?;
        let host_name = if version == 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let port = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrRequest {
    /// The current controller ID.
    /// 
    /// Supported API versions: 0-4
    pub controller_id: super::BrokerId,

    /// The current controller epoch.
    /// 
    /// Supported API versions: 0-4
    pub controller_epoch: i32,

    /// The current broker epoch.
    /// 
    /// Supported API versions: 2-4
    pub broker_epoch: i64,

    /// The state of each partition, in a v0 or v1 message.
    /// 
    /// Supported API versions: 0-1
    pub ungrouped_partition_states: Vec<LeaderAndIsrPartitionState>,

    /// Each topic.
    /// 
    /// Supported API versions: 2-4
    pub topic_states: Vec<LeaderAndIsrTopicState>,

    /// The current live leaders.
    /// 
    /// Supported API versions: 0-4
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.controller_id)?;
        types::Int32.encode(buf, &self.controller_epoch)?;
        if version >= 2 {
            types::Int64.encode(buf, &self.broker_epoch)?;
        }
        if version <= 1 {
            types::Array(types::Struct { version }).encode(buf, &self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            if version == 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topic_states)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topic_states)?;
            }
        } else {
            if !self.topic_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.live_leaders)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.live_leaders)?;
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.controller_id)?;
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        if version >= 2 {
            total_size += types::Int64.compute_size(&self.broker_epoch)?;
        }
        if version <= 1 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            if version == 4 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topic_states)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.topic_states)?;
            }
        } else {
            if !self.topic_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.live_leaders)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.live_leaders)?;
        }
        if version == 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for LeaderAndIsrRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let controller_id = types::Int32.decode(buf)?;
        let controller_epoch = types::Int32.decode(buf)?;
        let broker_epoch = if version >= 2 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let ungrouped_partition_states = if version <= 1 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let topic_states = if version >= 2 {
            if version == 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let live_leaders = if version == 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            controller_id,
            controller_epoch,
            broker_epoch,
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
            controller_epoch: 0,
            broker_epoch: -1,
            ungrouped_partition_states: Default::default(),
            topic_states: Default::default(),
            live_leaders: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for LeaderAndIsrRequest {
    fn header_version(version: i16) -> i16 {
        if version == 4 {
            2
        } else {
            1
        }
    }
}

