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


/// Valid versions: 0-4
#[derive(Debug, Clone)]
pub struct DeleteTopicsRequest {
    /// The names of the topics to delete
    /// 
    /// Supported API versions: 0-4
    pub topic_names: Vec<super::TopicName>,

    /// The length of time in milliseconds to wait for the deletions to complete.
    /// 
    /// Supported API versions: 0-4
    pub timeout_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DeleteTopicsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 4 {
            types::CompactArray(types::CompactString).encode(buf, &self.topic_names)?;
        } else {
            types::Array(types::String).encode(buf, &self.topic_names)?;
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
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
            total_size += types::CompactArray(types::CompactString).compute_size(&self.topic_names)?;
        } else {
            total_size += types::Array(types::String).compute_size(&self.topic_names)?;
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
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

impl Decodable for DeleteTopicsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_names = if version == 4 {
            types::CompactArray(types::CompactString).decode(buf)?
        } else {
            types::Array(types::String).decode(buf)?
        };
        let timeout_ms = types::Int32.decode(buf)?;
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
            topic_names,
            timeout_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for DeleteTopicsRequest {
    fn default() -> Self {
        Self {
            topic_names: Default::default(),
            timeout_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DeleteTopicsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for DeleteTopicsRequest {
    fn header_version(version: i16) -> i16 {
        if version == 4 {
            2
        } else {
            1
        }
    }
}

