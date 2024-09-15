use crate::protocol::buf::{ByteBuf, ByteBufMut};
use anyhow::{Context, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{Compressor, Decompressor};

/// Gzip compression algorithm. See [Kafka's broker configuration](https://kafka.apache.org/documentation/#brokerconfigs_compression.type)
/// for more information.
pub struct Zstd;

const COMPRESSION_LEVEL: i32 = 3;

impl<B: ByteBufMut> Compressor<B> for Zstd {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R>,
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        // Compress directly into the target buffer
        zstd::stream::copy_encode(tmp.reader(), buf.writer(), COMPRESSION_LEVEL)
            .context("Failed to compress zstd")?;
        Ok(res)
    }
}
