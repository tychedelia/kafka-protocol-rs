//! InitProducerIdRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/InitProducerIdRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    Decoder, Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes,
    VersionRange,
};

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct InitProducerIdRequest {
    /// The transactional id, or null if the producer is not transactional.
    ///
    /// Supported API versions: 0-4
    pub transactional_id: Option<super::TransactionalId>,

    /// The time in ms to wait before aborting idle transactions sent by this producer. This is only relevant if a TransactionalId has been defined.
    ///
    /// Supported API versions: 0-4
    pub transaction_timeout_ms: i32,

    /// The producer id. This is used to disambiguate requests if a transactional id is reused following its expiration.
    ///
    /// Supported API versions: 3-4
    pub producer_id: super::ProducerId,

    /// The producer's current epoch. This will be checked against the producer epoch on the broker, and the request will return an error if they do not match.
    ///
    /// Supported API versions: 3-4
    pub producer_epoch: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for InitProducerIdRequest {
    type Builder = InitProducerIdRequestBuilder;

    fn builder() -> Self::Builder {
        InitProducerIdRequestBuilder::default()
    }
}

impl Encodable for InitProducerIdRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 2 {
            types::CompactString.encode(buf, &self.transactional_id)?;
        } else {
            types::String.encode(buf, &self.transactional_id)?;
        }
        types::Int32.encode(buf, &self.transaction_timeout_ms)?;
        if version >= 3 {
            types::Int64.encode(buf, &self.producer_id)?;
        } else {
            if self.producer_id != -1 {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            types::Int16.encode(buf, &self.producer_epoch)?;
        } else {
            if self.producer_epoch != -1 {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.transactional_id)?;
        } else {
            total_size += types::String.compute_size(&self.transactional_id)?;
        }
        total_size += types::Int32.compute_size(&self.transaction_timeout_ms)?;
        if version >= 3 {
            total_size += types::Int64.compute_size(&self.producer_id)?;
        } else {
            if self.producer_id != -1 {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            total_size += types::Int16.compute_size(&self.producer_epoch)?;
        } else {
            if self.producer_epoch != -1 {
                bail!("failed to encode");
            }
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for InitProducerIdRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let transactional_id = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let transaction_timeout_ms = types::Int32.decode(buf)?;
        let producer_id = if version >= 3 {
            types::Int64.decode(buf)?
        } else {
            (-1).into()
        };
        let producer_epoch = if version >= 3 {
            types::Int16.decode(buf)?
        } else {
            -1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            transactional_id,
            transaction_timeout_ms,
            producer_id,
            producer_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for InitProducerIdRequest {
    fn default() -> Self {
        Self {
            transactional_id: Some(Default::default()),
            transaction_timeout_ms: 0,
            producer_id: (-1).into(),
            producer_epoch: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for InitProducerIdRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for InitProducerIdRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}
