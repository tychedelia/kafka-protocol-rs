//! ApiVersionsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ApiVersionsRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct ApiVersionsRequest {
    /// The name of the client.
    /// 
    /// Supported API versions: 3
    pub client_software_name: StrBytes,

    /// The version of the client.
    /// 
    /// Supported API versions: 3
    pub client_software_version: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ApiVersionsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, &self.client_software_name)?;
        }
        if version >= 3 {
            types::CompactString.encode(buf, &self.client_software_version)?;
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
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.client_software_name)?;
        }
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.client_software_version)?;
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

impl Decodable for ApiVersionsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let client_software_name = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let client_software_version = if version >= 3 {
            types::CompactString.decode(buf)?
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
            client_software_name,
            client_software_version,
            unknown_tagged_fields,
        })
    }
}

impl Default for ApiVersionsRequest {
    fn default() -> Self {
        Self {
            client_software_name: Default::default(),
            client_software_version: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ApiVersionsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for ApiVersionsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            2
        } else {
            1
        }
    }
}

