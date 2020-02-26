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


/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct CreatableTopicConfigs {
    /// The configuration name.
    /// 
    /// Supported API versions: 5
    pub name: StrBytes,

    /// The configuration value.
    /// 
    /// Supported API versions: 5
    pub value: Option<StrBytes>,

    /// True if the configuration is read-only.
    /// 
    /// Supported API versions: 5
    pub read_only: bool,

    /// The configuration source.
    /// 
    /// Supported API versions: 5
    pub config_source: i8,

    /// True if this configuration is sensitive.
    /// 
    /// Supported API versions: 5
    pub is_sensitive: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreatableTopicConfigs {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 5 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            types::CompactString.encode(buf, &self.value)?;
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            types::Boolean.encode(buf, &self.read_only)?;
        } else {
            if self.read_only {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            types::Int8.encode(buf, &self.config_source)?;
        }
        if version == 5 {
            types::Boolean.encode(buf, &self.is_sensitive)?;
        } else {
            if self.is_sensitive {
                return Err(EncodeError)
            }
        }
        if version == 5 {
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
        if version == 5 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            total_size += types::CompactString.compute_size(&self.value)?;
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            total_size += types::Boolean.compute_size(&self.read_only)?;
        } else {
            if self.read_only {
                return Err(EncodeError)
            }
        }
        if version == 5 {
            total_size += types::Int8.compute_size(&self.config_source)?;
        }
        if version == 5 {
            total_size += types::Boolean.compute_size(&self.is_sensitive)?;
        } else {
            if self.is_sensitive {
                return Err(EncodeError)
            }
        }
        if version == 5 {
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

impl Decodable for CreatableTopicConfigs {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version == 5 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let value = if version == 5 {
            types::CompactString.decode(buf)?
        } else {
            Some(Default::default())
        };
        let read_only = if version == 5 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let config_source = if version == 5 {
            types::Int8.decode(buf)?
        } else {
            -1
        };
        let is_sensitive = if version == 5 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 5 {
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
            value,
            read_only,
            config_source,
            is_sensitive,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreatableTopicConfigs {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Some(Default::default()),
            read_only: false,
            config_source: -1,
            is_sensitive: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableTopicConfigs {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct CreatableTopicResult {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-5
    pub error_code: i16,

    /// The error message, or null if there was no error.
    /// 
    /// Supported API versions: 1-5
    pub error_message: Option<StrBytes>,

    /// Optional topic config error returned if configs are not returned in the response.
    /// 
    /// Supported API versions: 5
    pub topic_config_error_code: i16,

    /// Number of partitions of the topic.
    /// 
    /// Supported API versions: 5
    pub num_partitions: i32,

    /// Replicator factor of the topic.
    /// 
    /// Supported API versions: 5
    pub replication_factor: i16,

    /// Configuration of the topic.
    /// 
    /// Supported API versions: 5
    pub configs: Option<Vec<CreatableTopicConfigs>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for CreatableTopicResult {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 5 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 1 {
            if version == 5 {
                types::CompactString.encode(buf, &self.error_message)?;
            } else {
                types::String.encode(buf, &self.error_message)?;
            }
        }
        if version == 5 {
            types::Int32.encode(buf, &self.num_partitions)?;
        }
        if version == 5 {
            types::Int16.encode(buf, &self.replication_factor)?;
        }
        if version == 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.configs)?;
        }
        if version == 5 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if self.topic_config_error_code != 0 {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;
            if self.topic_config_error_code != 0 {
                let computed_size = types::Int16.compute_size(&self.topic_config_error_code)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                types::UnsignedVarInt.encode(buf, 0)?;
                types::UnsignedVarInt.encode(buf, computed_size as u32)?;
                types::Int16.encode(buf, &self.topic_config_error_code)?;
            }

            write_unknown_tagged_fields(buf, 1.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version == 5 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 1 {
            if version == 5 {
                total_size += types::CompactString.compute_size(&self.error_message)?;
            } else {
                total_size += types::String.compute_size(&self.error_message)?;
            }
        }
        if version == 5 {
            total_size += types::Int32.compute_size(&self.num_partitions)?;
        }
        if version == 5 {
            total_size += types::Int16.compute_size(&self.replication_factor)?;
        }
        if version == 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.configs)?;
        }
        if version == 5 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if self.topic_config_error_code != 0 {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;
            if self.topic_config_error_code != 0 {
                let computed_size = types::Int16.compute_size(&self.topic_config_error_code)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += types::UnsignedVarInt.compute_size(0)?;
                total_size += types::UnsignedVarInt.compute_size(computed_size as u32)?;
                total_size += computed_size;
            }

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl MapDecodable for CreatableTopicResult {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version == 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version >= 1 {
            if version == 5 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let mut topic_config_error_code = 0;
        let num_partitions = if version == 5 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let replication_factor = if version == 5 {
            types::Int16.decode(buf)?
        } else {
            -1
        };
        let configs = if version == 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 5 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                match tag {
                    0 => {
                        topic_config_error_code = types::Int16.decode(buf)?;
                    },
                    _ => {
                        let mut unknown_value = vec![0; size as usize];
                        buf.try_copy_to_slice(&mut unknown_value)?;
                        unknown_tagged_fields.insert(tag as i32, unknown_value);
                    }
                }
            }
        }
        Ok((key_field, Self {
            error_code,
            error_message,
            topic_config_error_code,
            num_partitions,
            replication_factor,
            configs,
            unknown_tagged_fields,
        }))
    }
}

impl Default for CreatableTopicResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            topic_config_error_code: 0,
            num_partitions: -1,
            replication_factor: -1,
            configs: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 2-5
    pub throttle_time_ms: i32,

    /// Results for each topic we tried to create.
    /// 
    /// Supported API versions: 0-5
    pub topics: indexmap::IndexMap<super::TopicName, CreatableTopicResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreateTopicsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version == 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version == 5 {
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
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version == 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version == 5 {
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

impl Decodable for CreateTopicsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = if version == 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 5 {
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
            throttle_time_ms,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateTopicsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateTopicsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for CreateTopicsResponse {
    fn header_version(version: i16) -> i16 {
        if version == 5 {
            1
        } else {
            0
        }
    }
}

