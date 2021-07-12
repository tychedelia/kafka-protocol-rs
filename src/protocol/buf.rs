//! Utilities for working with the [`bytes`] crate.
use std::io::Cursor;
use std::mem::size_of;
use std::ops::Range;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error indicating there are not enough remaining bytes in a buffer to perform a read.
#[derive(Debug)]
pub struct NotEnoughBytesError;

impl Display for NotEnoughBytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Not enough bytes remaining in buffer!")
    }
}

impl Error for NotEnoughBytesError {}

/// Extension for working with [`bytes::Buf`].
pub trait ByteBuf: Buf {
    /// Peek ahead in the buffer by the provided range.
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes;
    /// Get `size` bytes from the underlying buffer.
    fn get_bytes(&mut self, size: usize) -> Bytes;
    /// Try to peek ahead in the buffer bu the provided range, returning an error if there are less
    /// bytes than the requested range.
    fn try_peek_bytes(&mut self, r: Range<usize>) -> Result<Bytes, NotEnoughBytesError> {
        if self.remaining() < r.end {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.peek_bytes(r))
        }
    }
    /// Try to get `size` bytes from the buffer, returning an error if there are less bytes than the
    /// requested number.
    fn try_get_bytes(&mut self, size: usize) -> Result<Bytes, NotEnoughBytesError> {
        if self.remaining() < size {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_bytes(size))
        }
    }
    /// Attempt to copy from buffer into destination slice, returning an error if not enough space
    /// remains.
    fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), NotEnoughBytesError> {
        if self.remaining() < dst.len() {
            Err(NotEnoughBytesError)
        } else {
            self.copy_to_slice(dst);
            Ok(())
        }
    }
    /// Attempt to read a `u8` from the buffer, returning an error if not enough space remains.
    fn try_get_u8(&mut self) -> Result<u8, NotEnoughBytesError> {
        if self.remaining() < size_of::<u8>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_u8())
        }
    }
    /// Attempt to read a `u16` from the buffer, returning an error if not enough space remains.
    fn try_get_u16(&mut self) -> Result<u16, NotEnoughBytesError> {
        if self.remaining() < size_of::<u16>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_u16())
        }
    }
    /// Attempt to read a `u32` from the buffer, returning an error if not enough space remains.
    fn try_get_u32(&mut self) -> Result<u32, NotEnoughBytesError> {
        if self.remaining() < size_of::<u32>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_u32())
        }
    }
    /// Attempt to read a `i8` from the buffer, returning an error if not enough space remains.
    fn try_get_i8(&mut self) -> Result<i8, NotEnoughBytesError> {
        if self.remaining() < size_of::<i8>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_i8())
        }
    }
    /// Attempt to read a `i16` from the buffer, returning an error if not enough space remains.
    fn try_get_i16(&mut self) -> Result<i16, NotEnoughBytesError> {
        if self.remaining() < size_of::<i16>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_i16())
        }
    }
    /// Attempt to read a `i32` from the buffer, returning an error if not enough space remains.
    fn try_get_i32(&mut self) -> Result<i32, NotEnoughBytesError> {
        if self.remaining() < size_of::<i32>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_i32())
        }
    }
    /// Attempt to read a `i64` from the buffer, returning an error if not enough space remains.
    fn try_get_i64(&mut self) -> Result<i64, NotEnoughBytesError> {
        if self.remaining() < size_of::<i64>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_i64())
        }
    }
    /// Attempt to read a `f32` from the buffer, returning an error if not enough space remains.
    fn try_get_f64(&mut self) -> Result<f64, NotEnoughBytesError> {
        if self.remaining() < size_of::<f64>() {
            Err(NotEnoughBytesError)
        } else {
            Ok(self.get_f64())
        }
    }
}

impl ByteBuf for Bytes {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes {
        self.slice(r)
    }
    fn get_bytes(&mut self, size: usize) -> Bytes {
        self.split_to(size)
    }
}

impl ByteBuf for BytesMut {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes {
        Bytes::copy_from_slice(&self[r])
    }
    fn get_bytes(&mut self, size: usize) -> Bytes {
        self.split_to(size).freeze()
    }
}

impl<T: ByteBuf> ByteBuf for &mut T {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes {
        (**self).peek_bytes(r)
    }
    fn get_bytes(&mut self, size: usize) -> Bytes {
        (**self).get_bytes(size)
    }
    fn try_peek_bytes(&mut self, r: Range<usize>) -> Result<Bytes, NotEnoughBytesError> {
        (**self).try_peek_bytes(r)
    }
    fn try_get_bytes(&mut self, size: usize) -> Result<Bytes, NotEnoughBytesError> {
        (**self).try_get_bytes(size)
    }
}

