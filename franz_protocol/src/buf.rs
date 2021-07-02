use std::io::Cursor;
use std::mem::size_of;
use std::ops::Range;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::error::ErrorKind;

pub trait ByteBuf: Buf {
    fn peek_bytes(&mut self, r: Range<usize>) -> Bytes;
    fn get_bytes(&mut self, size: usize) -> Bytes;
    fn try_peek_bytes(&mut self, r: Range<usize>) -> Result<Bytes, ErrorKind> {
        if self.remaining() < r.end {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.peek_bytes(r))
        }
    }
    fn try_get_bytes(&mut self, size: usize) -> Result<Bytes, ErrorKind> {
        if self.remaining() < size {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_bytes(size))
        }
    }
    fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), ErrorKind> {
        if self.remaining() < dst.len() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.copy_to_slice(dst))
        }
    }
    fn try_get_u8(&mut self) -> Result<u8, ErrorKind> {
        if self.remaining() < size_of::<u8>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_u8())
        }
    }
    fn try_get_u32(&mut self) -> Result<u32, ErrorKind> {
        if self.remaining() < size_of::<u32>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_u32())
        }
    }
    fn try_get_i8(&mut self) -> Result<i8, ErrorKind> {
        if self.remaining() < size_of::<i8>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_i8())
        }
    }
    fn try_get_i16(&mut self) -> Result<i16, ErrorKind> {
        if self.remaining() < size_of::<i16>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_i16())
        }
    }
    fn try_get_i32(&mut self) -> Result<i32, ErrorKind> {
        if self.remaining() < size_of::<i32>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_i32())
        }
    }
    fn try_get_i64(&mut self) -> Result<i64, ErrorKind> {
        if self.remaining() < size_of::<i64>() {
            Err(ErrorKind::NotEnoughBytes)
        } else {
            Ok(self.get_i64())
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
    fn try_peek_bytes(&mut self, r: Range<usize>) -> Result<Bytes, ErrorKind> {
        (**self).try_peek_bytes(r)
    }
    fn try_get_bytes(&mut self, size: usize) -> Result<Bytes, ErrorKind> {
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


#[derive(Debug, Copy, Clone)]
pub struct Gap {
    offset: usize,
    len: usize,
}

pub trait GapType {
    type Value;
    fn size(&self) -> usize;
    fn put(&self, buf: &mut [u8], value: Self::Value);
}

pub struct TypedGap<T> {
    gap: Gap,
    type_: T,
}

macro_rules! define_gap_types {
    {$($n:ident => $f:ident($t:ty)),*$(,)*} => {
        pub mod gap {
            use super::*;
            $(
                #[derive(Copy, Clone, Debug)]
                pub struct $n;

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
    U16LE => put_u16_le(u16),
    I16 => put_i16(i16),
    I16LE => put_i16_le(i16),
    U32 => put_u32(u32),
    U32LE => put_u32_le(u32),
    I32 => put_i32(i32),
    I32LE => put_i32_le(i32),
    U64 => put_u64(u64),
    U64LE => put_u64_le(u64),
    I64 => put_i64(i64),
    I64LE => put_i64_le(i64),
    U128 => put_u128(u128),
    U128LE => put_u128_le(u128),
    I128 => put_i128(i128),
    I128LE => put_i128_le(i128),
    F32 => put_f32(f32),
    F32LE => put_f32_le(f32),
    F64 => put_f64(f64),
    F64LE => put_f64_le(f64),
}

pub trait ByteBufMut: BufMut {
    fn offset(&self) -> usize;
    fn seek(&mut self, offset: usize);
    fn range(&mut self, r: Range<usize>) -> &mut [u8];

    fn put_gap(&mut self, len: usize) -> Gap {
        let res = Gap { offset: self.offset(), len };
        self.seek(res.offset + len);
        res
    }
    fn gap_buf(&mut self, gap: Gap) -> &mut [u8] {
        self.range(gap.offset..(gap.offset + gap.len))
    }
    fn put_typed_gap<T: GapType>(&mut self, type_: T) -> TypedGap<T> {
        TypedGap {
            gap: self.put_gap(type_.size()),
            type_,
        }
    }
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
