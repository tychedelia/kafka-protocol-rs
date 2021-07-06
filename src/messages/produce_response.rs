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


/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct BatchIndexAndErrorMessage {
    /// The batch index of the record that cause the batch to be dropped
    /// 
    /// Supported API versions: 8
    pub batch_index: i32,

    /// The error message of the record that caused the batch to be dropped
    /// 
    /// Supported API versions: 8
    pub batch_index_error_message: Option<StrBytes>,

}

impl Encodable for BatchIndexAndErrorMessage {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 8 {
            types::Int32.encode(buf, &self.batch_index)?;
        } else {
            if self.batch_index != 0 {
                return Err(EncodeError)
            }
        }
        if version == 8 {
            types::String.encode(buf, &self.batch_index_error_message)?;
        } else {
            if !self.batch_index_error_message.is_none() {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version == 8 {
            total_size += types::Int32.compute_size(&self.batch_index)?;
        } else {
            if self.batch_index != 0 {
                return Err(EncodeError)
            }
        }
        if version == 8 {
            total_size += types::String.compute_size(&self.batch_index_error_message)?;
        } else {
            if !self.batch_index_error_message.is_none() {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for BatchIndexAndErrorMessage {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let batch_index = if version == 8 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let batch_index_error_message = if version == 8 {
            types::String.decode(buf)?
        } else {
            None
        };
        Ok(Self {
            batch_index,
            batch_index_error_message,
        })
    }
}

impl Default for BatchIndexAndErrorMessage {
    fn default() -> Self {
        Self {
            batch_index: 0,
            batch_index_error_message: None,
        }
    }
}

impl Message for BatchIndexAndErrorMessage {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct PartitionProduceResponse {
    /// The partition index.
    /// 
    /// Supported API versions: 0-8
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-8
    pub error_code: i16,

    /// The base offset.
    /// 
    /// Supported API versions: 0-8
    pub base_offset: i64,

    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    /// 
    /// Supported API versions: 2-8
    pub log_append_time_ms: i64,

    /// The log start offset.
    /// 
    /// Supported API versions: 5-8
    pub log_start_offset: i64,

    /// The batch indices of records that caused the batch to be dropped
    /// 
    /// Supported API versions: 8
    pub record_errors: Vec<BatchIndexAndErrorMessage>,

    /// The global error message summarizing the common root cause of the records that caused the batch to be dropped
    /// 
    /// Supported API versions: 8
    pub error_message: Option<StrBytes>,

}

impl Encodable for PartitionProduceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int64.encode(buf, &self.base_offset)?;
        if version >= 2 {
            types::Int64.encode(buf, &self.log_append_time_ms)?;
        }
        if version >= 5 {
            types::Int64.encode(buf, &self.log_start_offset)?;
        }
        if version == 8 {
            types::Array(types::Struct { version }).encode(buf, &self.record_errors)?;
        }
        if version == 8 {
            types::String.encode(buf, &self.error_message)?;
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int64.compute_size(&self.base_offset)?;
        if version >= 2 {
            total_size += types::Int64.compute_size(&self.log_append_time_ms)?;
        }
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.log_start_offset)?;
        }
        if version == 8 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.record_errors)?;
        }
        if version == 8 {
            total_size += types::String.compute_size(&self.error_message)?;
        }

        Ok(total_size)
    }
}

impl Decodable for PartitionProduceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
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
        let record_errors = if version == 8 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let error_message = if version == 8 {
            types::String.decode(buf)?
        } else {
            None
        };
        Ok(Self {
            partition_index,
            error_code,
            base_offset,
            log_append_time_ms,
            log_start_offset,
            record_errors,
            error_message,
        })
    }
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            base_offset: 0,
            log_append_time_ms: -1,
            log_start_offset: -1,
            record_errors: Default::default(),
            error_message: None,
        }
    }
}

impl Message for PartitionProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct TopicProduceResponse {
    /// The topic name
    /// 
    /// Supported API versions: 0-8
    pub name: super::TopicName,

    /// Each partition that we produced to within the topic.
    /// 
    /// Supported API versions: 0-8
    pub partitions: Vec<PartitionProduceResponse>,

}

impl Encodable for TopicProduceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl Decodable for TopicProduceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for TopicProduceResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for TopicProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct ProduceResponse {
    /// Each produce response
    /// 
    /// Supported API versions: 0-8
    pub responses: Vec<TopicProduceResponse>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-8
    pub throttle_time_ms: i32,

}

impl Encodable for ProduceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.responses)?;
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.responses)?;
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }

        Ok(total_size)
    }
}

impl Decodable for ProduceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let responses = types::Array(types::Struct { version }).decode(buf)?;
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        Ok(Self {
            responses,
            throttle_time_ms,
        })
    }
}

impl Default for ProduceResponse {
    fn default() -> Self {
        Self {
            responses: Default::default(),
            throttle_time_ms: 0,
        }
    }
}

impl Message for ProduceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

impl HeaderVersion for ProduceResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