impl ByteBuf for &[u8] {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes {
        Bytes::copy_from_slice(&self[r])
    }
    fn get_bytes(&mut self, size: usize) -> Bytes {
        let (a, b) = self.split_at(size);
        *self = b;
        Bytes::copy_from_slice(a)
    }
}

impl<T: AsRef<[u8]>> ByteBuf for Cursor<T> {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes {
        Bytes::copy_from_slice(&self.get_ref().as_ref()[r])
    }
    fn get_bytes(&mut self, size: usize) -> Bytes {
        let pos = self.position() as usize;
        self.set_position((pos + size) as u64);
        Bytes::copy_from_slice(&self.get_ref().as_ref()[pos..(pos + size)])
    }
}

/// A gap of specified length at the specified offset.
#[derive(Debug, Copy, Clone)]
pub struct Gap {
    offset: usize,
    len: usize,
}

/// A type capable of being represented as a gap in a buffer.
pub trait GapType {
    /// The type of the gap.
    type Value;
    /// The size of the gap.
    fn size(&self) -> usize;
    /// Insert a value into the provided buffer.
    fn put(&self, buf: &mut [u8], value: Self::Value);
}

/// A gap of type `T`.
pub struct TypedGap<T> {
    gap: Gap,
    type_: T,
}

macro_rules! define_gap_types {
    {$($n:ident => $f:ident($t:ty)),*$(,)*} => {
        pub(crate) mod gap {
            use super::*;
            $(
                #[derive(Copy, Clone, Debug)]
                pub(crate) struct $n;

                impl GapType for $n {
                    type Value = $t;
                    fn size(&self) -> usize {
                        std::mem::size_of::<Self::Value>()
                    }
                    fn put(&self, mut buf: &mut [u8], value: Self::Value) {
                        buf.$f(value);
                    }
                }
            )*
        }
    };
}

define_gap_types! {
    U8 => put_u8(u8),
    I8 => put_i8(i8),
    U16 => put_u16(u16),
    U16Le => put_u16_le(u16),
    I16 => put_i16(i16),
    I16Le => put_i16_le(i16),
    U32 => put_u32(u32),
    U32Le => put_u32_le(u32),
    I32 => put_i32(i32),
    I32Le => put_i32_le(i32),
    U64 => put_u64(u64),
    U64Le => put_u64_le(u64),
    I64 => put_i64(i64),
    I64Le => put_i64_le(i64),
    U128 => put_u128(u128),
    U128Le => put_u128_le(u128),
    I128 => put_i128(i128),
    I128Le => put_i128_le(i128),
    F32 => put_f32(f32),
    F32Le => put_f32_le(f32),
    F64 => put_f64(f64),
    F64Le => put_f64_le(f64),
}

/// Extension for working with [`bytes::buf::BufMut`].
pub trait ByteBufMut: BufMut {
    /// Get the current offset of the buffer.
    fn offset(&self) -> usize;

    /// Seek to the provided offset in the buffer.
    fn seek(&mut self, offset: usize);

    /// Read a range from the buffer.
    fn range(&mut self, r: Range<usize>) -> &mut [u8];

    /// Put a gap of `len` at the current buffer offset.
    fn put_gap(&mut self, len: usize) -> Gap {
        let res = Gap { offset: self.offset(), len };
        self.seek(res.offset + len);
        res
    }

    /// Read a gap from the buffer.
    fn gap_buf(&mut self, gap: Gap) -> &mut [u8] {
        self.range(gap.offset..(gap.offset + gap.len))
    }

    /// Put a typed gap of type `T` at the current buffer offset.
    fn put_typed_gap<T: GapType>(&mut self, type_: T) -> TypedGap<T> {
        TypedGap {
            gap: self.put_gap(type_.size()),
            type_,
        }
    }

    /// Insert a value of the [`TypedGap`] type at the current buffer offset.
    fn fill_typed_gap<T: GapType>(&mut self, gap: TypedGap<T>, value: T::Value) {
        gap.type_.put(self.gap_buf(gap.gap), value);
    }
}

impl ByteBufMut for BytesMut {
    fn offset(&self) -> usize {
        self.len()
    }
    fn seek(&mut self, offset: usize) {
        self.resize(offset, 0);
    }
    fn range(&mut self, r: Range<usize>) -> &mut [u8] {
        &mut self[r]
    }
}

impl ByteBufMut for Vec<u8> {
    fn offset(&self) -> usize {
        self.len()
    }
    fn seek(&mut self, offset: usize) {
        self.resize(offset, 0);
    }
    fn range(&mut self, r: Range<usize>) -> &mut [u8] {
        &mut self[r]
    }
}

impl<T: ByteBufMut> ByteBufMut for &mut T {
    fn offset(&self) -> usize {
        (**self).offset()
    }
    fn seek(&mut self, offset: usize) {
        (**self).seek(offset)
    }
    fn range(&mut self, r: Range<usize>) -> &mut [u8] {
        (**self).range(r)
    }
}
