//! ApiVersionsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ApiVersionsResponse.json).
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


/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ApiVersion {
    /// The minimum supported version, inclusive.
    /// 
    /// Supported API versions: 0-3
    pub min_version: i16,

    /// The maximum supported version, inclusive.
    /// 
    /// Supported API versions: 0-3
    pub max_version: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ApiVersion {
    type Builder = ApiVersionBuilder;

    fn builder() -> Self::Builder{
        ApiVersionBuilder::default()
    }
}

impl MapEncodable for ApiVersion {
    type Key = i16;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i16(*key);
        buf.put_i16(self.min_version);
        buf.put_i16(self.max_version);
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 2;
        total_size += 2;
        total_size += 2;
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

impl MapDecodable for ApiVersion {
    type Key = i16;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = buf.try_get_i16()?;
        let min_version = buf.try_get_i16()?;
        let max_version = buf.try_get_i16()?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            min_version,
            max_version,
            unknown_tagged_fields,
        }))
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self {
            min_version: 0,
            max_version: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ApiVersion {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct SupportedFeatureKey {
    /// The minimum supported version for the feature.
    /// 
    /// Supported API versions: 3
    pub min_version: i16,

    /// The maximum supported version for the feature.
    /// 
    /// Supported API versions: 3
    pub max_version: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for SupportedFeatureKey {
    type Builder = SupportedFeatureKeyBuilder;

    fn builder() -> Self::Builder{
        SupportedFeatureKeyBuilder::default()
    }
}

impl MapEncodable for SupportedFeatureKey {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            buf.put_i16(self.min_version);
        } else {
            if self.min_version != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            buf.put_i16(self.max_version);
        } else {
            if self.max_version != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            total_size += 2;
        } else {
            if self.min_version != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            total_size += 2;
        } else {
            if self.max_version != 0 {
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

impl MapDecodable for SupportedFeatureKey {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let min_version = if version >= 3 {
            buf.try_get_i16()?
        } else {
            0
        };
        let max_version = if version >= 3 {
            buf.try_get_i16()?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            min_version,
            max_version,
            unknown_tagged_fields,
        }))
    }
}

impl Default for SupportedFeatureKey {
    fn default() -> Self {
        Self {
            min_version: 0,
            max_version: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SupportedFeatureKey {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FinalizedFeatureKey {
    /// The cluster-wide finalized max version level for the feature.
    /// 
    /// Supported API versions: 3
    pub max_version_level: i16,

    /// The cluster-wide finalized min version level for the feature.
    /// 
    /// Supported API versions: 3
    pub min_version_level: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for FinalizedFeatureKey {
    type Builder = FinalizedFeatureKeyBuilder;

    fn builder() -> Self::Builder{
        FinalizedFeatureKeyBuilder::default()
    }
}

impl MapEncodable for FinalizedFeatureKey {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            buf.put_i16(self.max_version_level);
        } else {
            if self.max_version_level != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            buf.put_i16(self.min_version_level);
        } else {
            if self.min_version_level != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            total_size += 2;
        } else {
            if self.max_version_level != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 3 {
            total_size += 2;
        } else {
            if self.min_version_level != 0 {
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

impl MapDecodable for FinalizedFeatureKey {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let max_version_level = if version >= 3 {
            buf.try_get_i16()?
        } else {
            0
        };
        let min_version_level = if version >= 3 {
            buf.try_get_i16()?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            max_version_level,
            min_version_level,
            unknown_tagged_fields,
        }))
    }
}

impl Default for FinalizedFeatureKey {
    fn default() -> Self {
        Self {
            max_version_level: 0,
            min_version_level: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FinalizedFeatureKey {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    /// 
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The APIs supported by the broker.
    /// 
    /// Supported API versions: 0-3
    pub api_keys: indexmap::IndexMap<i16, ApiVersion>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-3
    pub throttle_time_ms: i32,

    /// Features supported by the broker.
    /// 
    /// Supported API versions: 3
    pub supported_features: indexmap::IndexMap<StrBytes, SupportedFeatureKey>,

    /// The monotonically increasing epoch for the finalized features information. Valid values are >= 0. A value of -1 is special and represents unknown epoch.
    /// 
    /// Supported API versions: 3
    pub finalized_features_epoch: i64,

    /// List of cluster-wide finalized features. The information is valid only if FinalizedFeaturesEpoch >= 0.
    /// 
    /// Supported API versions: 3
    pub finalized_features: indexmap::IndexMap<StrBytes, FinalizedFeatureKey>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ApiVersionsResponse {
    type Builder = ApiVersionsResponseBuilder;

    fn builder() -> Self::Builder{
        ApiVersionsResponseBuilder::default()
    }
}

impl Encodable for ApiVersionsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i16(self.error_code);
        if version >= 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.api_keys)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.api_keys)?;
        }
        if version >= 1 {
            buf.put_i32(self.throttle_time_ms);
        }
        if version >= 3 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if !self.supported_features.is_empty() {
                num_tagged_fields += 1;
            }
            if self.finalized_features_epoch != -1 {
                num_tagged_fields += 1;
            }
            if !self.finalized_features.is_empty() {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);
            if !self.supported_features.is_empty() {
                let computed_size = types::CompactArray(types::Struct { version }).compute_size(&self.supported_features)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                buf.put_i32(0);
                buf.put_i32(computed_size as i32);
                types::CompactArray(types::Struct { version }).encode(buf, &self.supported_features)?;
            }
            if self.finalized_features_epoch != -1 {
                let computed_size = types::Int64.compute_size(self.finalized_features_epoch)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                buf.put_i32(1);
                buf.put_i32(computed_size as i32);
                buf.put_i64(self.finalized_features_epoch);
            }
            if !self.finalized_features.is_empty() {
                let computed_size = types::CompactArray(types::Struct { version }).compute_size(&self.finalized_features)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                buf.put_i32(2);
                buf.put_i32(computed_size as i32);
                types::CompactArray(types::Struct { version }).encode(buf, &self.finalized_features)?;
            }

            write_unknown_tagged_fields(buf, 3.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 2;
        if version >= 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.api_keys)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.api_keys)?;
        }
        if version >= 1 {
            total_size += 4;
        }
        if version >= 3 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if !self.supported_features.is_empty() {
                num_tagged_fields += 1;
            }
            if self.finalized_features_epoch != -1 {
                num_tagged_fields += 1;
            }
            if !self.finalized_features.is_empty() {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;
            if !self.supported_features.is_empty() {
                let computed_size = types::CompactArray(types::Struct { version }).compute_size(&self.supported_features)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += 4;
                total_size += 4;
                total_size += computed_size;
            }
            if self.finalized_features_epoch != -1 {
                let computed_size = types::Int64.compute_size(self.finalized_features_epoch)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += 4;
                total_size += 4;
                total_size += computed_size;
            }
            if !self.finalized_features.is_empty() {
                let computed_size = types::CompactArray(types::Struct { version }).compute_size(&self.finalized_features)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += 4;
                total_size += 4;
                total_size += computed_size;
            }

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for ApiVersionsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = buf.try_get_i16()?;
        let api_keys = if version >= 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let throttle_time_ms = if version >= 1 {
            buf.try_get_i32()?
        } else {
            0
        };
        let mut supported_features = Default::default();
        let mut finalized_features_epoch = -1;
        let mut finalized_features = Default::default();
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                match tag {
                    0 => {
                        supported_features = types::CompactArray(types::Struct { version }).decode(buf)?;
                    },
                    1 => {
                        finalized_features_epoch = types::Int64.decode(buf)?;
                    },
                    2 => {
                        finalized_features = types::CompactArray(types::Struct { version }).decode(buf)?;
                    },
                    _ => {
                        let unknown_value = buf.try_get_bytes(size as usize)?;
                        unknown_tagged_fields.insert(tag as i32, unknown_value);
                    }
                }
            }
        }
        Ok(Self {
            error_code,
            api_keys,
            throttle_time_ms,
            supported_features,
            finalized_features_epoch,
            finalized_features,
            unknown_tagged_fields,
        })
    }
}

impl Default for ApiVersionsResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            api_keys: Default::default(),
            throttle_time_ms: 0,
            supported_features: Default::default(),
            finalized_features_epoch: -1,
            finalized_features: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ApiVersionsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for ApiVersionsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

