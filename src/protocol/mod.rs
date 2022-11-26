//! Raw types and utilities for use with the Kafka protocol.
//!
//! Most types are used internally in encoding/decoding, and are not required by typical use cases
//! for interacting with the protocol. However, types can be used for decoding partial messages,
//! or rewriting parts of an encoded message.
use std::string::FromUtf8Error;
use std::{error::Error, str::Utf8Error};
use std::borrow::Borrow;
use std::ops::RangeBounds;
use std::collections::BTreeMap;
use std::cmp;

use buf::{ByteBuf, ByteBufMut};

use self::buf::NotEnoughBytesError;

pub mod types;
pub mod buf;

/// A string type backed by [`bytes::Bytes`].
pub type StrBytes = string::String<bytes::Bytes>;

/// An error representing any fault while decoding a message, usually when the buffer is incorrectly
/// sized or otherwise invalid for the decoded message.
#[derive(Debug)]
pub struct DecodeError;

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DecodeErrror")
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<NotEnoughBytesError> for DecodeError {
    fn from(_: NotEnoughBytesError) -> Self {
        DecodeError
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(_: Utf8Error) -> Self {
        DecodeError
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(_: FromUtf8Error) -> Self {
        DecodeError
    }
}

/// An error representing any fault while encoding a message, usually when a message in an invalid
/// state is attempted to be encoded.
#[derive(Debug)]
pub struct EncodeError;


impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EncodeError")
    }
}

impl Error for EncodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub(crate) trait NewType<Inner>: From<Inner> + Into<Inner> + Borrow<Inner> {}

impl<T> NewType<T> for T {}

pub(crate) trait Encoder<Value> {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, value: Value) -> Result<(), EncodeError>;
    fn compute_size(&self, value: Value) -> Result<usize, EncodeError>;
    fn fixed_size(&self) -> Option<usize> {
        None
    }
}

pub(crate) trait Decoder<Value> {
    fn decode<B: ByteBuf>(&self, buf: &mut B) -> Result<Value, DecodeError>;
}

/// The range of versions (min, max) allowed for agiven message.
#[derive(Debug, Copy, Clone)]
pub struct VersionRange {
    /// The minimum version in the range.
    pub min: i16,
    /// The maximum version in the range.
    pub max: i16,
}

impl VersionRange {
    /// Checks whether the version range contains no versions.
    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    /// Finds the valid intersection with a provided other version range.
    pub fn intersect(&self, other: &VersionRange) -> VersionRange {
        VersionRange {
            min: cmp::max(self.min, other.min),
            max: cmp::min(self.max, other.max),
        }
    }
}

/// An API request or response.
///
/// All API messages must provide a set of valid versions.
pub trait Message: Sized {
    /// The valid versions for this message.
    const VERSIONS: VersionRange;
}

/// An encodable message.
pub trait Encodable: Sized {
    /// Encode the message into the target buffer.
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError>;
    /// Compute the total size of the message when encoded.
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError>;
}

/// A decodable message.
pub trait Decodable: Sized {
    /// Decode the message from the provided buffer and version.
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError>;
}

pub(crate) trait MapEncodable: Sized {
    type Key;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError>;
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError>;
}

pub(crate) trait MapDecodable: Sized {
    type Key;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError>;
}

/// Every message has a set of versions valid for a given header version.
pub trait HeaderVersion {
    /// Maps a header version to a given version for a particular API message.
    fn header_version(version: i16) -> i16;
}

/// An API request.
///
/// Every abstract request must be able to provide the following items:
/// - An API key mapped to this request.
/// - A version based on a provided header version.
pub trait Request: Message + Encodable + Decodable + HeaderVersion {
    /// The API key of this request.
    const KEY: i16;
    /// The response associated with this request.
    type Response: Message + Encodable + Decodable + HeaderVersion;
}

pub(crate) fn write_unknown_tagged_fields<B: ByteBufMut, R: RangeBounds<i32>>(
    buf: &mut B,
    range: R,
    unknown_tagged_fields: &BTreeMap<i32, Vec<u8>>,
) -> Result<(), EncodeError> {
    for (&k, v) in unknown_tagged_fields.range(range) {
        if v.len() > std::u32::MAX as usize {
            error!("Tagged field is too long to encode ({} bytes)", v.len());
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, k as u32)?;
        types::UnsignedVarInt.encode(buf, v.len() as u32)?;
        buf.put_slice(v.as_slice());
    }
    Ok(())
}

pub(crate) fn compute_unknown_tagged_fields_size(
    unknown_tagged_fields: &BTreeMap<i32, Vec<u8>>,
) -> Result<usize, EncodeError> {
    let mut total_size = 0;
    for (&k, v) in unknown_tagged_fields {
        if v.len() > std::u32::MAX as usize {
            error!("Tagged field is too long to encode ({} bytes)", v.len());
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(k as u32)?;
        total_size += types::UnsignedVarInt.compute_size(v.len() as u32)?;
        total_size += v.len();
    }
    Ok(total_size)
}
