//! AlterUserScramCredentialsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AlterUserScramCredentialsRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AlterUserScramCredentialsRequest {
    /// The SCRAM credentials to remove.
    ///
    /// Supported API versions: 0
    pub deletions: Vec<ScramCredentialDeletion>,

    /// The SCRAM credentials to update/insert.
    ///
    /// Supported API versions: 0
    pub upsertions: Vec<ScramCredentialUpsertion>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AlterUserScramCredentialsRequest {
    type Builder = AlterUserScramCredentialsRequestBuilder;

    fn builder() -> Self::Builder {
        AlterUserScramCredentialsRequestBuilder::default()
    }
}

impl Encodable for AlterUserScramCredentialsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactArray(types::Struct { version }).encode(buf, &self.deletions)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.upsertions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.deletions)?;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.upsertions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for AlterUserScramCredentialsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let deletions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let upsertions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            deletions,
            upsertions,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterUserScramCredentialsRequest {
    fn default() -> Self {
        Self {
            deletions: Default::default(),
            upsertions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterUserScramCredentialsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ScramCredentialDeletion {
    /// The user name.
    ///
    /// Supported API versions: 0
    pub name: StrBytes,

    /// The SCRAM mechanism.
    ///
    /// Supported API versions: 0
    pub mechanism: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ScramCredentialDeletion {
    type Builder = ScramCredentialDeletionBuilder;

    fn builder() -> Self::Builder {
        ScramCredentialDeletionBuilder::default()
    }
}

impl Encodable for ScramCredentialDeletion {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.name)?;
        types::Int8.encode(buf, &self.mechanism)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.name)?;
        total_size += types::Int8.compute_size(&self.mechanism)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for ScramCredentialDeletion {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::CompactString.decode(buf)?;
        let mechanism = types::Int8.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            name,
            mechanism,
            unknown_tagged_fields,
        })
    }
}

impl Default for ScramCredentialDeletion {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mechanism: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ScramCredentialDeletion {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ScramCredentialUpsertion {
    /// The user name.
    ///
    /// Supported API versions: 0
    pub name: StrBytes,

    /// The SCRAM mechanism.
    ///
    /// Supported API versions: 0
    pub mechanism: i8,

    /// The number of iterations.
    ///
    /// Supported API versions: 0
    pub iterations: i32,

    /// A random salt generated by the client.
    ///
    /// Supported API versions: 0
    pub salt: Bytes,

    /// The salted password.
    ///
    /// Supported API versions: 0
    pub salted_password: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ScramCredentialUpsertion {
    type Builder = ScramCredentialUpsertionBuilder;

    fn builder() -> Self::Builder {
        ScramCredentialUpsertionBuilder::default()
    }
}

impl Encodable for ScramCredentialUpsertion {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.name)?;
        types::Int8.encode(buf, &self.mechanism)?;
        types::Int32.encode(buf, &self.iterations)?;
        types::CompactBytes.encode(buf, &self.salt)?;
        types::CompactBytes.encode(buf, &self.salted_password)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.name)?;
        total_size += types::Int8.compute_size(&self.mechanism)?;
        total_size += types::Int32.compute_size(&self.iterations)?;
        total_size += types::CompactBytes.compute_size(&self.salt)?;
        total_size += types::CompactBytes.compute_size(&self.salted_password)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for ScramCredentialUpsertion {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::CompactString.decode(buf)?;
        let mechanism = types::Int8.decode(buf)?;
        let iterations = types::Int32.decode(buf)?;
        let salt = types::CompactBytes.decode(buf)?;
        let salted_password = types::CompactBytes.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            name,
            mechanism,
            iterations,
            salt,
            salted_password,
            unknown_tagged_fields,
        })
    }
}

impl Default for ScramCredentialUpsertion {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mechanism: 0,
            iterations: 0,
            salt: Default::default(),
            salted_password: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ScramCredentialUpsertion {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for AlterUserScramCredentialsRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
