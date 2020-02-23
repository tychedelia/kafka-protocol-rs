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


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct SaslAuthenticateRequest {
    /// The SASL authentication bytes from the client, as defined by the SASL mechanism.
    /// 
    /// Supported API versions: 0-2
    pub auth_bytes: Vec<u8>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for SaslAuthenticateRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 2 {
            types::CompactBytes.encode(buf, &self.auth_bytes)?;
        } else {
            types::Bytes.encode(buf, &self.auth_bytes)?;
        }
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactBytes.compute_size(&self.auth_bytes)?;
        } else {
            total_size += types::Bytes.compute_size(&self.auth_bytes)?;
        }
        if version == 2 {
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

impl Decodable for SaslAuthenticateRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let auth_bytes = if version == 2 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            auth_bytes,
            unknown_tagged_fields,
        })
    }
}

impl Default for SaslAuthenticateRequest {
    fn default() -> Self {
        Self {
            auth_bytes: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SaslAuthenticateRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for SaslAuthenticateRequest {
    fn header_version(version: i16) -> i16 {
        if version == 2 {
            2
        } else {
            1
        }
    }
}

