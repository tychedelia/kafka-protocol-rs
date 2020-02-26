//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataResponseBroker {
    /// The broker hostname.
    /// 
    /// Supported API versions: 0-9
    pub host: StrBytes,

    /// The broker port.
    /// 
    /// Supported API versions: 0-9
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    /// 
    /// Supported API versions: 1-9
    pub rack: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for MetadataResponseBroker {
    type Key = super::BrokerId;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, key)?;
        if version == 9 {
            types::CompactString.encode(buf, &self.host)?;
        } else {
            types::String.encode(buf, &self.host)?;
        }
        types::Int32.encode(buf, &self.port)?;
        if version >= 1 {
            if version == 9 {
                types::CompactString.encode(buf, &self.rack)?;
            } else {
                types::String.encode(buf, &self.rack)?;
            }
        }
        if version == 9 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(key)?;
        if version == 9 {
            total_size += types::CompactString.compute_size(&self.host)?;
        } else {
            total_size += types::String.compute_size(&self.host)?;
        }
        total_size += types::Int32.compute_size(&self.port)?;
        if version >= 1 {
            if version == 9 {
                total_size += types::CompactString.compute_size(&self.rack)?;
            } else {
                total_size += types::String.compute_size(&self.rack)?;
            }
        }
        if version == 9 {
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

impl MapDecodable for MetadataResponseBroker {
    type Key = super::BrokerId;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int32.decode(buf)?;
        let host = if version == 9 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let port = types::Int32.decode(buf)?;
        let rack = if version >= 1 {
            if version == 9 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            host,
            port,
            rack,
            unknown_tagged_fields,
        }))
    }
}

impl Default for MetadataResponseBroker {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: 0,
            rack: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataResponseBroker {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-9
    pub error_code: i16,

    /// The partition index.
    /// 
    /// Supported API versions: 0-9
    pub partition_index: i32,

    /// The ID of the leader broker.
    /// 
    /// Supported API versions: 0-9
    pub leader_id: super::BrokerId,

    /// The leader epoch of this partition.
    /// 
    /// Supported API versions: 7-9
    pub leader_epoch: i32,

    /// The set of all nodes that host this partition.
    /// 
    /// Supported API versions: 0-9
    pub replica_nodes: Vec<super::BrokerId>,

    /// The set of nodes that are in sync with the leader for this partition.
    /// 
    /// Supported API versions: 0-9
    pub isr_nodes: Vec<i32>,

    /// The set of offline replicas of this partition.
    /// 
    /// Supported API versions: 5-9
    pub offline_replicas: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for MetadataResponsePartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.leader_id)?;
        if version >= 7 {
            types::Int32.encode(buf, &self.leader_epoch)?;
        }
        if version == 9 {
            types::CompactArray(types::Int32).encode(buf, &self.replica_nodes)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.replica_nodes)?;
        }
        if version == 9 {
            types::CompactArray(types::Int32).encode(buf, &self.isr_nodes)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.isr_nodes)?;
        }
        if version >= 5 {
            if version == 9 {
                types::CompactArray(types::Int32).encode(buf, &self.offline_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.offline_replicas)?;
            }
        }
        if version == 9 {
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        if version >= 7 {
            total_size += types::Int32.compute_size(&self.leader_epoch)?;
        }
        if version == 9 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.replica_nodes)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.replica_nodes)?;
        }
        if version == 9 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.isr_nodes)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.isr_nodes)?;
        }
        if version >= 5 {
            if version == 9 {
                total_size += types::CompactArray(types::Int32).compute_size(&self.offline_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.offline_replicas)?;
            }
        }
        if version == 9 {
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

impl Decodable for MetadataResponsePartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let partition_index = types::Int32.decode(buf)?;
        let leader_id = types::Int32.decode(buf)?;
        let leader_epoch = if version >= 7 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let replica_nodes = if version == 9 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let isr_nodes = if version == 9 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let offline_replicas = if version >= 5 {
            if version == 9 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
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
            error_code,
            partition_index,
            leader_id,
            leader_epoch,
            replica_nodes,
            isr_nodes,
            offline_replicas,
            unknown_tagged_fields,
        })
    }
}

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_index: 0,
            leader_id: (0).into(),
            leader_epoch: -1,
            replica_nodes: Default::default(),
            isr_nodes: Default::default(),
            offline_replicas: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataResponsePartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-9
    pub error_code: i16,

    /// True if the topic is internal.
    /// 
    /// Supported API versions: 1-9
    pub is_internal: bool,

    /// Each partition in the topic.
    /// 
    /// Supported API versions: 0-9
    pub partitions: Vec<MetadataResponsePartition>,

    /// 32-bit bitfield to represent authorized operations for this topic.
    /// 
    /// Supported API versions: 8-9
    pub topic_authorized_operations: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for MetadataResponseTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version == 9 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 1 {
            types::Boolean.encode(buf, &self.is_internal)?;
        }
        if version == 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 8 {
            types::Int32.encode(buf, &self.topic_authorized_operations)?;
        } else {
            if self.topic_authorized_operations != -2147483648 {
                return Err(EncodeError)
            }
        }
        if version == 9 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version == 9 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.is_internal)?;
        }
        if version == 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 8 {
            total_size += types::Int32.compute_size(&self.topic_authorized_operations)?;
        } else {
            if self.topic_authorized_operations != -2147483648 {
                return Err(EncodeError)
            }
        }
        if version == 9 {
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

impl MapDecodable for MetadataResponseTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let key_field = if version == 9 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let is_internal = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let partitions = if version == 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let topic_authorized_operations = if version >= 8 {
            types::Int32.decode(buf)?
        } else {
            -2147483648
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            error_code,
            is_internal,
            partitions,
            topic_authorized_operations,
            unknown_tagged_fields,
        }))
    }
}

