//! AlterReplicaLogDirsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AlterReplicaLogDirsRequest.json).
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

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AlterReplicaLogDir {
    /// The topics to add to the directory.
    ///
    /// Supported API versions: 0-2
    pub topics: indexmap::IndexMap<super::TopicName, AlterReplicaLogDirTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AlterReplicaLogDir {
    /// Sets `topics` to the passed value.
    ///
    /// The topics to add to the directory.
    ///
    /// Supported API versions: 0-2
    pub fn with_topics(
        mut self,
        value: indexmap::IndexMap<super::TopicName, AlterReplicaLogDirTopic>,
    ) -> Self {
        self.topics = value;
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

impl MapEncodable for AlterReplicaLogDir {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        if version >= 2 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 2 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 2 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 2 {
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

impl MapDecodable for AlterReplicaLogDir {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let topics = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((
            key_field,
            Self {
                topics,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AlterReplicaLogDir {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterReplicaLogDir {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = Some(VersionRange { min: 0, max: 0 });
}

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AlterReplicaLogDirTopic {
    /// The partition indexes.
    ///
    /// Supported API versions: 0-2
    pub partitions: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AlterReplicaLogDirTopic {
    /// Sets `partitions` to the passed value.
    ///
    /// The partition indexes.
    ///
    /// Supported API versions: 0-2
    pub fn with_partitions(mut self, value: Vec<i32>) -> Self {
        self.partitions = value;
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

impl MapEncodable for AlterReplicaLogDirTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        if version >= 2 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 2 {
            types::CompactArray(types::Int32).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.partitions)?;
        }
        if version >= 2 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 2 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.partitions)?;
        }
        if version >= 2 {
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

impl MapDecodable for AlterReplicaLogDirTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 2 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((
            key_field,
            Self {
                partitions,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AlterReplicaLogDirTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterReplicaLogDirTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = Some(VersionRange { min: 0, max: 0 });
}

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    ///
    /// Supported API versions: 0-2
    pub dirs: indexmap::IndexMap<StrBytes, AlterReplicaLogDir>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AlterReplicaLogDirsRequest {
    /// Sets `dirs` to the passed value.
    ///
    /// The alterations to make for each directory.
    ///
    /// Supported API versions: 0-2
    pub fn with_dirs(mut self, value: indexmap::IndexMap<StrBytes, AlterReplicaLogDir>) -> Self {
        self.dirs = value;
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

impl Encodable for AlterReplicaLogDirsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.dirs)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.dirs)?;
        }
        if version >= 2 {
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
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.dirs)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.dirs)?;
        }
        if version >= 2 {
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

impl Decodable for AlterReplicaLogDirsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let dirs = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            dirs,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterReplicaLogDirsRequest {
    fn default() -> Self {
        Self {
            dirs: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterReplicaLogDirsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = Some(VersionRange { min: 0, max: 0 });
}

impl HeaderVersion for AlterReplicaLogDirsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}
