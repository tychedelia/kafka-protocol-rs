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


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The resource error message, or null if there was no error.
    /// 
    /// Supported API versions: 0-1
    pub error_message: Option<StrBytes>,

    /// The resource type.
    /// 
    /// Supported API versions: 0-1
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-1
    pub resource_name: StrBytes,

}

impl Encodable for AlterConfigsResourceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::String.encode(buf, &self.error_message)?;
        types::Int8.encode(buf, &self.resource_type)?;
        types::String.encode(buf, &self.resource_name)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::String.compute_size(&self.error_message)?;
        total_size += types::Int8.compute_size(&self.resource_type)?;
        total_size += types::String.compute_size(&self.resource_name)?;

        Ok(total_size)
    }
}

impl Decodable for AlterConfigsResourceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let error_message = types::String.decode(buf)?;
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = types::String.decode(buf)?;
        Ok(Self {
            error_code,
            error_message,
            resource_type,
            resource_name,
        })
    }
}

impl Default for AlterConfigsResourceResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            resource_type: 0,
            resource_name: Default::default(),
        }
    }
}

impl Message for AlterConfigsResourceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The responses for each resource.
    /// 
    /// Supported API versions: 0-1
    pub responses: Vec<AlterConfigsResourceResponse>,

}

impl Encodable for AlterConfigsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.responses)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.responses)?;

        Ok(total_size)
    }
}

impl Decodable for AlterConfigsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let responses = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            responses,
        })
    }
}

impl Default for AlterConfigsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            responses: Default::default(),
        }
    }
}

impl Message for AlterConfigsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AlterConfigsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

