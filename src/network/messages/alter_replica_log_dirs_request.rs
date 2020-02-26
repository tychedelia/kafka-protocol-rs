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


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterReplicaLogDirTopic {
    /// The partition indexes.
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<i32>,

}

impl MapEncodable for AlterReplicaLogDirTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Int32).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Int32).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl MapDecodable for AlterReplicaLogDirTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Int32).decode(buf)?;
        Ok((key_field, Self {
            partitions,
        }))
    }
}

impl Default for AlterReplicaLogDirTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for AlterReplicaLogDirTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterReplicaLogDir {
    /// The topics to add to the directory.
    /// 
    /// Supported API versions: 0-1
    pub topics: indexmap::IndexMap<super::TopicName, AlterReplicaLogDirTopic>,

}

impl MapEncodable for AlterReplicaLogDir {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl MapDecodable for AlterReplicaLogDir {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok((key_field, Self {
            topics,
        }))
    }
}

impl Default for AlterReplicaLogDir {
    fn default() -> Self {
        Self {
            topics: Default::default(),
        }
    }
}

impl Message for AlterReplicaLogDir {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    /// 
    /// Supported API versions: 0-1
    pub dirs: indexmap::IndexMap<StrBytes, AlterReplicaLogDir>,

}

impl Encodable for AlterReplicaLogDirsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.dirs)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.dirs)?;

        Ok(total_size)
    }
}

impl Decodable for AlterReplicaLogDirsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let dirs = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            dirs,
        })
    }
}

impl Default for AlterReplicaLogDirsRequest {
    fn default() -> Self {
        Self {
            dirs: Default::default(),
        }
    }
}

impl Message for AlterReplicaLogDirsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AlterReplicaLogDirsRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

