//! UpdateMetadataRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/UpdateMetadataRequest.json).
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
    Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataBroker {
    /// The broker id.
    ///
    /// Supported API versions: 0-8
    pub id: super::BrokerId,

    /// The broker hostname.
    ///
    /// Supported API versions: 0
    pub v0_host: StrBytes,

    /// The broker port.
    ///
    /// Supported API versions: 0
    pub v0_port: i32,

    /// The broker endpoints.
    ///
    /// Supported API versions: 1-8
    pub endpoints: Vec<UpdateMetadataEndpoint>,

    /// The rack which this broker belongs to.
    ///
    /// Supported API versions: 2-8
    pub rack: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataBroker {
    /// Sets `id` to the passed value.
    ///
    /// The broker id.
    ///
    /// Supported API versions: 0-8
    pub fn with_id(mut self, value: super::BrokerId) -> Self {
        self.id = value;
        self
    }
    /// Sets `v0_host` to the passed value.
    ///
    /// The broker hostname.
    ///
    /// Supported API versions: 0
    pub fn with_v0_host(mut self, value: StrBytes) -> Self {
        self.v0_host = value;
        self
    }
    /// Sets `v0_port` to the passed value.
    ///
    /// The broker port.
    ///
    /// Supported API versions: 0
    pub fn with_v0_port(mut self, value: i32) -> Self {
        self.v0_port = value;
        self
    }
    /// Sets `endpoints` to the passed value.
    ///
    /// The broker endpoints.
    ///
    /// Supported API versions: 1-8
    pub fn with_endpoints(mut self, value: Vec<UpdateMetadataEndpoint>) -> Self {
        self.endpoints = value;
        self
    }
    /// Sets `rack` to the passed value.
    ///
    /// The rack which this broker belongs to.
    ///
    /// Supported API versions: 2-8
    pub fn with_rack(mut self, value: Option<StrBytes>) -> Self {
        self.rack = value;
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
impl Encodable for UpdateMetadataBroker {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.id)?;
        if version == 0 {
            types::String.encode(buf, &self.v0_host)?;
        }
        if version == 0 {
            types::Int32.encode(buf, &self.v0_port)?;
        }
        if version >= 1 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.endpoints)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.endpoints)?;
            }
        }
        if version >= 2 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.rack)?;
            } else {
                types::String.encode(buf, &self.rack)?;
            }
        }
        if version >= 6 {
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
        total_size += types::Int32.compute_size(&self.id)?;
        if version == 0 {
            total_size += types::String.compute_size(&self.v0_host)?;
        }
        if version == 0 {
            total_size += types::Int32.compute_size(&self.v0_port)?;
        }
        if version >= 1 {
            if version >= 6 {
                total_size +=
                    types::CompactArray(types::Struct { version }).compute_size(&self.endpoints)?;
            } else {
                total_size +=
                    types::Array(types::Struct { version }).compute_size(&self.endpoints)?;
            }
        }
        if version >= 2 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.rack)?;
            } else {
                total_size += types::String.compute_size(&self.rack)?;
            }
        }
        if version >= 6 {
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
impl Decodable for UpdateMetadataBroker {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let id = types::Int32.decode(buf)?;
        let v0_host = if version == 0 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let v0_port = if version == 0 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let endpoints = if version >= 1 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let rack = if version >= 2 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            id,
            v0_host,
            v0_port,
            endpoints,
            rack,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateMetadataBroker {
    fn default() -> Self {
        Self {
            id: (0).into(),
            v0_host: Default::default(),
            v0_port: 0,
            endpoints: Default::default(),
            rack: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataBroker {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataEndpoint {
    /// The port of this endpoint
    ///
    /// Supported API versions: 1-8
    pub port: i32,

    /// The hostname of this endpoint
    ///
    /// Supported API versions: 1-8
    pub host: StrBytes,

    /// The listener name.
    ///
    /// Supported API versions: 3-8
    pub listener: StrBytes,

    /// The security protocol type.
    ///
    /// Supported API versions: 1-8
    pub security_protocol: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataEndpoint {
    /// Sets `port` to the passed value.
    ///
    /// The port of this endpoint
    ///
    /// Supported API versions: 1-8
    pub fn with_port(mut self, value: i32) -> Self {
        self.port = value;
        self
    }
    /// Sets `host` to the passed value.
    ///
    /// The hostname of this endpoint
    ///
    /// Supported API versions: 1-8
    pub fn with_host(mut self, value: StrBytes) -> Self {
        self.host = value;
        self
    }
    /// Sets `listener` to the passed value.
    ///
    /// The listener name.
    ///
    /// Supported API versions: 3-8
    pub fn with_listener(mut self, value: StrBytes) -> Self {
        self.listener = value;
        self
    }
    /// Sets `security_protocol` to the passed value.
    ///
    /// The security protocol type.
    ///
    /// Supported API versions: 1-8
    pub fn with_security_protocol(mut self, value: i16) -> Self {
        self.security_protocol = value;
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
impl Encodable for UpdateMetadataEndpoint {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 1 {
            types::Int32.encode(buf, &self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 1 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.host)?;
            } else {
                types::String.encode(buf, &self.host)?;
            }
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.listener)?;
            } else {
                types::String.encode(buf, &self.listener)?;
            }
        }
        if version >= 1 {
            types::Int16.encode(buf, &self.security_protocol)?;
        } else {
            if self.security_protocol != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
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
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 1 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.host)?;
            } else {
                total_size += types::String.compute_size(&self.host)?;
            }
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.listener)?;
            } else {
                total_size += types::String.compute_size(&self.listener)?;
            }
        }
        if version >= 1 {
            total_size += types::Int16.compute_size(&self.security_protocol)?;
        } else {
            if self.security_protocol != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
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
impl Decodable for UpdateMetadataEndpoint {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let port = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let host = if version >= 1 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let listener = if version >= 3 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let security_protocol = if version >= 1 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            port,
            host,
            listener,
            security_protocol,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateMetadataEndpoint {
    fn default() -> Self {
        Self {
            port: 0,
            host: Default::default(),
            listener: Default::default(),
            security_protocol: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataEndpoint {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataPartitionState {
    /// In older versions of this RPC, the topic name.
    ///
    /// Supported API versions: 0-4
    pub topic_name: super::TopicName,

    /// The partition index.
    ///
    /// Supported API versions: 0-8
    pub partition_index: i32,

    /// The controller epoch.
    ///
    /// Supported API versions: 0-8
    pub controller_epoch: i32,

    /// The ID of the broker which is the current partition leader.
    ///
    /// Supported API versions: 0-8
    pub leader: super::BrokerId,

    /// The leader epoch of this partition.
    ///
    /// Supported API versions: 0-8
    pub leader_epoch: i32,

    /// The brokers which are in the ISR for this partition.
    ///
    /// Supported API versions: 0-8
    pub isr: Vec<super::BrokerId>,

    /// The Zookeeper version.
    ///
    /// Supported API versions: 0-8
    pub zk_version: i32,

    /// All the replicas of this partition.
    ///
    /// Supported API versions: 0-8
    pub replicas: Vec<super::BrokerId>,

    /// The replicas of this partition which are offline.
    ///
    /// Supported API versions: 4-8
    pub offline_replicas: Vec<super::BrokerId>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataPartitionState {
    /// Sets `topic_name` to the passed value.
    ///
    /// In older versions of this RPC, the topic name.
    ///
    /// Supported API versions: 0-4
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `partition_index` to the passed value.
    ///
    /// The partition index.
    ///
    /// Supported API versions: 0-8
    pub fn with_partition_index(mut self, value: i32) -> Self {
        self.partition_index = value;
        self
    }
    /// Sets `controller_epoch` to the passed value.
    ///
    /// The controller epoch.
    ///
    /// Supported API versions: 0-8
    pub fn with_controller_epoch(mut self, value: i32) -> Self {
        self.controller_epoch = value;
        self
    }
    /// Sets `leader` to the passed value.
    ///
    /// The ID of the broker which is the current partition leader.
    ///
    /// Supported API versions: 0-8
    pub fn with_leader(mut self, value: super::BrokerId) -> Self {
        self.leader = value;
        self
    }
    /// Sets `leader_epoch` to the passed value.
    ///
    /// The leader epoch of this partition.
    ///
    /// Supported API versions: 0-8
    pub fn with_leader_epoch(mut self, value: i32) -> Self {
        self.leader_epoch = value;
        self
    }
    /// Sets `isr` to the passed value.
    ///
    /// The brokers which are in the ISR for this partition.
    ///
    /// Supported API versions: 0-8
    pub fn with_isr(mut self, value: Vec<super::BrokerId>) -> Self {
        self.isr = value;
        self
    }
    /// Sets `zk_version` to the passed value.
    ///
    /// The Zookeeper version.
    ///
    /// Supported API versions: 0-8
    pub fn with_zk_version(mut self, value: i32) -> Self {
        self.zk_version = value;
        self
    }
    /// Sets `replicas` to the passed value.
    ///
    /// All the replicas of this partition.
    ///
    /// Supported API versions: 0-8
    pub fn with_replicas(mut self, value: Vec<super::BrokerId>) -> Self {
        self.replicas = value;
        self
    }
    /// Sets `offline_replicas` to the passed value.
    ///
    /// The replicas of this partition which are offline.
    ///
    /// Supported API versions: 4-8
    pub fn with_offline_replicas(mut self, value: Vec<super::BrokerId>) -> Self {
        self.offline_replicas = value;
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
impl Encodable for UpdateMetadataPartitionState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version <= 4 {
            types::String.encode(buf, &self.topic_name)?;
        }
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.controller_epoch)?;
        types::Int32.encode(buf, &self.leader)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        if version >= 6 {
            types::CompactArray(types::Int32).encode(buf, &self.isr)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.isr)?;
        }
        types::Int32.encode(buf, &self.zk_version)?;
        if version >= 6 {
            types::CompactArray(types::Int32).encode(buf, &self.replicas)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.replicas)?;
        }
        if version >= 4 {
            if version >= 6 {
                types::CompactArray(types::Int32).encode(buf, &self.offline_replicas)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.offline_replicas)?;
            }
        }
        if version >= 6 {
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
        if version <= 4 {
            total_size += types::String.compute_size(&self.topic_name)?;
        }
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        total_size += types::Int32.compute_size(&self.leader)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        if version >= 6 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.isr)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.isr)?;
        }
        total_size += types::Int32.compute_size(&self.zk_version)?;
        if version >= 6 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.replicas)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.replicas)?;
        }
        if version >= 4 {
            if version >= 6 {
                total_size +=
                    types::CompactArray(types::Int32).compute_size(&self.offline_replicas)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.offline_replicas)?;
            }
        }
        if version >= 6 {
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
impl Decodable for UpdateMetadataPartitionState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_name = if version <= 4 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let partition_index = types::Int32.decode(buf)?;
        let controller_epoch = types::Int32.decode(buf)?;
        let leader = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
        let isr = if version >= 6 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let zk_version = types::Int32.decode(buf)?;
        let replicas = if version >= 6 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let offline_replicas = if version >= 4 {
            if version >= 6 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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
            zk_version,
            replicas,
            offline_replicas,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateMetadataPartitionState {
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
            offline_replicas: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataPartitionState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataRequest {
    /// The controller id.
    ///
    /// Supported API versions: 0-8
    pub controller_id: super::BrokerId,

    /// If KRaft controller id is used during migration. See KIP-866
    ///
    /// Supported API versions: 8
    pub is_k_raft_controller: bool,

    /// Indicates if this request is a Full metadata snapshot (2), Incremental (1), or Unknown (0). Using during ZK migration, see KIP-866
    ///
    /// Supported API versions: 8
    pub _type: i8,

    /// The controller epoch.
    ///
    /// Supported API versions: 0-8
    pub controller_epoch: i32,

    /// The broker epoch.
    ///
    /// Supported API versions: 5-8
    pub broker_epoch: i64,

    /// In older versions of this RPC, each partition that we would like to update.
    ///
    /// Supported API versions: 0-4
    pub ungrouped_partition_states: Vec<UpdateMetadataPartitionState>,

    /// In newer versions of this RPC, each topic that we would like to update.
    ///
    /// Supported API versions: 5-8
    pub topic_states: Vec<UpdateMetadataTopicState>,

    ///
    ///
    /// Supported API versions: 0-8
    pub live_brokers: Vec<UpdateMetadataBroker>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataRequest {
    /// Sets `controller_id` to the passed value.
    ///
    /// The controller id.
    ///
    /// Supported API versions: 0-8
    pub fn with_controller_id(mut self, value: super::BrokerId) -> Self {
        self.controller_id = value;
        self
    }
    /// Sets `is_k_raft_controller` to the passed value.
    ///
    /// If KRaft controller id is used during migration. See KIP-866
    ///
    /// Supported API versions: 8
    pub fn with_is_k_raft_controller(mut self, value: bool) -> Self {
        self.is_k_raft_controller = value;
        self
    }
    /// Sets `_type` to the passed value.
    ///
    /// Indicates if this request is a Full metadata snapshot (2), Incremental (1), or Unknown (0). Using during ZK migration, see KIP-866
    ///
    /// Supported API versions: 8
    pub fn with_type(mut self, value: i8) -> Self {
        self._type = value;
        self
    }
    /// Sets `controller_epoch` to the passed value.
    ///
    /// The controller epoch.
    ///
    /// Supported API versions: 0-8
    pub fn with_controller_epoch(mut self, value: i32) -> Self {
        self.controller_epoch = value;
        self
    }
    /// Sets `broker_epoch` to the passed value.
    ///
    /// The broker epoch.
    ///
    /// Supported API versions: 5-8
    pub fn with_broker_epoch(mut self, value: i64) -> Self {
        self.broker_epoch = value;
        self
    }
    /// Sets `ungrouped_partition_states` to the passed value.
    ///
    /// In older versions of this RPC, each partition that we would like to update.
    ///
    /// Supported API versions: 0-4
    pub fn with_ungrouped_partition_states(
        mut self,
        value: Vec<UpdateMetadataPartitionState>,
    ) -> Self {
        self.ungrouped_partition_states = value;
        self
    }
    /// Sets `topic_states` to the passed value.
    ///
    /// In newer versions of this RPC, each topic that we would like to update.
    ///
    /// Supported API versions: 5-8
    pub fn with_topic_states(mut self, value: Vec<UpdateMetadataTopicState>) -> Self {
        self.topic_states = value;
        self
    }
    /// Sets `live_brokers` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-8
    pub fn with_live_brokers(mut self, value: Vec<UpdateMetadataBroker>) -> Self {
        self.live_brokers = value;
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
impl Encodable for UpdateMetadataRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.controller_id)?;
        if version >= 8 {
            types::Boolean.encode(buf, &self.is_k_raft_controller)?;
        } else {
            if self.is_k_raft_controller {
                bail!("failed to encode");
            }
        }
        types::Int32.encode(buf, &self.controller_epoch)?;
        if version >= 5 {
            types::Int64.encode(buf, &self.broker_epoch)?;
        }
        if version <= 4 {
            types::Array(types::Struct { version })
                .encode(buf, &self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 5 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topic_states)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topic_states)?;
            }
        } else {
            if !self.topic_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.live_brokers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.live_brokers)?;
        }
        if version >= 6 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if version >= 8 {
                if self._type != 0 {
                    num_tagged_fields += 1;
                }
            }
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;
            if version >= 8 {
                if self._type != 0 {
                    let computed_size = types::Int8.compute_size(&self._type)?;
                    if computed_size > std::u32::MAX as usize {
                        bail!(
                            "Tagged field is too large to encode ({} bytes)",
                            computed_size
                        );
                    }
                    types::UnsignedVarInt.encode(buf, 0)?;
                    types::UnsignedVarInt.encode(buf, computed_size as u32)?;
                    types::Int8.encode(buf, &self._type)?;
                }
            }
            write_unknown_tagged_fields(buf, 1.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.controller_id)?;
        if version >= 8 {
            total_size += types::Boolean.compute_size(&self.is_k_raft_controller)?;
        } else {
            if self.is_k_raft_controller {
                bail!("failed to encode");
            }
        }
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.broker_epoch)?;
        }
        if version <= 4 {
            total_size += types::Array(types::Struct { version })
                .compute_size(&self.ungrouped_partition_states)?;
        } else {
            if !self.ungrouped_partition_states.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 5 {
            if version >= 6 {
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
        if version >= 6 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.live_brokers)?;
        } else {
            total_size +=
                types::Array(types::Struct { version }).compute_size(&self.live_brokers)?;
        }
        if version >= 6 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if version >= 8 {
                if self._type != 0 {
                    num_tagged_fields += 1;
                }
            }
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;
            if version >= 8 {
                if self._type != 0 {
                    let computed_size = types::Int8.compute_size(&self._type)?;
                    if computed_size > std::u32::MAX as usize {
                        bail!(
                            "Tagged field is too large to encode ({} bytes)",
                            computed_size
                        );
                    }
                    total_size += types::UnsignedVarInt.compute_size(0)?;
                    total_size += types::UnsignedVarInt.compute_size(computed_size as u32)?;
                    total_size += computed_size;
                }
            }
            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

#[cfg(feature = "broker")]
impl Decodable for UpdateMetadataRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let controller_id = types::Int32.decode(buf)?;
        let is_k_raft_controller = if version >= 8 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut _type = 0;
        let controller_epoch = types::Int32.decode(buf)?;
        let broker_epoch = if version >= 5 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let ungrouped_partition_states = if version <= 4 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let topic_states = if version >= 5 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let live_brokers = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                match tag {
                    0 => {
                        if version >= 8 {
                            _type = types::Int8.decode(buf)?;
                        } else {
                            bail!("Tag {} is not valid for version {}", tag, version);
                        }
                    }
                    _ => {
                        let unknown_value = buf.try_get_bytes(size as usize)?;
                        unknown_tagged_fields.insert(tag as i32, unknown_value);
                    }
                }
            }
        }
        Ok(Self {
            controller_id,
            is_k_raft_controller,
            _type,
            controller_epoch,
            broker_epoch,
            ungrouped_partition_states,
            topic_states,
            live_brokers,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateMetadataRequest {
    fn default() -> Self {
        Self {
            controller_id: (0).into(),
            is_k_raft_controller: false,
            _type: 0,
            controller_epoch: 0,
            broker_epoch: -1,
            ungrouped_partition_states: Default::default(),
            topic_states: Default::default(),
            live_brokers: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataTopicState {
    /// The topic name.
    ///
    /// Supported API versions: 5-8
    pub topic_name: super::TopicName,

    /// The topic id.
    ///
    /// Supported API versions: 7-8
    pub topic_id: Uuid,

    /// The partition that we would like to update.
    ///
    /// Supported API versions: 5-8
    pub partition_states: Vec<UpdateMetadataPartitionState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataTopicState {
    /// Sets `topic_name` to the passed value.
    ///
    /// The topic name.
    ///
    /// Supported API versions: 5-8
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `topic_id` to the passed value.
    ///
    /// The topic id.
    ///
    /// Supported API versions: 7-8
    pub fn with_topic_id(mut self, value: Uuid) -> Self {
        self.topic_id = value;
        self
    }
    /// Sets `partition_states` to the passed value.
    ///
    /// The partition that we would like to update.
    ///
    /// Supported API versions: 5-8
    pub fn with_partition_states(mut self, value: Vec<UpdateMetadataPartitionState>) -> Self {
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
impl Encodable for UpdateMetadataTopicState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 5 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.topic_name)?;
            } else {
                types::String.encode(buf, &self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 7 {
            types::Uuid.encode(buf, &self.topic_id)?;
        }
        if version >= 5 {
            if version >= 6 {
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
        if version >= 6 {
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
        if version >= 5 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.topic_name)?;
            } else {
                total_size += types::String.compute_size(&self.topic_name)?;
            }
        } else {
            if !self.topic_name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 7 {
            total_size += types::Uuid.compute_size(&self.topic_id)?;
        }
        if version >= 5 {
            if version >= 6 {
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
        if version >= 6 {
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
impl Decodable for UpdateMetadataTopicState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_name = if version >= 5 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let topic_id = if version >= 7 {
            types::Uuid.decode(buf)?
        } else {
            Uuid::nil()
        };
        let partition_states = if version >= 5 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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

impl Default for UpdateMetadataTopicState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            topic_id: Uuid::nil(),
            partition_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataTopicState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for UpdateMetadataRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            2
        } else {
            1
        }
    }
}
