//! ElectLeadersRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ElectLeadersRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ElectLeadersRequest {
    /// Type of elections to conduct for the partition. A value of '0' elects the preferred replica. A value of '1' elects the first live replica if there are no in-sync replica.
    ///
    /// Supported API versions: 1-2
    pub election_type: i8,

    /// The topic partitions to elect leaders.
    ///
    /// Supported API versions: 0-2
    pub topic_partitions: Option<indexmap::IndexMap<super::TopicName, TopicPartitions>>,

    /// The time in ms to wait for the election to complete.
    ///
    /// Supported API versions: 0-2
    pub timeout_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ElectLeadersRequest {
    type Builder = ElectLeadersRequestBuilder;

    fn builder() -> Self::Builder {
        ElectLeadersRequestBuilder::default()
    }
}

impl Encodable for ElectLeadersRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int8.encode(buf, &self.election_type)?;
        } else {
            if self.election_type != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topic_partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topic_partitions)?;
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
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
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.election_type)?;
        } else {
            if self.election_type != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version })
                .compute_size(&self.topic_partitions)?;
        } else {
            total_size +=
                types::Array(types::Struct { version }).compute_size(&self.topic_partitions)?;
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
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

impl Decodable for ElectLeadersRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let election_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let topic_partitions = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let timeout_ms = types::Int32.decode(buf)?;
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
            election_type,
            topic_partitions,
            timeout_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ElectLeadersRequest {
    fn default() -> Self {
        Self {
            election_type: 0,
            topic_partitions: Some(Default::default()),
            timeout_ms: 60000,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ElectLeadersRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TopicPartitions {
    /// The partitions of this topic whose leader should be elected.
    ///
    /// Supported API versions: 0-2
    pub partitions: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for TopicPartitions {
    type Builder = TopicPartitionsBuilder;

    fn builder() -> Self::Builder {
        TopicPartitionsBuilder::default()
    }
}

impl MapEncodable for TopicPartitions {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(
        &self,
        key: &Self::Key,
        buf: &mut B,
        version: i16,
    ) -> Result<(), EncodeError> {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
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

impl MapDecodable for TopicPartitions {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
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

impl Default for TopicPartitions {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicPartitions {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for ElectLeadersRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}
