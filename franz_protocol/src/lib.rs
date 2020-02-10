#[macro_use]
extern crate log;

use std::error::Error;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::cmp;

use bytes::{Buf, BufMut, Bytes};

pub mod types;

#[derive(Debug)]
pub struct DecodeError;

#[derive(Debug)]
pub struct EncodeError;

impl<E: Error> From<E> for DecodeError {
    fn from(other: E) -> Self {
        error!("{}", other);
        DecodeError
    }
}

pub trait Encoder<Value> {
    fn encode<B: BufMut>(&self, buf: &mut B, value: Value) -> Result<(), EncodeError>;
}
pub trait Decoder<Value> {
    fn decode<B: Buf>(&self, buf: &mut B) -> Result<Value, DecodeError>;
}

pub trait Primitive {}

impl<T: Copy, E: Encoder<T> + Primitive> Encoder<&T> for E {
    fn encode<B: BufMut>(&self, buf: &mut B, value: &T) -> Result<(), EncodeError> {
        self.encode(buf, *value)
    }
}

pub trait Encodable {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), EncodeError>;
}

pub trait Decodable: Sized {
    fn decode<B: Buf>(buf: &mut B) -> Result<Self, DecodeError>;
}

#[derive(Debug, Clone)]
pub struct DowngradeError;

pub trait MaybeDowngrade<R: Request>: MaybeDowngradeExt<R> {
    type Request;
    type Response;

    fn downgrade(req: R) -> Result<Self::Request, DowngradeError>;
    fn upgrade_response(other: Self::Response) -> R::Response;
}

pub trait MaybeUpgrade<R: Request>: MaybeUpgradeExt<R> {
    type Request;
    type Response;

    fn upgrade(req: R) -> Self::Request;
    fn downgrade_response(other: Self::Response) -> Result<R::Response, DowngradeError>;
}

#[derive(Debug)]
enum TransformDirection {
    Upgrade,
    Downgrade,
}

#[derive(Debug)]
pub struct DecodeCompatible<R> {
    version: i16,
    direction: TransformDirection,
    phantom: PhantomData<fn() -> R>,
}

impl<R> DecodeCompatible<R> {
    fn new(version: i16, direction: TransformDirection) -> Self {
        DecodeCompatible { version, direction, phantom: PhantomData }
    }
}

impl<R: Request> DecodeCompatible<R> {
    pub fn decode_compatible(&self, buf: &mut Bytes) -> Result<R::Response, CompatibilityError> {
        match self.direction {
            TransformDirection::Downgrade => R::Downgrade::decode_compatible(buf, self.version),
            TransformDirection::Upgrade => R::Upgrade::decode_compatible(buf, self.version),
        }
    }
}

pub trait MaybeDowngradeExt<R: Request> {
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError>;
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, version: i16) -> Result<(), CompatibilityError>;
    fn find_compatible_version(max_version: i16) -> Result<i16, CompatibilityError>;
}

impl<R: Request> MaybeDowngradeExt<R> for Downgrade
where
    Self: MaybeDowngrade<R>,
    <Self as MaybeDowngrade<R>>::Request: Request,
    Self: MaybeDowngrade<R, Response=<<Self as MaybeDowngrade<R>>::Request as Request>::Response>,
{
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError> {
        Ok(if R::VERSION > version {
            let decoded = <<Self as MaybeDowngrade<R>>::Request as Request>::Downgrade::decode_compatible(buf, version)?;
            <Self as MaybeDowngrade<R>>::upgrade_response(decoded)
        } else {
            R::Response::decode(buf)?
        })
    }
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, version: i16) -> Result<(), CompatibilityError> {
        if R::VERSION > version {
            <<Self as MaybeDowngrade<R>>::Request as Request>::Downgrade::encode_compatible(buf, Self::downgrade(req)?, version)
        } else {
            req.encode(buf)?;
            Ok(())
        }
    }
    fn find_compatible_version(max_version: i16) -> Result<i16, CompatibilityError> {
        if R::VERSION > max_version {
            <<Self as MaybeDowngrade<R>>::Request as Request>::Downgrade::find_compatible_version(max_version)
        } else {
            Ok(max_version)
        }
    }
}

impl<R: Request> MaybeDowngradeExt<R> for NoDowngrade {
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError> {
        assert!(version >= R::VERSION);
        Ok(R::Response::decode(buf)?)
    }
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, version: i16) -> Result<(), CompatibilityError> {
        if version >= R::VERSION {
            req.encode(buf)?;
            Ok(())
        } else {
            Err(CompatibilityError::Downgrade)
        }
    }
    fn find_compatible_version(max_version: i16) -> Result<i16, CompatibilityError> {
        if max_version >= R::VERSION {
            Ok(max_version)
        } else {
            Err(CompatibilityError::Downgrade)
        }
    }
}

pub trait MaybeUpgradeExt<R: Request> {
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError>;
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, version: i16) -> Result<(), CompatibilityError>;
    fn find_compatible_version(min_version: i16) -> Result<i16, CompatibilityError>;
}