impl Default for MetadataResponseTopic {
    fn default() -> Self {
        Self {
            error_code: 0,
            is_internal: false,
            partitions: Default::default(),
            topic_authorized_operations: -2147483648,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataResponseTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 3-9
    pub throttle_time_ms: i32,

    /// Each broker in the response.
    /// 
    /// Supported API versions: 0-9
    pub brokers: indexmap::IndexMap<super::BrokerId, MetadataResponseBroker>,

    /// The cluster ID that responding broker belongs to.
    /// 
    /// Supported API versions: 2-9
    pub cluster_id: Option<StrBytes>,

    /// The ID of the controller broker.
    /// 
    /// Supported API versions: 1-9
    pub controller_id: super::BrokerId,

    /// Each topic in the response.
    /// 
    /// Supported API versions: 0-9
    pub topics: indexmap::IndexMap<super::TopicName, MetadataResponseTopic>,

    /// 32-bit bitfield to represent authorized operations for this cluster.
    /// 
    /// Supported API versions: 8-9
    pub cluster_authorized_operations: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for MetadataResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version == 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.brokers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.brokers)?;
        }
        if version >= 2 {
            if version == 9 {
                types::CompactString.encode(buf, &self.cluster_id)?;
            } else {
                types::String.encode(buf, &self.cluster_id)?;
            }
        }
        if version >= 1 {
            types::Int32.encode(buf, &self.controller_id)?;
        }
        if version == 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 8 {
            types::Int32.encode(buf, &self.cluster_authorized_operations)?;
        } else {
            if self.cluster_authorized_operations != -2147483648 {
                return Err(EncodeError)
            }
        }
        if version == 9 {
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
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version == 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.brokers)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.brokers)?;
        }
        if version >= 2 {
            if version == 9 {
                total_size += types::CompactString.compute_size(&self.cluster_id)?;
            } else {
                total_size += types::String.compute_size(&self.cluster_id)?;
            }
        }
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.controller_id)?;
        }
        if version == 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 8 {
            total_size += types::Int32.compute_size(&self.cluster_authorized_operations)?;
        } else {
            if self.cluster_authorized_operations != -2147483648 {
                return Err(EncodeError)
            }
        }
        if version == 9 {
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

impl Decodable for MetadataResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let brokers = if version == 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let cluster_id = if version >= 2 {
            if version == 9 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let controller_id = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            (-1).into()
        };
        let topics = if version == 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let cluster_authorized_operations = if version >= 8 {
            types::Int32.decode(buf)?
        } else {
            -2147483648
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
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
            throttle_time_ms,
            brokers,
            cluster_id,
            controller_id,
            topics,
            cluster_authorized_operations,
            unknown_tagged_fields,
        })
    }
}

impl Default for MetadataResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            brokers: Default::default(),
            cluster_id: None,
            controller_id: (-1).into(),
            topics: Default::default(),
            cluster_authorized_operations: -2147483648,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

impl HeaderVersion for MetadataResponse {
    fn header_version(version: i16) -> i16 {
        if version == 9 {
            1
        } else {
            0
        }
    }
}

