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


/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataRequestTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-9
    pub name: super::TopicName,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for MetadataRequestTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 9 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version == 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version == 9 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        if version == 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for MetadataRequestTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version == 9 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            unknown_tagged_fields,
        })
    }
}

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataRequestTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone)]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    /// 
    /// Supported API versions: 0-9
    pub topics: Option<Vec<MetadataRequestTopic>>,

    /// If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
    /// 
    /// Supported API versions: 4-9
    pub allow_auto_topic_creation: bool,

    /// Whether to include cluster authorized operations.
    /// 
    /// Supported API versions: 8-9
    pub include_cluster_authorized_operations: bool,

    /// Whether to include topic authorized operations.
    /// 
    /// Supported API versions: 8-9
    pub include_topic_authorized_operations: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for MetadataRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 4 {
            types::Boolean.encode(buf, &self.allow_auto_topic_creation)?;
        } else {
            if !self.allow_auto_topic_creation {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::Boolean.encode(buf, &self.include_cluster_authorized_operations)?;
        } else {
            if self.include_cluster_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::Boolean.encode(buf, &self.include_topic_authorized_operations)?;
        } else {
            if self.include_topic_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version == 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version == 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 4 {
            total_size += types::Boolean.compute_size(&self.allow_auto_topic_creation)?;
        } else {
            if !self.allow_auto_topic_creation {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::Boolean.compute_size(&self.include_cluster_authorized_operations)?;
        } else {
            if self.include_cluster_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::Boolean.compute_size(&self.include_topic_authorized_operations)?;
        } else {
            if self.include_topic_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version == 9 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for MetadataRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = if version == 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let allow_auto_topic_creation = if version >= 4 {
            types::Boolean.decode(buf)?
        } else {
            true
        };
        let include_cluster_authorized_operations = if version >= 8 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let include_topic_authorized_operations = if version >= 8 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            topics,
            allow_auto_topic_creation,
            include_cluster_authorized_operations,
            include_topic_authorized_operations,
            unknown_tagged_fields,
        })
    }
}

impl Default for MetadataRequest {
    fn default() -> Self {
        Self {
            topics: Some(Default::default()),
            allow_auto_topic_creation: true,
            include_cluster_authorized_operations: false,
            include_topic_authorized_operations: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MetadataRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

impl HeaderVersion for MetadataRequest {
    fn header_version(version: i16) -> i16 {
        if version == 9 {
            2
        } else {
            1
        }
    }
}

