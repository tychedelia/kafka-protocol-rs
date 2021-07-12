//! JoinGroupRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/JoinGroupRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct JoinGroupRequestProtocol {
    /// The protocol metadata.
    /// 
    /// Supported API versions: 0-7
    pub metadata: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for JoinGroupRequestProtocol {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 6 {
            types::CompactBytes.encode(buf, &self.metadata)?;
        } else {
            types::Bytes.encode(buf, &self.metadata)?;
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 6 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 6 {
            total_size += types::CompactBytes.compute_size(&self.metadata)?;
        } else {
            total_size += types::Bytes.compute_size(&self.metadata)?;
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl MapDecodable for JoinGroupRequestProtocol {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let metadata = if version >= 6 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            metadata,
            unknown_tagged_fields,
        }))
    }
}

impl Default for JoinGroupRequestProtocol {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for JoinGroupRequestProtocol {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct JoinGroupRequest {
    /// The group identifier.
    /// 
    /// Supported API versions: 0-7
    pub group_id: super::GroupId,

    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
    /// 
    /// Supported API versions: 0-7
    pub session_timeout_ms: i32,

    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
    /// 
    /// Supported API versions: 1-7
    pub rebalance_timeout_ms: i32,

    /// The member id assigned by the group coordinator.
    /// 
    /// Supported API versions: 0-7
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 5-7
    pub group_instance_id: Option<StrBytes>,

    /// The unique name the for class of protocols implemented by the group we want to join.
    /// 
    /// Supported API versions: 0-7
    pub protocol_type: StrBytes,

    /// The list of protocols that the member supports.
    /// 
    /// Supported API versions: 0-7
    pub protocols: indexmap::IndexMap<StrBytes, JoinGroupRequestProtocol>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for JoinGroupRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        types::Int32.encode(buf, &self.session_timeout_ms)?;
        if version >= 1 {
            types::Int32.encode(buf, &self.rebalance_timeout_ms)?;
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 5 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.protocol_type)?;
        } else {
            types::String.encode(buf, &self.protocol_type)?;
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.protocols)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.protocols)?;
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        total_size += types::Int32.compute_size(&self.session_timeout_ms)?;
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.rebalance_timeout_ms)?;
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 5 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.protocol_type)?;
        } else {
            total_size += types::String.compute_size(&self.protocol_type)?;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.protocols)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.protocols)?;
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for JoinGroupRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let session_timeout_ms = types::Int32.decode(buf)?;
        let rebalance_timeout_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let member_id = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_instance_id = if version >= 5 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let protocol_type = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let protocols = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            group_id,
            session_timeout_ms,
            rebalance_timeout_ms,
            member_id,
            group_instance_id,
            protocol_type,
            protocols,
            unknown_tagged_fields,
        })
    }
}

impl Default for JoinGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            session_timeout_ms: 0,
            rebalance_timeout_ms: -1,
            member_id: Default::default(),
            group_instance_id: None,
            protocol_type: Default::default(),
            protocols: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for JoinGroupRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

impl HeaderVersion for JoinGroupRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            2
        } else {
            1
        }
    }
}

