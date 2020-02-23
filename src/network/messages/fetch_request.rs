//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::{Buf, BufMut};
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size,
};


/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-11
    pub partition_index: i32,

    /// The current leader epoch of the partition.
    /// 
    /// Supported API versions: 9-11
    pub current_leader_epoch: i32,

    /// The message offset.
    /// 
    /// Supported API versions: 0-11
    pub fetch_offset: i64,

    /// The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
    /// 
    /// Supported API versions: 5-11
    pub log_start_offset: i64,

    /// The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
    /// 
    /// Supported API versions: 0-11
    pub max_bytes: i32,

}

impl Encodable for FetchPartition {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        if version >= 9 {
            types::Int32.encode(buf, &self.current_leader_epoch)?;
        }
        types::Int64.encode(buf, &self.fetch_offset)?;
        if version >= 5 {
            types::Int64.encode(buf, &self.log_start_offset)?;
        } else {
            if self.log_start_offset != -1 {
                return Err(EncodeError)
            }
        }
        types::Int32.encode(buf, &self.max_bytes)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        if version >= 9 {
            total_size += types::Int32.compute_size(&self.current_leader_epoch)?;
        }
        total_size += types::Int64.compute_size(&self.fetch_offset)?;
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.log_start_offset)?;
        } else {
            if self.log_start_offset != -1 {
                return Err(EncodeError)
            }
        }
        total_size += types::Int32.compute_size(&self.max_bytes)?;

        Ok(total_size)
    }
}

impl Decodable for FetchPartition {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let current_leader_epoch = if version >= 9 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let fetch_offset = types::Int64.decode(buf)?;
        let log_start_offset = if version >= 5 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let max_bytes = types::Int32.decode(buf)?;
        Ok(Self {
            partition_index,
            current_leader_epoch,
            fetch_offset,
            log_start_offset,
            max_bytes,
        })
    }
}

impl Default for FetchPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            current_leader_epoch: -1,
            fetch_offset: 0,
            log_start_offset: -1,
            max_bytes: 0,
        }
    }
}

impl Message for FetchPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchableTopic {
    /// The name of the topic to fetch.
    /// 
    /// Supported API versions: 0-11
    pub name: super::TopicName,

    /// The partitions to fetch.
    /// 
    /// Supported API versions: 0-11
    pub fetch_partitions: Vec<FetchPartition>,

}

impl Encodable for FetchableTopic {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::Array(types::Struct { version }).encode(buf, &self.fetch_partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.fetch_partitions)?;

        Ok(total_size)
    }
}

impl Decodable for FetchableTopic {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let fetch_partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            fetch_partitions,
        })
    }
}

impl Default for FetchableTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            fetch_partitions: Default::default(),
        }
    }
}

impl Message for FetchableTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct ForgottenTopic {
    /// The partition name.
    /// 
    /// Supported API versions: 7-11
    pub name: super::TopicName,

    /// The partitions indexes to forget.
    /// 
    /// Supported API versions: 7-11
    pub forgotten_partition_indexes: Vec<i32>,

}

impl Encodable for ForgottenTopic {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 7 {
            types::String.encode(buf, &self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            types::Array(types::Int32).encode(buf, &self.forgotten_partition_indexes)?;
        } else {
            if !self.forgotten_partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 7 {
            total_size += types::String.compute_size(&self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            total_size += types::Array(types::Int32).compute_size(&self.forgotten_partition_indexes)?;
        } else {
            if !self.forgotten_partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for ForgottenTopic {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 7 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let forgotten_partition_indexes = if version >= 7 {
            types::Array(types::Int32).decode(buf)?
        } else {
            Default::default()
        };
        Ok(Self {
            name,
            forgotten_partition_indexes,
        })
    }
}

impl Default for ForgottenTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            forgotten_partition_indexes: Default::default(),
        }
    }
}

impl Message for ForgottenTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

/// Valid versions: 0-11
#[derive(Debug, Clone)]
pub struct FetchRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    /// 
    /// Supported API versions: 0-11
    pub replica_id: i32,

