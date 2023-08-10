use std::cmp;
use std::fmt::{self, Display};
use std::ops::RangeInclusive;
use std::str::FromStr;

use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use serde_plain::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<i16>,
    #[serde(rename = "type")]
    pub type_: SpecType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerSpec>>,
    pub name: String,
    pub valid_versions: VersionSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_versions: Option<VersionSpec>,
    pub fields: Vec<FieldSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub common_structs: Vec<StructSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: TypeSpec,
    pub versions: VersionSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<i32>,
    #[serde(default, skip_serializing_if = "VersionSpec::is_none")]
    pub tagged_versions: VersionSpec,
    #[serde(default, skip_serializing_if = "VersionSpec::is_none")]
    pub nullable_versions: VersionSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignorable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub map_key: bool,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub about: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldSpec>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_versions: Option<VersionSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_copy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructSpec {
    pub name: String,
    pub versions: VersionSpec,
    pub fields: Vec<FieldSpec>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ListenerSpec {
    ZkBroker,
    Broker,
    Controller,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SpecType {
    Header,
    Request,
    Response,
    Data,
}

#[derive(Debug, Copy, Clone, Display, FromStr, Eq, PartialEq)]
pub enum VersionSpec {
    #[display("none")]
    #[from_str(regex = r"none")]
    None,
    #[display("{0}")]
    #[from_str(regex = r"(?P<0>\d+)")]
    Exact(i16),
    #[display("{0}+")]
    #[from_str(regex = r"(?P<0>\d+)\+")]
    Since(i16),
    #[display("{0}-{1}")]
    #[from_str(regex = r"(?P<0>\d+)-(?P<1>\d+)")]
    Range(i16, i16),
}
derive_serialize_from_display!(VersionSpec);
derive_deserialize_from_str!(VersionSpec, "valid version specification");

impl VersionSpec {
    pub fn intersect(self, other: VersionSpec) -> VersionSpec {
        use VersionSpec::*;
        match (self, other) {
            (Exact(a), Exact(b)) if a == b => Exact(a),
            (Exact(a), Since(b)) | (Since(b), Exact(a)) if a >= b => Exact(a),
            (Exact(v), Range(a, b)) | (Range(a, b), Exact(v)) if v >= a && v <= b => Exact(v),
            (Since(a), Since(b)) => Since(cmp::max(a, b)),
            (Since(v), Range(_, b)) | (Range(_, b), Since(v)) if b == v => Exact(v),
            (Since(v), Range(a, b)) | (Range(a, b), Since(v)) if b > v => Range(cmp::max(a, v), b),
            (Range(_, b), Range(a, _)) if a == b => Exact(b),
            (Range(a, _), Range(_, b)) if a == b => Exact(a),
            (Range(a, b), Range(c, d)) if b > c && d > a => Range(cmp::max(a, c), cmp::min(b, d)),
            _ => None,
        }
    }
    pub fn contains(self, other: VersionSpec) -> bool {
        other.intersect(self) == other
    }
    pub fn range(self) -> Option<RangeInclusive<i16>> {
        match self {
            VersionSpec::None => Some(1..=0),
            VersionSpec::Exact(v) => Some(v..=v),
            VersionSpec::Since(_) => None,
            VersionSpec::Range(a, b) => Some(a..=b),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveType {
    Bool,
    Int8,
    Int16,
    Uint16,
    Int32,
    Int64,
    Float64,
    String,
    Bytes,
    Records,
    Uuid,
}
forward_display_to_serde!(PrimitiveType);
forward_from_str_to_serde!(PrimitiveType);

impl PrimitiveType {
    pub fn rust_name(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::Int8 => "i8",
            Self::Int16 => "i16",
            Self::Uint16 => "u16",
            Self::Int32 => "i32",
            Self::Int64 => "i64",
            Self::Float64 => "f64",
            Self::String => "StrBytes",
            Self::Bytes | Self::Records => "Bytes",
            Self::Uuid => "Uuid",
        }
    }
    pub fn name(&self, flexible: bool) -> &str {
        match self {
            Self::Bool => "types::Boolean",
            Self::Int8 => "types::Int8",
            Self::Int16 => "types::Int16",
            Self::Uint16 => "types::UInt16",
            Self::Int32 => "types::Int32",
            Self::Int64 => "types::Int64",
            Self::Float64 => "types::Float64",
            Self::String => {
                if flexible {
                    "types::CompactString"
                } else {
                    "types::String"
                }
            }
            Self::Bytes | Self::Records => {
                if flexible {
                    "types::CompactBytes"
                } else {
                    "types::Bytes"
                }
            }
            Self::Uuid => "types::Uuid",
        }
    }
    pub fn is_copy(&self) -> bool {
        match self {
            Self::String | Self::Bytes | Self::Records => false,
            _ => true,
        }
    }
    pub fn has_compact_form(&self) -> bool {
        !self.is_copy()
    }
}

#[derive(Debug, Clone)]
pub enum TypeSpec {
    Primitive(PrimitiveType),
    Struct(String),
    Array(Box<TypeSpec>),
}

impl FromStr for TypeSpec {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(if s.starts_with("[]") {
            Self::Array(Box::new(Self::from_str(&s[2..])?))
        } else if let Ok(prim) = PrimitiveType::from_str(s) {
            Self::Primitive(prim)
        } else {
            Self::Struct(s.into())
        })
    }
}

impl Display for TypeSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Primitive(prim) => prim.fmt(f),
            Self::Struct(name) => name.fmt(f),
            Self::Array(inner) => write!(f, "[]{}", inner),
        }
    }
}
derive_serialize_from_display!(TypeSpec);
derive_deserialize_from_str!(TypeSpec, "valid type specification");

impl VersionSpec {
    pub fn is_none(&self) -> bool {
        match self {
            VersionSpec::None => true,
            _ => false,
        }
    }
}

impl Default for VersionSpec {
    fn default() -> Self {
        VersionSpec::None
    }
}
