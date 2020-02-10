use bytes::{Buf, BufMut};

use super::{DecodeError, EncodeError, Encoder, Decoder, Primitive, Encodable, Decodable};

#[derive(Debug, Copy, Clone, Default)]
pub struct Boolean;

impl Primitive for Boolean {}

impl Encoder<bool> for Boolean {
    fn encode<B: BufMut>(&self, buf: &mut B, value: bool) -> Result<(), EncodeError> {
        Ok(buf.put_u8(if value { 1 } else { 0 }))
    }
}

impl Decoder<bool> for Boolean {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<bool, DecodeError> {
        Ok(buf.try_get_u8()? != 0)
    }
}

macro_rules! define_simple_ints {
    ($($name:ident: $t:ty [$put:ident, $get:ident],)*) => (
        $(
            #[derive(Debug, Copy, Clone)]
            pub struct $name;

            impl Primitive for $name {}

            impl Encoder<$t> for $name {
                fn encode<B: BufMut>(&self, buf: &mut B, value: $t) -> Result<(), EncodeError> {
                    Ok(buf.$put(value))
                }
            }

            impl Decoder<$t> for $name {
                fn decode<B: Buf>(&self, buf: &mut B) -> Result<$t, DecodeError> {
                    Ok(buf.$get()?)
                }
            }
        )*
    )
}

define_simple_ints!{
    Int8: i8 [put_i8, try_get_i8],
    Int16: i16 [put_i16, try_get_i16],
    Int32: i32 [put_i32, try_get_i32],
    Int64: i64 [put_i64, try_get_i64],
    UInt32: u32 [put_u32, try_get_u32],
}

#[derive(Debug, Copy, Clone, Default)]
pub struct UnsignedVarInt;

impl Primitive for UnsignedVarInt {}

impl Encoder<u32> for UnsignedVarInt {
    fn encode<B: BufMut>(&self, buf: &mut B, mut value: u32) -> Result<(), EncodeError> {
        while value >= 0x80 {
            buf.put_u8((value as u8) | 0x80);
            value >>= 7;
        }
        buf.put_u8(value as u8);
        Ok(())
    }
}

