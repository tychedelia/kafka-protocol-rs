//! ProduceResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use uuid::Uuid;
use anyhow::bail;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct BatchIndexAndErrorMessage {
    /// The batch index of the record that cause the batch to be dropped
    /// 
    /// Supported API versions: 8-9
    pub batch_index: i32,

    /// The error message of the record that caused the batch to be dropped
    /// 
    /// Supported API versions: 8-9
    pub batch_index_error_message: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for BatchIndexAndErrorMessage {
    type Builder = BatchIndexAndErrorMessageBuilder;

    fn builder() -> Self::Builder{
        BatchIndexAndErrorMessageBuilder::default()
    }
}

impl Encodable for BatchIndexAndErrorMessage {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 8 {
            types::Int32.encode(buf, &self.batch_index)?;
        } else {
            if self.batch_index != 0 {
                bail!("failed to decode");
            }
        }
        if version >= 8 {
            if version >= 9 {
                types::CompactString.encode(buf, &self.batch_index_error_message)?;
            } else {
                types::String.encode(buf, &self.batch_index_error_message)?;
            }
        } else {
            if !self.batch_index_error_message.is_none() {
                bail!("failed to decode");
            }
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::Int32.compute_size(&self.batch_index)?;
        } else {
            if self.batch_index != 0 {
                bail!("failed to decode");
            }
        }
        if version >= 8 {
            if version >= 9 {
                total_size += types::CompactString.compute_size(&self.batch_index_error_message)?;
            } else {
                total_size += types::String.compute_size(&self.batch_index_error_message)?;
            }
        } else {
            if !self.batch_index_error_message.is_none() {
                bail!("failed to decode");
            }
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for BatchIndexAndErrorMessage {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let batch_index = if version >= 8 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let batch_index_error_message = if version >= 8 {
            if version >= 9 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            batch_index,
            batch_index_error_message,
            unknown_tagged_fields,
        })
    }
}

impl Default for BatchIndexAndErrorMessage {
    fn default() -> Self {
        Self {
            batch_index: 0,
            batch_index_error_message: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for BatchIndexAndErrorMessage {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct PartitionProduceResponse {
    /// The partition index.
    /// 
    /// Supported API versions: 0-9
    pub index: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-9
    pub error_code: i16,

    /// The base offset.
    /// 
    /// Supported API versions: 0-9
    pub base_offset: i64,

    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    /// 
    /// Supported API versions: 2-9
    pub log_append_time_ms: i64,

    /// The log start offset.
    /// 
    /// Supported API versions: 5-9
    pub log_start_offset: i64,

    /// The batch indices of records that caused the batch to be dropped
    /// 
    /// Supported API versions: 8-9
    pub record_errors: Vec<BatchIndexAndErrorMessage>,

    /// The global error message summarizing the common root cause of the records that caused the batch to be dropped
    /// 
    /// Supported API versions: 8-9
    pub error_message: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for PartitionProduceResponse {
    type Builder = PartitionProduceResponseBuilder;

    fn builder() -> Self::Builder{
        PartitionProduceResponseBuilder::default()
    }
}

impl Encodable for PartitionProduceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int64.encode(buf, &self.base_offset)?;
        if version >= 2 {
            types::Int64.encode(buf, &self.log_append_time_ms)?;
        }
        if version >= 5 {
            types::Int64.encode(buf, &self.log_start_offset)?;
        }
        if version >= 8 {
            if version >= 9 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.record_errors)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.record_errors)?;
            }
        }
        if version >= 8 {
            if version >= 9 {
                types::CompactString.encode(buf, &self.error_message)?;
            } else {
                types::String.encode(buf, &self.error_message)?;
            }
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int64.compute_size(&self.base_offset)?;
        if version >= 2 {
            total_size += types::Int64.compute_size(&self.log_append_time_ms)?;
        }
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.log_start_offset)?;
        }
        if version >= 8 {
            if version >= 9 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.record_errors)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.record_errors)?;
            }
        }
        if version >= 8 {
            if version >= 9 {
                total_size += types::CompactString.compute_size(&self.error_message)?;
            } else {
                total_size += types::String.compute_size(&self.error_message)?;
            }
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for PartitionProduceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let base_offset = types::Int64.decode(buf)?;
        let log_append_time_ms = if version >= 2 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let log_start_offset = if version >= 5 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let record_errors = if version >= 8 {
            if version >= 9 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let error_message = if version >= 8 {
            if version >= 9 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            index,
            error_code,
            base_offset,
            log_append_time_ms,
            log_start_offset,
            record_errors,
            error_message,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        Self {
            index: 0,
            error_code: 0,
            base_offset: 0,
            log_append_time_ms: -1,
            log_start_offset: -1,
            record_errors: Default::default(),
            error_message: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TopicProduceResponse {
    /// Each partition that we produced to within the topic.
    /// 
    /// Supported API versions: 0-9
    pub partition_responses: Vec<PartitionProduceResponse>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for TopicProduceResponse {
    type Builder = TopicProduceResponseBuilder;

    fn builder() -> Self::Builder{
        TopicProduceResponseBuilder::default()
    }
}

impl MapEncodable for TopicProduceResponse {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 9 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partition_responses)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partition_responses)?;
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 9 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_responses)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partition_responses)?;
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl MapDecodable for TopicProduceResponse {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 9 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partition_responses = if version >= 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            partition_responses,
            unknown_tagged_fields,
        }))
    }
}

impl Default for TopicProduceResponse {
    fn default() -> Self {
        Self {
            partition_responses: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ProduceResponse {
    /// Each produce response
    /// 
    /// Supported API versions: 0-9
    pub responses: indexmap::IndexMap<super::TopicName, TopicProduceResponse>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-9
    pub throttle_time_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ProduceResponse {
    type Builder = ProduceResponseBuilder;

    fn builder() -> Self::Builder{
        ProduceResponseBuilder::default()
    }
}

impl Encodable for ProduceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.responses)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.responses)?;
        }
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.responses)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.responses)?;
        }
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version >= 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for ProduceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let responses = if version >= 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            responses,
            throttle_time_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ProduceResponse {
    fn default() -> Self {
        Self {
            responses: Default::default(),
            throttle_time_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

impl HeaderVersion for ProduceResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 9 {
            1
        } else {
            0
        }
    }
}

