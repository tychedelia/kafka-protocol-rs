use crate::protocol::buf::{ByteBuf, ByteBufMut};
use anyhow::{Context, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use lz4::BlockMode;
use lz4::{Decoder, EncoderBuilder};
use std::io;

use super::{Compressor, Decompressor};

/// Gzip compression algorithm. See [Kafka's broker configuration](https://kafka.apache.org/documentation/#brokerconfigs_compression.type)
/// for more information.
pub struct Lz4;

const COMPRESSION_LEVEL: u32 = 4;

impl<B: ByteBufMut> Compressor<B> for Lz4 {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R>,
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        let mut encoder = EncoderBuilder::new()
            .level(COMPRESSION_LEVEL)
            .block_mode(BlockMode::Independent)
            .build(buf.writer())
            .context("Failed to compress lz4")?;

        io::copy(&mut tmp.reader(), &mut encoder).context("Failed to compress lz4")?;
        encoder.finish().1.context("Failed to compress lz4")?;

        Ok(res)
    }
}
