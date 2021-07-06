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


/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct AbortedTransaction {
    /// The producer id associated with the aborted transaction.
    /// 
    /// Supported API versions: 4-11
    pub producer_id: super::ProducerId,

    /// The first offset in the aborted transaction.
    /// 
    /// Supported API versions: 4-11
    pub first_offset: i64,

}

impl Encodable for AbortedTransaction {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::Int64.encode(buf, &self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            types::Int64.encode(buf, &self.first_offset)?;
        } else {
            if self.first_offset != 0 {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.first_offset)?;
        } else {
            if self.first_offset != 0 {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for AbortedTransaction {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let producer_id = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            (0).into()
        };
        let first_offset = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            0
        };
        Ok(Self {
            producer_id,
            first_offset,
        })
    }
}

impl Default for AbortedTransaction {
    fn default() -> Self {
        Self {
            producer_id: (0).into(),
            first_offset: 0,
        }
    }
}

impl Message for AbortedTransaction {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchablePartitionResponse {
    /// The partiiton index.
    /// 
    /// Supported API versions: 0-11
    pub partition_index: i32,

    /// The error code, or 0 if there was no fetch error.
    /// 
    /// Supported API versions: 0-11
    pub error_code: i16,

    /// The current high water mark.
    /// 
    /// Supported API versions: 0-11
    pub high_watermark: i64,

    /// The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
    /// 
    /// Supported API versions: 4-11
    pub last_stable_offset: i64,

    /// The current log start offset.
    /// 
    /// Supported API versions: 5-11
    pub log_start_offset: i64,

    /// The aborted transactions.
    /// 
    /// Supported API versions: 4-11
    pub aborted: Option<Vec<AbortedTransaction>>,

    /// The preferred read replica for the consumer to use on its next fetch request
    /// 
    /// Supported API versions: 11
    pub preferred_read_replica: i32,

    /// The record data.
    /// 
    /// Supported API versions: 0-11
    pub records: Option<Bytes>,

}

impl Encodable for FetchablePartitionResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int64.encode(buf, &self.high_watermark)?;
        if version >= 4 {
            types::Int64.encode(buf, &self.last_stable_offset)?;
        }
        if version >= 5 {
            types::Int64.encode(buf, &self.log_start_offset)?;
        }
        if version >= 4 {
            types::Array(types::Struct { version }).encode(buf, &self.aborted)?;
        } else {
            if !self.aborted.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version == 11 {
            types::Int32.encode(buf, &self.preferred_read_replica)?;
        }
        types::Bytes.encode(buf, &self.records)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int64.compute_size(&self.high_watermark)?;
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.last_stable_offset)?;
        }
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.log_start_offset)?;
        }
        if version >= 4 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.aborted)?;
        } else {
            if !self.aborted.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version == 11 {
            total_size += types::Int32.compute_size(&self.preferred_read_replica)?;
        }
        total_size += types::Bytes.compute_size(&self.records)?;

        Ok(total_size)
    }
}

impl Decodable for FetchablePartitionResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let high_watermark = types::Int64.decode(buf)?;
        let last_stable_offset = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let log_start_offset = if version >= 5 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let aborted = if version >= 4 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Some(Default::default())
        };
        let preferred_read_replica = if version == 11 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let records = types::Bytes.decode(buf)?;
        Ok(Self {
            partition_index,
            error_code,
            high_watermark,
            last_stable_offset,
            log_start_offset,
            aborted,
            preferred_read_replica,
            records,
        })
    }
}

impl Default for FetchablePartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            high_watermark: 0,
            last_stable_offset: -1,
            log_start_offset: -1,
            aborted: Some(Default::default()),
            preferred_read_replica: 0,
            records: Some(Default::default()),
        }
    }
}

impl Message for FetchablePartitionResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchableTopicResponse {
    /// The topic name.
    /// 
    /// Supported API versions: 0-11
    pub name: super::TopicName,

    /// The topic partitions.
    /// 
    /// Supported API versions: 0-11
    pub partitions: Vec<FetchablePartitionResponse>,

}

impl Encodable for FetchableTopicResponse {
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

impl Decodable for FetchableTopicResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for FetchableTopicResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for FetchableTopicResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-11
    pub throttle_time_ms: i32,

    /// The top level response error code.
    /// 
    /// Supported API versions: 7-11
    pub error_code: i16,

    /// The fetch session ID, or 0 if this is not part of a fetch session.
    /// 
    /// Supported API versions: 7-11
    pub session_id: i32,

    /// The response topics.
    /// 
    /// Supported API versions: 0-11
    pub topics: Vec<FetchableTopicResponse>,

}

impl Encodable for FetchResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version >= 7 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            types::Int32.encode(buf, &self.session_id)?;
        } else {
            if self.session_id != 0 {
                return Err(EncodeError)
            }
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version >= 7 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            total_size += types::Int32.compute_size(&self.session_id)?;
        } else {
            if self.session_id != 0 {
                return Err(EncodeError)
            }
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for FetchResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = if version >= 7 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let session_id = if version >= 7 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            error_code,
            session_id,
            topics,
        })
    }
}

impl Default for FetchResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            session_id: 0,
            topics: Default::default(),
        }
    }
}

impl Message for FetchResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

impl HeaderVersion for FetchResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