impl Decoder<u32> for UnsignedVarInt {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<u32, DecodeError> {
        let mut value = 0;
        for i in 0..5 {
            let b = buf.try_get_u8()? as u32;
            value |= (b & 0x7F) << (i*7);
            if b < 0x80 {
                break;
            }
        }
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct UnsignedVarLong;

impl Primitive for UnsignedVarLong {}

impl Encoder<u64> for UnsignedVarLong {
    fn encode<B: BufMut>(&self, buf: &mut B, mut value: u64) -> Result<(), EncodeError> {
        while value >= 0x80 {
            buf.put_u8((value as u8) | 0x80);
            value >>= 7;
        }
        buf.put_u8(value as u8);
        Ok(())
    }
}

impl Decoder<u64> for UnsignedVarLong {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<u64, DecodeError> {
        let mut value = 0;
        for i in 0..10 {
            let b = buf.try_get_u8()? as u64;
            value |= (b & 0x7F) << (i*7);
            if b < 0x80 {
                break;
            }
        }
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct VarInt;

impl Primitive for VarInt {}

impl Encoder<i32> for VarInt {
    fn encode<B: BufMut>(&self, buf: &mut B, value: i32) -> Result<(), EncodeError> {
        let zigzag = ((value << 1) ^ (value >> 31)) as u32;
        UnsignedVarInt.encode(buf, zigzag)
    }
}

impl Decoder<i32> for VarInt {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<i32, DecodeError> {
        let zigzag = UnsignedVarInt.decode(buf)?;
        Ok(((zigzag >> 1) as i32) ^ (-((zigzag & 1) as i32)))
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct VarLong;

impl Primitive for VarLong {}

impl Encoder<i64> for VarLong {
    fn encode<B: BufMut>(&self, buf: &mut B, value: i64) -> Result<(), EncodeError> {
        let zigzag = ((value << 1) ^ (value >> 63)) as u64;
        UnsignedVarLong.encode(buf, &zigzag)
    }
}

impl Decoder<i64> for VarLong {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<i64, DecodeError> {
        let zigzag = UnsignedVarLong.decode(buf)?;
        Ok(((zigzag >> 1) as i64) ^ (-((zigzag & 1) as i64)))
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Uuid;

impl Primitive for Uuid {}

impl Encoder<uuid::Uuid> for Uuid {
    fn encode<B: BufMut>(&self, buf: &mut B, value: uuid::Uuid) -> Result<(), EncodeError> {
        Ok(buf.put_slice(value.as_bytes()))
    }
}

impl Decoder<uuid::Uuid> for Uuid {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<uuid::Uuid, DecodeError> {
        let mut result = [0; 16];
        buf.try_copy_to_slice(&mut result)?;
        Ok(uuid::Uuid::from_bytes(result))
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct String;

impl Encoder<Option<&str>> for String {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&str>) -> Result<(), EncodeError> {
        if let Some(s) = value {
            if s.len() > std::i16::MAX as usize {
                error!("String is too long to encode ({} bytes)", s.len());
                Err(EncodeError)
            } else {
                Int16.encode(buf, s.len() as i16)?;
                buf.put_slice(s.as_bytes());
                Ok(())
            }
        } else {
            Int16.encode(buf, -1)?;
            Ok(())
        }
    }
}

impl Encoder<Option<&std::string::String>> for String {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&std::string::String>) -> Result<(), EncodeError> {
        String.encode(buf, value.map(|s| s.as_str()))
    }
}

impl Encoder<&Option<std::string::String>> for String {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<std::string::String>) -> Result<(), EncodeError> {
        String.encode(buf, value.as_ref())
    }
}

impl Encoder<&str> for String {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &str) -> Result<(), EncodeError> {
        String.encode(buf, Some(value))
    }
}

impl Encoder<&std::string::String> for String {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &std::string::String) -> Result<(), EncodeError> {
        String.encode(buf, Some(value))
    }
}

impl Decoder<Option<std::string::String>> for String {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<std::string::String>, DecodeError> {
        match Int16.decode(buf)? {
            -1 => Ok(None),
            n if n >= 0 => {
                let mut strbuf = vec![0; n as usize];
                buf.try_copy_to_slice(&mut strbuf)?;
                Ok(Some(std::string::String::from_utf8(strbuf)?))
            },
            n => {
                error!("String length is negative ({})", n);
                Err(DecodeError)
            }
        }
    }
}

impl Decoder<std::string::String> for String {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<std::string::String, DecodeError> {
        match String.decode(buf) {
            Ok(None) => {
                error!("String length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CompactString;

impl Encoder<Option<&str>> for CompactString {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&str>) -> Result<(), EncodeError> {
        if let Some(s) = value {
            // Use >= because we're going to add one to the length
            if s.len() >= std::u32::MAX as usize {
                error!("CompactString is too long to encode ({} bytes)", s.len());
                Err(EncodeError)
            } else {
                UnsignedVarInt.encode(buf, (s.len() as u32) + 1)?;
                buf.put_slice(s.as_bytes());
                Ok(())
            }
        } else {
            UnsignedVarInt.encode(buf, 0)?;
            Ok(())
        }
    }
}

impl Encoder<Option<&std::string::String>> for CompactString {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&std::string::String>) -> Result<(), EncodeError> {
        CompactString.encode(buf, value.map(|s| s.as_str()))
    }
}

impl Encoder<&Option<std::string::String>> for CompactString {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<std::string::String>) -> Result<(), EncodeError> {
        CompactString.encode(buf, value.as_ref())
    }
}

impl Encoder<&str> for CompactString {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &str) -> Result<(), EncodeError> {
        CompactString.encode(buf, Some(value))
    }
}

impl Encoder<&std::string::String> for CompactString {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &std::string::String) -> Result<(), EncodeError> {
        CompactString.encode(buf, Some(value))
    }
}

impl Decoder<Option<std::string::String>> for CompactString {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<std::string::String>, DecodeError> {
        match UnsignedVarInt.decode(buf)? {
            0 => Ok(None),
            n => {
                let mut strbuf = vec![0; (n-1) as usize];
                buf.try_copy_to_slice(&mut strbuf)?;
                Ok(Some(std::string::String::from_utf8(strbuf)?))
            },
        }
    }
}

impl Decoder<std::string::String> for CompactString {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<std::string::String, DecodeError> {
        match CompactString.decode(buf) {
            Ok(None) => {
                error!("CompactString length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Bytes;

impl Encoder<Option<&[u8]>> for Bytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&[u8]>) -> Result<(), EncodeError> {
        if let Some(s) = value {
            if s.len() > std::i32::MAX as usize {
                error!("Data is too long to encode ({} bytes)", s.len());
                Err(EncodeError)
            } else {
                Int32.encode(buf, s.len() as i32)?;
                buf.put_slice(s);
                Ok(())
            }
        } else {
            Int32.encode(buf, -1)?;
            Ok(())
        }
    }
}

impl Encoder<Option<&Vec<u8>>> for Bytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&Vec<u8>>) -> Result<(), EncodeError> {
        Bytes.encode(buf, value.map(|s| s.as_slice()))
    }
}

impl Encoder<&Option<Vec<u8>>> for Bytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<Vec<u8>>) -> Result<(), EncodeError> {
        Bytes.encode(buf, value.as_ref())
    }
}

impl Encoder<&[u8]> for Bytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &[u8]) -> Result<(), EncodeError> {
        Bytes.encode(buf, Some(value))
    }
}

impl Encoder<&Vec<u8>> for Bytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Vec<u8>) -> Result<(), EncodeError> {
        Bytes.encode(buf, Some(value))
    }
}

