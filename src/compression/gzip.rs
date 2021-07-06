use std::io::Write;

use bytes::{Bytes, BytesMut};
use bytes::buf::BufMutExt;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};
use log::error;

use protocol_base::buf::{ByteBuf, ByteBufMut};
use protocol_base::{EncodeError, DecodeError};

use super::{Compressor, Decompressor};

pub struct Gzip;

fn compression_err(e: std::io::Error) -> EncodeError {
    error!("Error whilst compressing data: {}", e);
    EncodeError
}

fn decompression_err(e: std::io::Error) -> DecodeError {
    error!("Error whilst decompressing data: {}", e);
    DecodeError
}

impl<B: ByteBufMut> Compressor<B> for Gzip {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        // Compress directly into the target buffer
        let mut e = GzEncoder::new(buf.writer(), Compression::default());
        e.write_all(&tmp).map_err(compression_err)?;
        e.finish().map_err(compression_err)?;

        Ok(res)
    }
}

impl<B: ByteBuf> Decompressor<B> for Gzip {
    type Buf = Bytes;
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>
    {
        let mut tmp = BytesMut::new();

        // Decompress directly from the input buffer
        let mut d = GzDecoder::new((&mut tmp).writer());
        d.write_all(&buf.to_bytes()).map_err(decompression_err)?;
        d.finish().map_err(decompression_err)?;

        f(&mut tmp.into())
    }
}
