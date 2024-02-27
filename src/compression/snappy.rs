use bytes::{Bytes, BytesMut};
use snap::raw::*;

use crate::protocol::buf::{ByteBuf, ByteBufMut};
use crate::protocol::{DecodeError, EncodeError};

use super::{Compressor, Decompressor};

/// Snappy compression algorithm. See [Kafka's broker configuration](https://kafka.apache.org/documentation/#brokerconfigs_compression.type)
/// for more information.
pub struct Snappy;

impl<B: ByteBufMut> Compressor<B> for Snappy {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>,
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        // Compress directly into the target buffer
        let start_pos = buf.offset();
        let compress_gap = buf.put_gap(max_compress_len(tmp.len()));
        let actual_len = Encoder::new().compress(&tmp, buf.gap_buf(compress_gap))?;
        buf.seek(start_pos + actual_len);

        Ok(res)
    }
}

impl<B: ByteBuf> Decompressor<B> for Snappy {
    type Buf = Bytes;
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>,
    {
        // Allocate a temporary buffer to hold the uncompressed bytes
        let buf = buf.copy_to_bytes(buf.remaining());
        let actual_len = decompress_len(&buf)?;
        let mut tmp = BytesMut::new();
        tmp.resize(actual_len, 0);

        // Decompress directly from the input buffer
        Decoder::new().decompress(&buf, &mut tmp)?;

        f(&mut tmp.into())
    }
}
