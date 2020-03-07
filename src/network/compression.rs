use franz_protocol::buf::{ByteBuf, ByteBufMut};
use franz_protocol::{EncodeError, DecodeError};

mod none;
mod snappy;
mod gzip;

pub use none::None;
pub use snappy::Snappy;
pub use gzip::Gzip;

pub trait Compressor<B: ByteBufMut> {
    type BufMut: ByteBufMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>;
}

pub trait Decompressor<B: ByteBuf> {
    type Buf: ByteBuf;
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>;
}
