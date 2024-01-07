//! Provides compression utilities for encoding records.
//!
//! This module has implementations of gzip, Snappy, as well as a noop compression format that
//! allows encoding and decoding records into a [`Record`](crate::records::Record).

use std::fmt::Debug;
use crate::protocol::buf::{ByteBuf, ByteBufMut};
use crate::protocol::{DecodeError, EncodeError};

mod gzip;
mod none;
mod snappy;
mod zstd;
mod lz4;

pub use gzip::Gzip;
pub use none::None;
pub use snappy::Snappy;
pub use zstd::Zstd;
pub use lz4::Lz4;

pub(crate) fn compression_err<Error: Debug>(e: Error) -> EncodeError {
    error!("Error whilst compressing data: {:?}", e);
    EncodeError
}

pub(crate) fn decompression_err<Error: Debug>(e: Error) -> DecodeError {
    error!("Error whilst decompressing data: {:?}", e);
    DecodeError
}

/// A trait for record compression algorithms.
pub trait Compressor<B: ByteBufMut> {
    /// Target buffer type for compression.
    type BufMut: ByteBufMut;
    /// Compresses into provided [`ByteBufMut`], with records encoded by `F` into `R`.
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>;
}

/// A trait for record decompression algorithms.
pub trait Decompressor<B: ByteBuf> {
    /// Target buffer type for decompression.
    type Buf: ByteBuf;
    /// Decompress records from `B` mapped using `F` into `R`.
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>;
}
