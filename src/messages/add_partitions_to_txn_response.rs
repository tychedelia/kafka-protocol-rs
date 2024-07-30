//! AddPartitionsToTxnResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AddPartitionsToTxnResponse.json).
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

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The response error code.
    ///
    /// Supported API versions: 0-4
    pub partition_error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AddPartitionsToTxnPartitionResult {
    /// Sets `partition_error_code` to the passed value.
    ///
    /// The response error code.
    ///
    /// Supported API versions: 0-4
    pub fn with_partition_error_code(mut self, value: i16) -> Self {
        self.partition_error_code = value;
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

impl MapEncodable for AddPartitionsToTxnPartitionResult {
    type Key = i32;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, key)?;
        types::Int16.encode(buf, &self.partition_error_code)?;
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.partition_error_code)?;
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

impl MapDecodable for AddPartitionsToTxnPartitionResult {
    type Key = i32;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = types::Int32.decode(buf)?;
        let partition_error_code = types::Int16.decode(buf)?;
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
                partition_error_code,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AddPartitionsToTxnPartitionResult {
    fn default() -> Self {
        Self {
            partition_error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0-4
    pub throttle_time_ms: i32,

    /// The response top level error code.
    ///
    /// Supported API versions: 4
    pub error_code: i16,

    /// Results categorized by transactional ID.
    ///
    /// Supported API versions: 4
    pub results_by_transaction:
        indexmap::IndexMap<super::TransactionalId, AddPartitionsToTxnResult>,

    /// The results for each topic.
    ///
    /// Supported API versions: 0-3
    pub results_by_topic_v3_and_below:
        indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopicResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AddPartitionsToTxnResponse {
    /// Sets `throttle_time_ms` to the passed value.
    ///
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0-4
    pub fn with_throttle_time_ms(mut self, value: i32) -> Self {
        self.throttle_time_ms = value;
        self
    }
    /// Sets `error_code` to the passed value.
    ///
    /// The response top level error code.
    ///
    /// Supported API versions: 4
    pub fn with_error_code(mut self, value: i16) -> Self {
        self.error_code = value;
        self
    }
    /// Sets `results_by_transaction` to the passed value.
    ///
    /// Results categorized by transactional ID.
    ///
    /// Supported API versions: 4
    pub fn with_results_by_transaction(
        mut self,
        value: indexmap::IndexMap<super::TransactionalId, AddPartitionsToTxnResult>,
    ) -> Self {
        self.results_by_transaction = value;
        self
    }
    /// Sets `results_by_topic_v3_and_below` to the passed value.
    ///
    /// The results for each topic.
    ///
    /// Supported API versions: 0-3
    pub fn with_results_by_topic_v3_and_below(
        mut self,
        value: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopicResult>,
    ) -> Self {
        self.results_by_topic_v3_and_below = value;
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

impl Encodable for AddPartitionsToTxnResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        if version >= 4 {
            types::Int16.encode(buf, &self.error_code)?;
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version })
                .encode(buf, &self.results_by_transaction)?;
        } else {
            if !self.results_by_transaction.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                types::CompactArray(types::Struct { version })
                    .encode(buf, &self.results_by_topic_v3_and_below)?;
            } else {
                types::Array(types::Struct { version })
                    .encode(buf, &self.results_by_topic_v3_and_below)?;
            }
        } else {
            if !self.results_by_topic_v3_and_below.is_empty() {
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
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        if version >= 4 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        }
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version })
                .compute_size(&self.results_by_transaction)?;
        } else {
            if !self.results_by_transaction.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                total_size += types::CompactArray(types::Struct { version })
                    .compute_size(&self.results_by_topic_v3_and_below)?;
            } else {
                total_size += types::Array(types::Struct { version })
                    .compute_size(&self.results_by_topic_v3_and_below)?;
            }
        } else {
            if !self.results_by_topic_v3_and_below.is_empty() {
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

impl Decodable for AddPartitionsToTxnResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = if version >= 4 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let results_by_transaction = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let results_by_topic_v3_and_below = if version <= 3 {
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
            throttle_time_ms,
            error_code,
            results_by_transaction,
            results_by_topic_v3_and_below,
            unknown_tagged_fields,
        })
    }
}

impl Default for AddPartitionsToTxnResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            results_by_transaction: Default::default(),
            results_by_topic_v3_and_below: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPartitionsToTxnResult {
    /// The results for each topic.
    ///
    /// Supported API versions: 4
    pub topic_results: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopicResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AddPartitionsToTxnResult {
    /// Sets `topic_results` to the passed value.
    ///
    /// The results for each topic.
    ///
    /// Supported API versions: 4
    pub fn with_topic_results(
        mut self,
        value: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopicResult>,
    ) -> Self {
        self.topic_results = value;
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

impl MapEncodable for AddPartitionsToTxnResult {
    type Key = super::TransactionalId;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        if version >= 4 {
            types::CompactString.encode(buf, key)?;
        } else {
            if !key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topic_results)?;
        } else {
            if !self.topic_results.is_empty() {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 4 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            if !key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.topic_results)?;
        } else {
            if !self.topic_results.is_empty() {
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

impl MapDecodable for AddPartitionsToTxnResult {
    type Key = super::TransactionalId;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let topic_results = if version >= 4 {
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
                topic_results,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AddPartitionsToTxnResult {
    fn default() -> Self {
        Self {
            topic_results: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPartitionsToTxnTopicResult {
    /// The results for each partition
    ///
    /// Supported API versions: 0-4
    pub results_by_partition: indexmap::IndexMap<i32, AddPartitionsToTxnPartitionResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl AddPartitionsToTxnTopicResult {
    /// Sets `results_by_partition` to the passed value.
    ///
    /// The results for each partition
    ///
    /// Supported API versions: 0-4
    pub fn with_results_by_partition(
        mut self,
        value: indexmap::IndexMap<i32, AddPartitionsToTxnPartitionResult>,
    ) -> Self {
        self.results_by_partition = value;
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

impl MapEncodable for AddPartitionsToTxnTopicResult {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        if version >= 3 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 3 {
            types::CompactArray(types::Struct { version })
                .encode(buf, &self.results_by_partition)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.results_by_partition)?;
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 3 {
            total_size += types::CompactArray(types::Struct { version })
                .compute_size(&self.results_by_partition)?;
        } else {
            total_size +=
                types::Array(types::Struct { version }).compute_size(&self.results_by_partition)?;
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

impl MapDecodable for AddPartitionsToTxnTopicResult {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let results_by_partition = if version >= 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
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
                results_by_partition,
                unknown_tagged_fields,
            },
        ))
    }
}

impl Default for AddPartitionsToTxnTopicResult {
    fn default() -> Self {
        Self {
            results_by_partition: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AddPartitionsToTxnTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for AddPartitionsToTxnResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            1
        } else {
            0
        }
    }
}
