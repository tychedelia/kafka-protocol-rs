//! UpdateFeaturesRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/UpdateFeaturesRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FeatureUpdateKey {
    /// The new maximum version level for the finalized feature. A value >= 1 is valid. A value < 1, is special, and can be used to request the deletion of the finalized feature.
    /// 
    /// Supported API versions: 0-1
    pub max_version_level: i16,

    /// DEPRECATED in version 1 (see DowngradeType). When set to true, the finalized feature version level is allowed to be downgraded/deleted. The downgrade request will fail if the new maximum version level is a value that's not lower than the existing maximum finalized version level.
    /// 
    /// Supported API versions: 0
    pub allow_downgrade: bool,

    /// Determine which type of upgrade will be performed: 1 will perform an upgrade only (default), 2 is safe downgrades only (lossless), 3 is unsafe downgrades (lossy).
    /// 
    /// Supported API versions: 1
    pub upgrade_type: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for FeatureUpdateKey {
    type Builder = FeatureUpdateKeyBuilder;

    fn builder() -> Self::Builder{
        FeatureUpdateKeyBuilder::default()
    }
}

impl MapEncodable for FeatureUpdateKey {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, key)?;
        types::Int16.encode(buf, &self.max_version_level)?;
        if version == 0 {
            types::Boolean.encode(buf, &self.allow_downgrade)?;
        } else {
            if self.allow_downgrade {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.upgrade_type)?;
        } else {
            if self.upgrade_type != 1 {
                return Err(EncodeError)
            }
        }
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.max_version_level)?;
        if version == 0 {
            total_size += types::Boolean.compute_size(&self.allow_downgrade)?;
        } else {
            if self.allow_downgrade {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.upgrade_type)?;
        } else {
            if self.upgrade_type != 1 {
                return Err(EncodeError)
            }
        }
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl MapDecodable for FeatureUpdateKey {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::CompactString.decode(buf)?;
        let max_version_level = types::Int16.decode(buf)?;
        let allow_downgrade = if version == 0 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let upgrade_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok((key_field, Self {
            max_version_level,
            allow_downgrade,
            upgrade_type,
            unknown_tagged_fields,
        }))
    }
}

impl Default for FeatureUpdateKey {
    fn default() -> Self {
        Self {
            max_version_level: 0,
            allow_downgrade: false,
            upgrade_type: 1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FeatureUpdateKey {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct UpdateFeaturesRequest {
    /// How long to wait in milliseconds before timing out the request.
    /// 
    /// Supported API versions: 0-1
    pub timeout_ms: i32,

    /// The list of updates to finalized features.
    /// 
    /// Supported API versions: 0-1
    pub feature_updates: indexmap::IndexMap<StrBytes, FeatureUpdateKey>,

    /// True if we should validate the request, but not perform the upgrade or downgrade.
    /// 
    /// Supported API versions: 1
    pub validate_only: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for UpdateFeaturesRequest {
    type Builder = UpdateFeaturesRequestBuilder;

    fn builder() -> Self::Builder{
        UpdateFeaturesRequestBuilder::default()
    }
}

impl Encodable for UpdateFeaturesRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.timeout_ms)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.feature_updates)?;
        if version >= 1 {
            types::Boolean.encode(buf, &self.validate_only)?;
        } else {
            if self.validate_only {
                return Err(EncodeError)
            }
        }
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.feature_updates)?;
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.validate_only)?;
        } else {
            if self.validate_only {
                return Err(EncodeError)
            }
        }
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for UpdateFeaturesRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let timeout_ms = types::Int32.decode(buf)?;
        let feature_updates = types::CompactArray(types::Struct { version }).decode(buf)?;
        let validate_only = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            timeout_ms,
            feature_updates,
            validate_only,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateFeaturesRequest {
    fn default() -> Self {
        Self {
            timeout_ms: 60000,
            feature_updates: Default::default(),
            validate_only: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateFeaturesRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for UpdateFeaturesRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