    /// The maximum time in milliseconds to wait for the response.
    /// 
    /// Supported API versions: 0-11
    pub max_wait: i32,

    /// The minimum bytes to accumulate in the response.
    /// 
    /// Supported API versions: 0-11
    pub min_bytes: i32,

    /// The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
    /// 
    /// Supported API versions: 3-11
    pub max_bytes: i32,

    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    /// 
    /// Supported API versions: 4-11
    pub isolation_level: i8,

    /// The fetch session ID.
    /// 
    /// Supported API versions: 7-11
    pub session_id: i32,

    /// The epoch of the partition leader as known to the follower replica or a consumer.
    /// 
    /// Supported API versions: 7-11
    pub epoch: i32,

    /// The topics to fetch.
    /// 
    /// Supported API versions: 0-11
    pub topics: Vec<FetchableTopic>,

    /// In an incremental fetch request, the partitions to remove.
    /// 
    /// Supported API versions: 7-11
    pub forgotten: Vec<ForgottenTopic>,

    /// Rack ID of the consumer making this request
    /// 
    /// Supported API versions: 11
    pub rack_id: String,

}

impl Encodable for FetchRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.replica_id)?;
        types::Int32.encode(buf, &self.max_wait)?;
        types::Int32.encode(buf, &self.min_bytes)?;
        if version >= 3 {
            types::Int32.encode(buf, &self.max_bytes)?;
        }
        if version >= 4 {
            types::Int8.encode(buf, &self.isolation_level)?;
        } else {
            if self.isolation_level != 0 {
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
        if version >= 7 {
            types::Int32.encode(buf, &self.epoch)?;
        } else {
            if self.epoch != -1 {
                return Err(EncodeError)
            }
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        if version >= 7 {
            types::Array(types::Struct { version }).encode(buf, &self.forgotten)?;
        } else {
            if !self.forgotten.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 11 {
            types::String.encode(buf, &self.rack_id)?;
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.replica_id)?;
        total_size += types::Int32.compute_size(&self.max_wait)?;
        total_size += types::Int32.compute_size(&self.min_bytes)?;
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.max_bytes)?;
        }
        if version >= 4 {
            total_size += types::Int8.compute_size(&self.isolation_level)?;
        } else {
            if self.isolation_level != 0 {
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
        if version >= 7 {
            total_size += types::Int32.compute_size(&self.epoch)?;
        } else {
            if self.epoch != -1 {
                return Err(EncodeError)
            }
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        if version >= 7 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.forgotten)?;
        } else {
            if !self.forgotten.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 11 {
            total_size += types::String.compute_size(&self.rack_id)?;
        }

        Ok(total_size)
    }
}

impl Decodable for FetchRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let replica_id = types::Int32.decode(buf)?;
        let max_wait = types::Int32.decode(buf)?;
        let min_bytes = types::Int32.decode(buf)?;
        let max_bytes = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            0x7fffffff
        };
        let isolation_level = if version >= 4 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let session_id = if version >= 7 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let epoch = if version >= 7 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        let forgotten = if version >= 7 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let rack_id = if version == 11 {
            types::String.decode(buf)?
        } else {
            "".to_owned()
        };
        Ok(Self {
            replica_id,
            max_wait,
            min_bytes,
            max_bytes,
            isolation_level,
            session_id,
            epoch,
            topics,
            forgotten,
            rack_id,
        })
    }
}

impl Default for FetchRequest {
    fn default() -> Self {
        Self {
            replica_id: 0,
            max_wait: 0,
            min_bytes: 0,
            max_bytes: 0x7fffffff,
            isolation_level: 0,
            session_id: 0,
            epoch: -1,
            topics: Default::default(),
            forgotten: Default::default(),
            rack_id: "".to_owned(),
        }
    }
}

impl Message for FetchRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 11 };
}

impl HeaderVersion for FetchRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