impl<R: Request> MaybeUpgradeExt<R> for Upgrade
where
    Self: MaybeUpgrade<R>,
    <Self as MaybeUpgrade<R>>::Request: Request,
    Self: MaybeUpgrade<R, Response=<<Self as MaybeUpgrade<R>>::Request as Request>::Response>,
{
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError> {
        Ok(if <<Self as MaybeUpgrade<R>>::Request as Request>::VERSION <= version {
            let decoded = <<Self as MaybeUpgrade<R>>::Request as Request>::Upgrade::decode_compatible(buf, version)?;
            <Self as MaybeUpgrade<R>>::downgrade_response(decoded)?
        } else {
            R::Response::decode(buf)?
        })
    }
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, version: i16) -> Result<(), CompatibilityError> {
        if <<Self as MaybeUpgrade<R>>::Request as Request>::VERSION <= version {
            <<Self as MaybeUpgrade<R>>::Request as Request>::Upgrade::encode_compatible(buf, Self::upgrade(req), version)
        } else {
            req.encode(buf)?;
            Ok(())
        }
    }
    fn find_compatible_version(min_version: i16) -> Result<i16, CompatibilityError> {
        if <<Self as MaybeUpgrade<R>>::Request as Request>::VERSION <= min_version {
            <<Self as MaybeUpgrade<R>>::Request as Request>::Upgrade::find_compatible_version(min_version)
        } else {
            Ok(min_version)
        }
    }
}

impl<R: Request> MaybeUpgradeExt<R> for NoUpgrade {
    fn decode_compatible(buf: &mut Bytes, version: i16) -> Result<R::Response, CompatibilityError> {
        assert!(version <= R::Type::MAX_VERSION);
        Ok(R::Response::decode(buf)?)
    }
    fn encode_compatible<B: BufMut>(buf: &mut B, req: R, min_version: i16) -> Result<(), CompatibilityError> {
        if min_version <= R::Type::MAX_VERSION {
            req.encode(buf)?;
            Ok(())
        } else {
            Err(CompatibilityError::Upgrade)
        }
    }
    fn find_compatible_version(min_version: i16) -> Result<i16, CompatibilityError> {
        if min_version <= R::Type::MAX_VERSION {
            Ok(min_version)
        } else {
            Err(CompatibilityError::Upgrade)
        }
    }
}

#[derive(Debug)]
pub struct Downgrade;
#[derive(Debug)]
pub struct NoDowngrade;

#[derive(Debug)]
pub struct Upgrade;
#[derive(Debug)]
pub struct NoUpgrade;

impl<R: Request> MaybeDowngrade<R> for NoDowngrade {
    type Request = Infallible;
    type Response = Infallible;

    fn downgrade(_req: R) -> Result<Infallible, DowngradeError> {
        unreachable!()
    }
    fn upgrade_response(_other: Infallible) -> R::Response {
        unreachable!()
    }
}

impl<R: Request> MaybeUpgrade<R> for NoUpgrade {
    type Request = Infallible;
    type Response = Infallible;

    fn upgrade(_req: R) -> Infallible {
        unreachable!()
    }
    fn downgrade_response(_other: Infallible) -> Result<R::Response, DowngradeError> {
        unreachable!()
    }
}

#[derive(Debug)]
pub enum CompatibilityError {
    Encode,
    Decode,
    Downgrade,
    Upgrade,
}

impl From<DowngradeError> for CompatibilityError {
    fn from(_other: DowngradeError) -> Self {
        CompatibilityError::Downgrade
    }
}

impl From<EncodeError> for CompatibilityError {
    fn from(_other: EncodeError) -> Self {
        CompatibilityError::Encode
    }
}

impl From<DecodeError> for CompatibilityError {
    fn from(_other: DecodeError) -> Self {
        CompatibilityError::Decode
    }
}

pub trait RequestType {
    const KEY: i16;
    const MAX_VERSION: i16;
}

pub trait Request: Encodable + Sized {
    type Type: RequestType;
    const VERSION: i16;
    type Response: Decodable;
    type Downgrade: MaybeDowngrade<Self>;
    type Upgrade: MaybeUpgrade<Self>;

    fn find_compatible_version(min_version: i16, max_version: i16) -> Result<i16, CompatibilityError> {
        if Self::VERSION < min_version {
            Self::Upgrade::find_compatible_version(min_version)
        } else {
            Self::Downgrade::find_compatible_version(cmp::min(Self::VERSION, max_version))
        }
    }

    fn encode_compatible<B: BufMut>(self, buf: &mut B, version: i16) -> Result<DecodeCompatible<Self>, CompatibilityError> {
        Ok(if Self::VERSION < version {
            Self::Upgrade::encode_compatible(buf, self, version)?;
            DecodeCompatible::new(version, TransformDirection::Upgrade)
        } else {
            Self::Downgrade::encode_compatible(buf, self, version)?;
            DecodeCompatible::new(version, TransformDirection::Downgrade)
        })
    }
}
