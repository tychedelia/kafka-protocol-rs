//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-4
#[derive(Debug, Clone, PartialEq)]
pub struct FindCoordinatorRequest {
    /// The coordinator key.
    /// 
    /// Supported API versions: 0-3
    pub key: StrBytes,

    /// The coordinator key type. (Group, transaction, etc.)
    /// 
    /// Supported API versions: 1-4
    pub key_type: i8,

    /// The coordinator keys.
    /// 
    /// Supported API versions: 4
    pub coordinator_keys: Vec<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for FindCoordinatorRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 3 {
            if version == 3 {
                types::CompactString.encode(buf, &self.key)?;
            } else {
                types::String.encode(buf, &self.key)?;
            }
        } else {
            if !self.key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.key_type)?;
        } else {
            if self.key_type != 0 {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            types::CompactArray(types::CompactString).encode(buf, &self.coordinator_keys)?;
        } else {
            if !self.coordinator_keys.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
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
        if version <= 3 {
            if version == 3 {
                total_size += types::CompactString.compute_size(&self.key)?;
            } else {
                total_size += types::String.compute_size(&self.key)?;
            }
        } else {
            if !self.key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.key_type)?;
        } else {
            if self.key_type != 0 {
                return Err(EncodeError)
            }
        }
        if version == 4 {
            total_size += types::CompactArray(types::CompactString).compute_size(&self.coordinator_keys)?;
        } else {
            if !self.coordinator_keys.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
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

impl Decodable for FindCoordinatorRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let key = if version <= 3 {
            if version == 3 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let key_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let coordinator_keys = if version == 4 {
            types::CompactArray(types::CompactString).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
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
            key,
            key_type,
            coordinator_keys,
            unknown_tagged_fields,
        })
    }
}

impl Default for FindCoordinatorRequest {
    fn default() -> Self {
        Self {
            key: Default::default(),
            key_type: 0,
            coordinator_keys: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FindCoordinatorRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for FindCoordinatorRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            2
        } else {
            1
        }
    }
}