impl Decoder<Option<Vec<u8>>> for Bytes {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<Vec<u8>>, DecodeError> {
        match Int32.decode(buf)? {
            -1 => Ok(None),
            n if n >= 0 => {
                let mut data = vec![0; n as usize];
                buf.try_copy_to_slice(&mut data)?;
                Ok(Some(data))
            },
            n => {
                error!("Data length is negative ({})", n);
                Err(DecodeError)
            }
        }
    }
}

impl Decoder<Vec<u8>> for Bytes {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Vec<u8>, DecodeError> {
        match Bytes.decode(buf) {
            Ok(None) => {
                error!("Data length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CompactBytes;

impl Encoder<Option<&[u8]>> for CompactBytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&[u8]>) -> Result<(), EncodeError> {
        if let Some(s) = value {
            // Use >= because we're going to add one to the length
            if s.len() >= std::u32::MAX as usize {
                error!("CompactBytes is too long to encode ({} bytes)", s.len());
                Err(EncodeError)
            } else {
                UnsignedVarInt.encode(buf, (s.len() as u32) + 1)?;
                buf.put_slice(s);
                Ok(())
            }
        } else {
            UnsignedVarInt.encode(buf, 0)?;
            Ok(())
        }
    }
}

impl Encoder<Option<&Vec<u8>>> for CompactBytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&Vec<u8>>) -> Result<(), EncodeError> {
        CompactBytes.encode(buf, value.map(|s| s.as_slice()))
    }
}

impl Encoder<&Option<Vec<u8>>> for CompactBytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<Vec<u8>>) -> Result<(), EncodeError> {
        CompactBytes.encode(buf, value.as_ref())
    }
}

impl Encoder<&[u8]> for CompactBytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &[u8]) -> Result<(), EncodeError> {
        CompactBytes.encode(buf, Some(value))
    }
}

