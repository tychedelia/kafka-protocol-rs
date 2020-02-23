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


/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrPartitionError {
    /// The topic name.
    /// 
    /// Supported API versions: 0-4
    pub topic_name: super::TopicName,

    /// The partition index.
    /// 
    /// Supported API versions: 0-4
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-4
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrPartitionError {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 4 {
            types::CompactString.encode(buf, &self.topic_name)?;
        } else {
            types::String.encode(buf, &self.topic_name)?;
        }
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        if version == 4 {
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
        if version == 4 {
            total_size += types::CompactString.compute_size(&self.topic_name)?;
        } else {
            total_size += types::String.compute_size(&self.topic_name)?;
        }
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version == 4 {
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

impl Decodable for LeaderAndIsrPartitionError {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version == 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
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
            topic_name,
            partition_index,
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrPartitionError {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: 0,
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrPartitionError {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-4
    pub error_code: i16,

    /// Each partition.
    /// 
    /// Supported API versions: 0-4
    pub partition_errors: Vec<LeaderAndIsrPartitionError>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrResponse {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version == 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partition_errors)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partition_errors)?;
        }
        if version == 4 {
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version == 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_errors)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partition_errors)?;
        }
        if version == 4 {
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

impl Decodable for LeaderAndIsrResponse {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let partition_errors = if version == 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 4 {
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
            error_code,
            partition_errors,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_errors: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for LeaderAndIsrResponse {
    fn header_version(version: i16) -> i16 {
        if version == 4 {
            1
        } else {
            0
        }
    }
}

