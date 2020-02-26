#[macro_use]
extern crate log;

use std::error::Error;
use std::borrow::Borrow;
use std::ops::RangeBounds;
use std::collections::BTreeMap;
use std::cmp;

use buf::{ByteBuf, ByteBufMut};

pub mod types;
pub mod buf;

pub type StrBytes = string::String<bytes::Bytes>;

#[derive(Debug)]
pub struct DecodeError;

#[derive(Debug)]
pub struct EncodeError;

impl<E: Error> From<E> for DecodeError {
    fn from(other: E) -> Self {
        error!("{}", other);
        DecodeError
    }
}

pub trait NewType<Inner>: From<Inner> + Into<Inner> + Borrow<Inner> {}

impl<T> NewType<T> for T {}

pub trait Encoder<Value> {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, value: Value) -> Result<(), EncodeError>;
    fn compute_size(&self, value: Value) -> Result<usize, EncodeError>;
    fn fixed_size(&self) -> Option<usize> {
        None
    }
}
pub trait Decoder<Value> {
    fn decode<B: ByteBuf>(&self, buf: &mut B) -> Result<Value, DecodeError>;
}

#[derive(Debug, Copy, Clone)]
pub struct VersionRange {
    pub min: i16,
    pub max: i16,
}

impl VersionRange {
    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }
    pub fn intersect(&self, other: &VersionRange) -> VersionRange {
        VersionRange {
            min: cmp::max(self.min, other.min),
            max: cmp::min(self.max, other.max),
        }
    }
}

pub trait Message: Sized {
    const VERSIONS: VersionRange;
}

pub trait Encodable: Sized {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError>;
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError>;
}

pub trait Decodable: Sized {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError>;
}

pub trait MapEncodable: Sized {
    type Key;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError>;
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError>;
}

pub trait MapDecodable: Sized {
    type Key;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError>;
}

pub trait HeaderVersion {
    fn header_version(version: i16) -> i16;
}

pub trait Request: Message + Encodable + Decodable + HeaderVersion {
    const KEY: i16;
    type Response: Message + Encodable + Decodable + HeaderVersion;
}

pub fn write_unknown_tagged_fields<B: ByteBufMut, R: RangeBounds<i32>>(
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

pub fn compute_unknown_tagged_fields_size(
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