impl Encoder<&Vec<u8>> for CompactBytes {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Vec<u8>) -> Result<(), EncodeError> {
        CompactBytes.encode(buf, Some(value))
    }
}

impl Decoder<Option<Vec<u8>>> for CompactBytes {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<Vec<u8>>, DecodeError> {
        match UnsignedVarInt.decode(buf)? {
            0 => Ok(None),
            n => {
                let mut data = vec![0; (n-1) as usize];
                buf.try_copy_to_slice(&mut data)?;
                Ok(Some(data))
            },
        }
    }
}

impl Decoder<Vec<u8>> for CompactBytes {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Vec<u8>, DecodeError> {
        match CompactBytes.decode(buf) {
            Ok(None) => {
                error!("Data length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Struct;

impl<T: Encodable> Encoder<&T> for Struct {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &T) -> Result<(), EncodeError> {
        value.encode(buf)
    }
}

impl<T: Decodable> Decoder<T> for Struct {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<T, DecodeError> {
        T::decode(buf)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Array<E>(pub E);

impl<T, E: for<'a> Encoder<&'a T>> Encoder<Option<&[T]>> for Array<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&[T]>) -> Result<(), EncodeError> {
        if let Some(a) = value {
            if a.len() > std::i32::MAX as usize {
                error!("Array is too long to encode ({} items)", a.len());
                Err(EncodeError)
            } else {
                Int32.encode(buf, a.len() as i32)?;
                for item in a {
                    self.0.encode(buf, item)?;
                }
                Ok(())
            }
        } else {
            Int32.encode(buf, -1)?;
            Ok(())
        }
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<Option<&Vec<T>>> for Array<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&Vec<T>>) -> Result<(), EncodeError> {
        self.encode(buf, value.map(|s| s.as_slice()))
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&Option<Vec<T>>> for Array<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<Vec<T>>) -> Result<(), EncodeError> {
        self.encode(buf, value.as_ref())
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&[T]> for Array<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &[T]) -> Result<(), EncodeError> {
        self.encode(buf, Some(value))
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&Vec<T>> for Array<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Vec<T>) -> Result<(), EncodeError> {
        self.encode(buf, Some(value))
    }
}

impl<T, E: Decoder<T>> Decoder<Option<Vec<T>>> for Array<E> {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<Vec<T>>, DecodeError> {
        match Int32.decode(buf)? {
            -1 => Ok(None),
            n if n >= 0 => {
                let mut result = Vec::with_capacity(n as usize);
                for _ in 0..n {
                    result.push(self.0.decode(buf)?);
                }
                Ok(Some(result))
            },
            n => {
                error!("Array length is negative ({})", n);
                Err(DecodeError)
            }
        }
    }
}

impl<T, E: Decoder<T>> Decoder<Vec<T>> for Array<E> {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Vec<T>, DecodeError> {
        match self.decode(buf) {
            Ok(None) => {
                error!("Array length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CompactArray<E>(pub E);

impl<T, E: for<'a> Encoder<&'a T>> Encoder<Option<&[T]>> for CompactArray<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&[T]>) -> Result<(), EncodeError> {
        if let Some(a) = value {
            // Use >= because we're going to add one to the length
            if a.len() >= std::u32::MAX as usize {
                error!("CompactArray is too long to encode ({} items)", a.len());
                Err(EncodeError)
            } else {
                UnsignedVarInt.encode(buf, (a.len() as u32) + 1)?;
                for item in a {
                    self.0.encode(buf, item)?;
                }
                Ok(())
            }
        } else {
            UnsignedVarInt.encode(buf, 0)?;
            Ok(())
        }
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<Option<&Vec<T>>> for CompactArray<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Option<&Vec<T>>) -> Result<(), EncodeError> {
        self.encode(buf, value.map(|s| s.as_slice()))
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&Option<Vec<T>>> for CompactArray<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Option<Vec<T>>) -> Result<(), EncodeError> {
        self.encode(buf, value.as_ref())
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&[T]> for CompactArray<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &[T]) -> Result<(), EncodeError> {
        self.encode(buf, Some(value))
    }
}

impl<T, E: for<'a> Encoder<&'a T>> Encoder<&Vec<T>> for CompactArray<E> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &Vec<T>) -> Result<(), EncodeError> {
        self.encode(buf, Some(value))
    }
}

impl<T, E: Decoder<T>> Decoder<Option<Vec<T>>> for CompactArray<E> {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Option<Vec<T>>, DecodeError> {
        match UnsignedVarInt.decode(buf)? {
            0 => Ok(None),
            n => {
                let mut result = Vec::with_capacity((n-1) as usize);
                for _ in 0..n {
                    result.push(self.0.decode(buf)?);
                }
                Ok(Some(result))
            },
        }
    }
}

impl<T, E: Decoder<T>> Decoder<Vec<T>> for CompactArray<E> {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Vec<T>, DecodeError> {
        match self.decode(buf) {
            Ok(None) => {
                error!("CompactArray length is negative (-1)");
                Err(DecodeError)
            },
            Ok(Some(s)) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    fn test_encoder_decoder<V: PartialEq + Debug, E: for<'a> Encoder<&'a V> + Decoder<V>>(encoder: E, value: V, mut expected: &[u8]) {
        let mut buf = vec![];
        encoder.encode(&mut buf, &value).unwrap();
        assert_eq!(buf, expected);

        let decoded = encoder.decode(&mut expected).unwrap();
        assert_eq!(value, decoded);
    }

    #[test]
    fn smoke_varint_encoder_decoder() {
        test_encoder_decoder(VarInt, 0, &[0]);
        test_encoder_decoder(VarInt, -1, &[1]);
        test_encoder_decoder(VarInt, 1, &[2]);
        test_encoder_decoder(VarInt, -2, &[3]);
        test_encoder_decoder(VarInt, 300, &[216, 4]);
        test_encoder_decoder(VarInt, std::i32::MAX, &[254, 255, 255, 255, 15]);
        test_encoder_decoder(VarInt, std::i32::MIN, &[255, 255, 255, 255, 15]);
    }

    #[test]
    fn smoke_varlong_encoder_decoder() {
        test_encoder_decoder(VarLong, 0, &[0]);
        test_encoder_decoder(VarLong, -1, &[1]);
        test_encoder_decoder(VarLong, 1, &[2]);
        test_encoder_decoder(VarLong, -2, &[3]);
        test_encoder_decoder(VarLong, 300, &[216, 4]);
        test_encoder_decoder(VarLong, std::i64::MAX, &[254, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
        test_encoder_decoder(VarLong, std::i64::MIN, &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    }

    #[test]
    fn smoke_string_encoder_decoder() {
        test_encoder_decoder(String, std::string::String::from("hello"), &[0, 5, 104, 101, 108, 108, 111]);
        test_encoder_decoder(String, None::<std::string::String>, &[255, 255]);
    }

    #[test]
    fn smoke_compact_string_encoder_decoder() {
        test_encoder_decoder(CompactString, std::string::String::from("hello"), &[6, 104, 101, 108, 108, 111]);
        test_encoder_decoder(CompactString, None::<std::string::String>, &[0]);
    }

    #[test]
    fn smoke_bytes_encoder_decoder() {
        test_encoder_decoder(Bytes, vec![1,2,3,4], &[0, 0, 0, 4, 1, 2, 3, 4]);
        test_encoder_decoder(Bytes, None::<Vec<u8>>, &[255, 255, 255, 255]);
    }

    #[test]
    fn smoke_compact_bytes_encoder_decoder() {
        test_encoder_decoder(CompactBytes, vec![1,2,3,4], &[5, 1, 2, 3, 4]);
        test_encoder_decoder(CompactBytes, None::<Vec<u8>>, &[0]);
    }

}
