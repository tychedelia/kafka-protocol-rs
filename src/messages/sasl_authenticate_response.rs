//! SaslAuthenticateResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/SaslAuthenticateResponse.json).
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


/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct SaslAuthenticateResponse {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_code: i16,

    /// The error message, or null if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_message: Option<StrBytes>,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    /// 
    /// Supported API versions: 0-2
    pub auth_bytes: Bytes,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    /// 
    /// Supported API versions: 1-2
    pub session_lifetime_ms: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for SaslAuthenticateResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
        }
        if version >= 2 {
            types::CompactBytes.encode(buf, &self.auth_bytes)?;
        } else {
            types::Bytes.encode(buf, &self.auth_bytes)?;
        }
        if version >= 1 {
            types::Int64.encode(buf, &self.session_lifetime_ms)?;
        }
        if version >= 2 {
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
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
        }
        if version >= 2 {
            total_size += types::CompactBytes.compute_size(&self.auth_bytes)?;
        } else {
            total_size += types::Bytes.compute_size(&self.auth_bytes)?;
        }
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.session_lifetime_ms)?;
        }
        if version >= 2 {
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

impl Decodable for SaslAuthenticateResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let auth_bytes = if version >= 2 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let session_lifetime_ms = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            error_message,
            auth_bytes,
            session_lifetime_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for SaslAuthenticateResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            auth_bytes: Default::default(),
            session_lifetime_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SaslAuthenticateResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for SaslAuthenticateResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            1
        } else {
            0
        }
    }
}

