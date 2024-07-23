//! AddPartitionsToTxnRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AddPartitionsToTxnRequest.json).
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

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AddPartitionsToTxnRequest {
    /// List of transactions to add partitions to.
    ///
    /// Supported API versions: 4
    pub transactions: indexmap::IndexMap<super::TransactionalId, AddPartitionsToTxnTransaction>,

    /// The transactional id corresponding to the transaction.
    ///
    /// Supported API versions: 0-3
    pub v3_and_below_transactional_id: super::TransactionalId,

    /// Current producer id in use by the transactional id.
    ///
    /// Supported API versions: 0-3
    pub v3_and_below_producer_id: super::ProducerId,

    /// Current epoch associated with the producer id.
    ///
    /// Supported API versions: 0-3
    pub v3_and_below_producer_epoch: i16,

    /// The partitions to add to the transaction.
    ///
    /// Supported API versions: 0-3
    pub v3_and_below_topics: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AddPartitionsToTxnRequest {
    type Builder = AddPartitionsToTxnRequestBuilder;

    fn builder() -> Self::Builder {
        AddPartitionsToTxnRequestBuilder::default()
    }
}

impl Encodable for AddPartitionsToTxnRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.transactions)?;
        } else {
            if !self.transactions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                types::CompactString.encode(buf, &self.v3_and_below_transactional_id)?;
            } else {
                types::String.encode(buf, &self.v3_and_below_transactional_id)?;
            }
        } else {
            if !self.v3_and_below_transactional_id.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            types::Int64.encode(buf, &self.v3_and_below_producer_id)?;
        } else {
            if self.v3_and_below_producer_id != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            types::Int16.encode(buf, &self.v3_and_below_producer_epoch)?;
        } else {
            if self.v3_and_below_producer_epoch != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                types::CompactArray(types::Struct { version })
                    .encode(buf, &self.v3_and_below_topics)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.v3_and_below_topics)?;
            }
        } else {
            if !self.v3_and_below_topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
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
        if version >= 4 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.transactions)?;
        } else {
            if !self.transactions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                total_size +=
                    types::CompactString.compute_size(&self.v3_and_below_transactional_id)?;
            } else {
                total_size += types::String.compute_size(&self.v3_and_below_transactional_id)?;
            }
        } else {
            if !self.v3_and_below_transactional_id.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            total_size += types::Int64.compute_size(&self.v3_and_below_producer_id)?;
        } else {
            if self.v3_and_below_producer_id != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            total_size += types::Int16.compute_size(&self.v3_and_below_producer_epoch)?;
        } else {
            if self.v3_and_below_producer_epoch != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                total_size += types::CompactArray(types::Struct { version })
                    .compute_size(&self.v3_and_below_topics)?;
            } else {
                total_size += types::Array(types::Struct { version })
                    .compute_size(&self.v3_and_below_topics)?;
            }
        } else {
            if !self.v3_and_below_topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
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

impl Decodable for AddPartitionsToTxnRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactions = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let v3_and_below_transactional_id = if version <= 3 {
            if version >= 3 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let v3_and_below_producer_id = if version <= 3 {
            types::Int64.decode(buf)?
        } else {
            (0).into()
        };
        let v3_and_below_producer_epoch = if version <= 3 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let v3_and_below_topics = if version <= 3 {
            if version >= 3 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            transactions,
            v3_and_below_transactional_id,
            v3_and_below_producer_id,
            v3_and_below_producer_epoch,
            v3_and_below_topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for AddPartitionsToTxnRequest {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
            v3_and_below_transactional_id: Default::default(),
            v3_and_below_producer_id: (0).into(),
            v3_and_below_producer_epoch: 0,
            v3_and_below_topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AddPartitionsToTxnTopic {
    /// The partition indexes to add to the transaction
    ///
    /// Supported API versions: 0-4
    pub partitions: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AddPartitionsToTxnTopic {
    type Builder = AddPartitionsToTxnTopicBuilder;

    fn builder() -> Self::Builder {
        AddPartitionsToTxnTopicBuilder::default()
    }
}

impl MapEncodable for AddPartitionsToTxnTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(
        &self,
        key: &Self::Key,
        buf: &mut B,
        version: i16,
    ) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 3 {
            types::CompactArray(types::Int32).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.partitions)?;
        }
        if version >= 3 {
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
        if version >= 3 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 3 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.partitions)?;
        }
        if version >= 3 {
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

impl MapDecodable for AddPartitionsToTxnTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 3 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
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

impl Default for AddPartitionsToTxnTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AddPartitionsToTxnTransaction {
    /// Current producer id in use by the transactional id.
    ///
    /// Supported API versions: 4
    pub producer_id: super::ProducerId,

    /// Current epoch associated with the producer id.
    ///
    /// Supported API versions: 4
    pub producer_epoch: i16,

    /// Boolean to signify if we want to check if the partition is in the transaction rather than add it.
    ///
    /// Supported API versions: 4
    pub verify_only: bool,

    /// The partitions to add to the transaction.
    ///
    /// Supported API versions: 4
    pub topics: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AddPartitionsToTxnTransaction {
    type Builder = AddPartitionsToTxnTransactionBuilder;

    fn builder() -> Self::Builder {
        AddPartitionsToTxnTransactionBuilder::default()
    }
}

impl MapEncodable for AddPartitionsToTxnTransaction {
    type Key = super::TransactionalId;
    fn encode<B: ByteBufMut>(
        &self,
        key: &Self::Key,
        buf: &mut B,
        version: i16,
    ) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, key)?;
        } else {
            if !key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Int64.encode(buf, &self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Int16.encode(buf, &self.producer_epoch)?;
        } else {
            if self.producer_epoch != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Boolean.encode(buf, &self.verify_only)?;
        } else {
            if self.verify_only {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
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
        if version >= 4 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            if !key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Int16.compute_size(&self.producer_epoch)?;
        } else {
            if self.producer_epoch != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Boolean.compute_size(&self.verify_only)?;
        } else {
            if self.verify_only {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
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

impl MapDecodable for AddPartitionsToTxnTransaction {
    type Key = super::TransactionalId;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let producer_id = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            (0).into()
        };
        let producer_epoch = if version >= 4 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let verify_only = if version >= 4 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let topics = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
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
                producer_id,
                producer_epoch,
                verify_only,
                topics,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AddPartitionsToTxnTransaction {
    fn default() -> Self {
        Self {
            producer_id: (0).into(),
            producer_epoch: 0,
            verify_only: false,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnTransaction {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for AddPartitionsToTxnRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            2
        } else {
            1
        }
    }
}
