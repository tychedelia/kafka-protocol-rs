//! LeaderChangeMessage
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/LeaderChangeMessage.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct Voter {
    /// 
    /// 
    /// Supported API versions: 0
    pub voter_id: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for Voter {
    type Builder = VoterBuilder;

    fn builder() -> Self::Builder{
        VoterBuilder::default()
    }
}

impl Encodable for Voter {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.voter_id)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.voter_id)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for Voter {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let voter_id = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            voter_id,
            unknown_tagged_fields,
        })
    }
}

impl Default for Voter {
    fn default() -> Self {
        Self {
            voter_id: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for Voter {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct LeaderChangeMessage {
    /// The version of the leader change message
    /// 
    /// Supported API versions: 0
    pub version: i16,

    /// The ID of the newly elected leader
    /// 
    /// Supported API versions: 0
    pub leader_id: super::BrokerId,

    /// The set of voters in the quorum for this epoch
    /// 
    /// Supported API versions: 0
    pub voters: Vec<Voter>,

    /// The voters who voted for the leader at the time of election
    /// 
    /// Supported API versions: 0
    pub granting_voters: Vec<Voter>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for LeaderChangeMessage {
    type Builder = LeaderChangeMessageBuilder;

    fn builder() -> Self::Builder{
        LeaderChangeMessageBuilder::default()
    }
}

impl Encodable for LeaderChangeMessage {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.version)?;
        types::Int32.encode(buf, &self.leader_id)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.voters)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.granting_voters)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.version)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.voters)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.granting_voters)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for LeaderChangeMessage {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let version = types::Int16.decode(buf)?;
        let leader_id = types::Int32.decode(buf)?;
        let voters = types::CompactArray(types::Struct { version }).decode(buf)?;
        let granting_voters = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            version,
            leader_id,
            voters,
            granting_voters,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderChangeMessage {
    fn default() -> Self {
        Self {
            version: 0,
            leader_id: (0).into(),
            voters: Default::default(),
            granting_voters: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderChangeMessage {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

