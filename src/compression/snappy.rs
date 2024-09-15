use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use snap::raw::*;

use crate::protocol::buf::{ByteBuf, ByteBufMut};

use super::{Compressor, Decompressor};

/// Snappy compression algorithm. See [Kafka's broker configuration](https://kafka.apache.org/documentation/#brokerconfigs_compression.type)
/// for more information.
pub struct Snappy;

impl<B: ByteBufMut> Compressor<B> for Snappy {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R>,
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        // Compress directly into the target buffer
        let start_pos = buf.offset();
        let compress_gap = buf.put_gap(max_compress_len(tmp.len()));
        let actual_len = Encoder::new()
            .compress(&tmp, buf.gap_buf(compress_gap))
            .context("Failed to compress snappy")?;
        buf.seek(start_pos + actual_len);

        Ok(res)
    }
}
