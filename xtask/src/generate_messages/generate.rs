use std::io::{Write, BufWriter};
use std::fs::File;
use std::collections::{BTreeSet, BTreeMap};
use std::fmt::Display;

use failure::Error;
use inflector::Inflector;

use super::spec::{Spec, SpecType, TypeSpec, PrimitiveType, FieldSpec, VersionSpec};
use super::code_writer::CodeWriter;
use super::expr::{Expr, CmpType};

#[derive(Debug, Clone, PartialEq, Eq)]
struct WrittenStruct {
    name: String,
    map_key: Option<Box<PreparedType>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EntityType {
    pub name: String,
    pub inner: PrimitiveType,
}

fn primitive_default(prim: PrimitiveType) -> PreparedDefault {
    use PrimitiveType::*;
    match prim {
        Bool => PreparedDefault::Boolean(false),
        Int8 | Int16 | Int32 | Int64 => PreparedDefault::Numeric("0".into()),
        Float64 => PreparedDefault::Numeric("0.0".into()),
        String | Bytes => PreparedDefault::Empty,
    }
}

fn parse_primitive_default(prim: PrimitiveType, default_str: String, type_: &PreparedType) -> PreparedDefault {
    use PrimitiveType::*;
    match prim {
        Int8 | Int16 | Int32 | Int64 | Float64 => PreparedDefault::Numeric(default_str),
        String => PreparedDefault::String(default_str),
        _ => panic!("Unexpected default value {:?} for {}", default_str, type_.rust_name()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PreparedType {
    Primitive(PrimitiveType),
    Entity(EntityType),
    Struct(WrittenStruct),
    Array(Box<PreparedType>),
    Map(Box<PreparedType>, String),
}

impl PreparedType {
    fn rust_name(&self) -> String {
        match self {
            Self::Primitive(prim) => prim.rust_name().into(),
            Self::Entity(entity_type) => format!("super::{}", entity_type.name),
            Self::Struct(inner) => inner.name.clone(),
            Self::Array(inner) => format!("Vec<{}>", inner.rust_name()),
            Self::Map(key, value) => format!("indexmap::IndexMap<{}, {}>", key.rust_name(), value),
        }
    }
    fn name(&self, flexible: bool) -> String {
        match self {
            Self::Primitive(prim) => prim.name(flexible).into(),
            Self::Entity(entity_type) => entity_type.inner.name(flexible).into(),
            Self::Struct(_) => "types::Struct { version }".into(),
            Self::Array(inner) => if flexible {
                format!("types::CompactArray({})", inner.name(flexible))
            } else {
                format!("types::Array({})", inner.name(flexible))
            },
            Self::Map(_, _) => if flexible {
                "types::CompactArray(types::Struct { version })".into()
            } else {
                "types::Array(types::Struct { version })".into()
            },
        }
    }
    pub fn has_compact_form(&self) -> bool {
        match self {
            Self::Primitive(prim) => prim.has_compact_form(),
            Self::Entity(entity_type) => entity_type.inner.has_compact_form(),
            Self::Struct(_) => false,
            Self::Array(_) => true,
            Self::Map(_, _) => true,
        }
    }
    fn default(&self) -> PreparedDefault {
        match self {
            Self::Primitive(prim) => primitive_default(*prim),
            Self::Entity(entity_type) => primitive_default(entity_type.inner),
            Self::Struct(_) => panic!("Structs do not have a default value"),
            Self::Array(_) | Self::Map(_, _) => PreparedDefault::Empty,
        }
    }
    fn is_entity(&self) ->bool {
        if let Self::Entity(_) = self {
            true
        } else {
            false
        }
    }
}

enum PreparedDefault {
    Null,
    Empty,
    Boolean(bool),
    Numeric(String),
    String(String),
}

impl PreparedDefault {
    fn gen_is_default(&self, expr: &Expr, optional: bool) -> Expr {
        if optional {
            if let Self::Null = self {
                expr.method("is_none", "")
            } else {
                expr
                    .method("as_ref", "")
                    .method("map", format!("|x| {}", self.gen_is_default(&Expr::new_atom("x"), false)))
                    .method("unwrap_or_default", "")
            }
        } else {
            match self {
                Self::Null => unreachable!(),
                Self::Empty => expr.method("is_empty", ""),
                Self::Boolean(true) => expr.deref(),
                Self::Boolean(false) => expr.deref().not(),
                Self::Numeric(v) => expr.deref().compare(CmpType::Eq, &Expr::new_atom(v)),
                Self::String(s) => expr.compare(CmpType::Eq, &Expr::new_str(s)),
            }
        }
    }
    fn gen_default(&self, optional: bool, is_entity: bool) -> Expr {
        if optional {
            if let Self::Null = self {
                Expr::new_atom("None".into())
            } else {
                Expr::new_atom(&format!("Some({})", self.gen_default(false, is_entity)))
            }
        } else if is_entity {
            if let Self::Empty = self {
                self.gen_default(false, false)
            } else {
                self.gen_default(false, false).method("into", "")
            }
        } else {
            match self {
                Self::Null => unreachable!(),
                Self::Empty => Expr::new_atom("Default::default()"),
                Self::Boolean(true) => Expr::new_atom("true"),
                Self::Boolean(false) => Expr::new_atom("false"),
                Self::Numeric(v) => Expr::new_unary(v),
                Self::String(s) => Expr::new_atom(&format!("StrBytes::from_str({:?})", s)),
            }
        }
    }
}

struct PreparedField {
    name: String,
    optional: bool,
    type_: PreparedType,
    versions: VersionSpec,
    tag: Option<i32>,
    tagged_versions: VersionSpec,
    _nullable_versions: VersionSpec,
    default: PreparedDefault,
    ignorable: bool,
    _entity_type: Option<String>,
    map_key: bool,
    about: String,
    flexible_versions: VersionSpec,
}

fn prepare_field_type<W: Write>(
    w: &mut CodeWriter<W>,
    type_: &TypeSpec,
    field: &FieldSpec,
    entity_types: &mut BTreeSet<EntityType>,
    valid_versions: VersionSpec,
    flexible_msg_versions: VersionSpec,
) -> Result<PreparedType, Error> {
    Ok(match type_ {
        TypeSpec::Primitive(prim) => {
            if let Some(entity_type) = &field.entity_type {
                let entity_type = EntityType {
                    inner: *prim,
                    name: entity_type.to_pascal_case(),
                };
                entity_types.insert(entity_type.clone());
                PreparedType::Entity(entity_type)
            } else {
                PreparedType::Primitive(*prim)
            }
        },
        TypeSpec::Struct(name) => {
            if let Some(fields) = &field.fields {
                let written_struct = write_struct_def(w, name, fields, entity_types, valid_versions, flexible_msg_versions)?;
                PreparedType::Struct(written_struct)
            } else {
                PreparedType::Struct(WrittenStruct {
                    name: name.clone(),
                    map_key: None,
                })
            }
        },
        TypeSpec::Array(elem) => {
            let prepared_elem = prepare_field_type(w, elem, field, entity_types, valid_versions, flexible_msg_versions)?;
            match prepared_elem {
                PreparedType::Struct(WrittenStruct { name, map_key: Some(map_key) }) => PreparedType::Map(map_key, name),
                other => PreparedType::Array(Box::new(other)),
            }
        }
    })
}

fn write_version_cond<W: Write, FT: FnOnce(&mut CodeWriter<W>) -> Result<(), Error>, FF: FnOnce(&mut CodeWriter<W>) -> Result<(), Error>>(
    w: &mut CodeWriter<W>,
    valid_versions: VersionSpec,
    condition: VersionSpec,
    if_true: FT,
    if_false: FF,
    skip_true: bool,
    skip_false: bool,
) -> Result<(), Error> {
    let (min, max) = match valid_versions {
        VersionSpec::None => return Ok(()),
        VersionSpec::Exact(version) => (version, version),
        VersionSpec::Since(_) => panic!("Valid version range should be bounded"),
        VersionSpec::Range(a, b) => (a, b),
    };

    let condition = condition.intersect(valid_versions);
    let always_true = condition == valid_versions;
    if skip_true {
        if !always_true {
            match condition {
                VersionSpec::None => return if_false(w),
                VersionSpec::Exact(version) => write!(w, "if version != {} ", version)?,
                VersionSpec::Since(version) => write!(w, "if version < {} ", version)?,
                VersionSpec::Range(a, b) if a == min => write!(w, "if version > {} ", b)?,
                VersionSpec::Range(a, b) if b == max => write!(w, "if version < {} ", a)?,
                VersionSpec::Range(a, b) => write!(w, "if version < {} || version > {} ", a, b)?,
            }
            w.block(if_false)?;
        }
    } else {
        if always_true {
            if_true(w)?;
        } else {
            match condition {
                VersionSpec::None => return if_false(w),
                VersionSpec::Exact(version) => write!(w, "if version == {} ", version)?,
                VersionSpec::Since(version) => write!(w, "if version >= {} ", version)?,
                VersionSpec::Range(a, b) if a == min => write!(w, "if version <= {} ", b)?,
                VersionSpec::Range(a, b) if b == max => write!(w, "if version >= {} ", a)?,
                VersionSpec::Range(a, b) => write!(w, "if version >= {} && version <= {} ", a, b)?,
            }
            w.block(if_true)?;
            if !skip_false {
                write!(w, " else ")?;
                w.block(if_false)?;
            }
        }
    }
    Ok(())
}

fn write_encode_or_compute<W: Write, T: Display>(
    w: &mut CodeWriter<W>,
    type_: &str,
    var_name: T,
    compute_size: bool,
) -> Result<(), Error> {
    if compute_size {
        write!(w, "total_size += {}.compute_size({})?;", type_, var_name)?;
    } else {
        write!(w, "{}.encode(buf, {})?;", type_, var_name)?;
    }
    Ok(())
}

fn write_encode_field<W: Write>(
    w: &mut CodeWriter<W>,
    field: &PreparedField,
    valid_versions: VersionSpec,
    compute_size: bool,
) -> Result<(), Error> {
    let var_name = if field.map_key {
        Expr::new_atom("key")
    } else {
        Expr::new_atom("self").field(&field.name).by_ref()
    };

    if field.tagged_versions.contains(field.versions.intersect(valid_versions)) {
        return Ok(());
    }

    write_version_cond(w, valid_versions, field.tagged_versions, |_| Ok(()), |w| {
        write_version_cond(w, valid_versions, field.versions, |w| {
            let valid_versions = valid_versions.intersect(field.versions);
            if !field.type_.has_compact_form() {
                write_encode_or_compute(w, &field.type_.name(false), &var_name, compute_size)?;
            } else {
                write_version_cond(w, valid_versions, field.flexible_versions, |w| {
                    write_encode_or_compute(w, &field.type_.name(true), &var_name, compute_size)?;
                    Ok(())
                }, |w| {
                    write_encode_or_compute(w, &field.type_.name(false), &var_name, compute_size)?;
                    Ok(())
                }, false, false)?;
            }
            Ok(())
        }, |w| {
            write!(w, "if {} ", field.default.gen_is_default(&var_name, field.optional).not())?;
            w.block(|w| {
                write!(w, "return Err(EncodeError)")?;
                Ok(())
            })?;
            Ok(())
        }, false, field.ignorable)?;
        Ok(())
    }, true, false)?;
    writeln!(w)?;
    Ok(())
}

fn write_size_check<W: Write, T: Display>(
    w: &mut CodeWriter<W>,
    expr: T,
    t: &str,
    msg: &str,
) -> Result<(), Error> {
    writeln!(w, "if {} > std::{}::MAX as usize {{", expr, t)?;
    writeln!(w, "    error!({:?}, {});", msg, expr)?;
    writeln!(w, "    return Err(EncodeError);")?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_encode_tag_buffer<W: Write>(
    w: &mut CodeWriter<W>,
    prepared_fields: &[PreparedField],
    valid_versions: VersionSpec,
    flexible_msg_versions: VersionSpec,
    compute_size: bool,
) -> Result<(), Error> {
    write_version_cond(w, valid_versions, flexible_msg_versions, |w| {
        let valid_versions = valid_versions.intersect(flexible_msg_versions);

        let sorted_tagged_fields: BTreeMap<i32, &PreparedField> = prepared_fields
            .iter()
            .filter_map(|field| Some((field.tag?, field)))
            .collect();
        
        if sorted_tagged_fields.is_empty() {
            writeln!(w, "let num_tagged_fields = self.unknown_tagged_fields.len();")?;
        } else {
            writeln!(w, "let mut num_tagged_fields = self.unknown_tagged_fields.len();")?;
        }

        // Count number of tagged fields
        for field in sorted_tagged_fields.values() {
            let var_name = if field.map_key {
                Expr::new_atom("key")
            } else {
                Expr::new_atom("self").field(&field.name).by_ref()
            };
            write_version_cond(w, valid_versions, field.tagged_versions, |w| {
                let valid_versions = valid_versions.intersect(field.tagged_versions);
                write_version_cond(w, valid_versions, field.versions, |w| {
                    write!(w, "if {} ", field.default.gen_is_default(&var_name, field.optional).not())?;
                    w.block(|w| {
                        write!(w, "num_tagged_fields += 1;")?;
                        Ok(())
                    })?;
                    Ok(())
                }, |_| Ok(()), false, true)?;
                writeln!(w)?;
                Ok(())
            }, |_| Ok(()), false, true)?;
        }

        write_size_check(w, "num_tagged_fields", "u32", "Too many tagged fields to encode ({} fields)")?;
        write_encode_or_compute(w, "types::UnsignedVarInt", "num_tagged_fields as u32", compute_size)?;
        writeln!(w)?;

        // Write out tagged fields
        let mut current_tag = -1;
        for (&k, field) in &sorted_tagged_fields {
            current_tag += 1;
            if k != current_tag {
                if !compute_size {
                    writeln!(w, "write_unknown_tagged_fields(buf, {}..{}, &self.unknown_tagged_fields)?;", current_tag, k)?;
                }
                current_tag = k;
            }

            let var_name = if field.map_key {
                Expr::new_atom("key")
            } else {
                Expr::new_atom("self").field(&field.name).by_ref()
            };
            write_version_cond(w, valid_versions, field.tagged_versions, |w| {
                let valid_versions = valid_versions.intersect(field.tagged_versions);
                write_version_cond(w, valid_versions, field.versions, |w| {
                    let valid_versions = valid_versions.intersect(field.versions);
                    write!(w, "if {} ", field.default.gen_is_default(&var_name, field.optional).not())?;
                    w.block(|w| {
                        write!(w, "let computed_size = ")?;
                        if !field.type_.has_compact_form() {
                            write!(w, "{}.compute_size({})?", &field.type_.name(false), &var_name)?;
                        } else {
                            write_version_cond(w, valid_versions, field.flexible_versions, |w| {
                                write!(w, "{}.compute_size({})?", &field.type_.name(true), &var_name)?;
                                Ok(())
                            }, |w| {
                                write!(w, "{}.compute_size({})?", &field.type_.name(false), &var_name)?;
                                Ok(())
                            }, false, false)?;
                        }
                        writeln!(w, ";")?;
                        write_size_check(w, "computed_size", "u32", "Tagged field is too large to encode ({} bytes)")?;
                        write_encode_or_compute(w, "types::UnsignedVarInt", k, compute_size)?;
                        writeln!(w)?;
                        write_encode_or_compute(w, "types::UnsignedVarInt", "computed_size as u32", compute_size)?;
                        writeln!(w)?;
                        if compute_size {
                            writeln!(w, "total_size += computed_size;")?;
                        } else {
                            if !field.type_.has_compact_form() {
                                write_encode_or_compute(w, &field.type_.name(false), &var_name, compute_size)?;
                            } else {
                                write_version_cond(w, valid_versions, field.flexible_versions, |w| {
                                    write_encode_or_compute(w, &field.type_.name(true), &var_name, compute_size)?;
                                    Ok(())
                                }, |w| {
                                    write_encode_or_compute(w, &field.type_.name(false), &var_name, compute_size)?;
                                    Ok(())
                                }, false, false)?;
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                }, |w| {
                    write!(w, "if {} ", field.default.gen_is_default(&var_name, field.optional).not())?;
                    w.block(|w| {
                        write!(w, "return Err(EncodeError)")?;
                        Ok(())
                    })?;
                    Ok(())
                }, false, field.ignorable)?;
                writeln!(w)?;
                Ok(())
            }, |_| Ok(()), false, true)?;
        }
        writeln!(w)?;

        if compute_size {
            write!(w, "total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;")?;
        } else {
            write!(w, "write_unknown_tagged_fields(buf, {}.., &self.unknown_tagged_fields)?;", current_tag+1)?;
        }

        Ok(())
    }, |_| Ok(()), false, true)?;
    writeln!(w)?;
    Ok(())
}

fn write_decode_field<W: Write>(
    w: &mut CodeWriter<W>,
    field: &PreparedField,
    valid_versions: VersionSpec,
) -> Result<(), Error> {
    let var_name = if field.map_key {
        "key_field"
    } else {
        &field.name
    };

    if field.tagged_versions.is_none() {
        write!(w, "let {} = ", var_name)?;
    } else {
        write!(w, "let mut {} = ", var_name)?;
    }

    if field.tagged_versions.contains(field.versions.intersect(valid_versions)) {
        writeln!(w, "{};", field.default.gen_default(field.optional, field.type_.is_entity()))?;
        return Ok(());
    }

    write_version_cond(w, valid_versions, field.tagged_versions, |w| {
        write!(w, "{}", field.default.gen_default(field.optional, field.type_.is_entity()))?;
        Ok(())
    }, |w| {
        write_version_cond(w, valid_versions, field.versions, |w| {
            let valid_versions = valid_versions.intersect(field.versions);
            if !field.type_.has_compact_form() {
                write!(w, "{}.decode(buf)?", field.type_.name(false))?;
            } else {

                write_version_cond(w, valid_versions, field.flexible_versions, |w| {
                    write!(w, "{}.decode(buf)?", field.type_.name(true))?;
                    Ok(())
                }, |w| {
                    write!(w, "{}.decode(buf)?", field.type_.name(false))?;
                    Ok(())
                }, false, false)?;
            }
            Ok(())
        }, |w| {
            write!(w, "{}", field.default.gen_default(field.optional, field.type_.is_entity()))?;
            Ok(())
        }, false, false)?;
        Ok(())
    }, false, false)?;
    writeln!(w, ";")?;

    Ok(())
}

fn write_decode_tag_buffer<W: Write>(
    w: &mut CodeWriter<W>,
    prepared_fields: &[PreparedField],
    valid_versions: VersionSpec,
    flexible_msg_versions: VersionSpec,
) -> Result<(), Error> {
    write_version_cond(w, valid_versions, flexible_msg_versions, |w| {
        let valid_versions = valid_versions.intersect(flexible_msg_versions);

        let sorted_tagged_fields: BTreeMap<i32, &PreparedField> = prepared_fields
            .iter()
            .filter_map(|field| Some((field.tag?, field)))
            .collect();
        
        writeln!(w, "let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;")?;
        write!(w, "for _ in 0..num_tagged_fields ")?;
        w.block(|w| {
            writeln!(w, "let tag: u32 = types::UnsignedVarInt.decode(buf)?;")?;
            writeln!(w, "let size: u32 = types::UnsignedVarInt.decode(buf)?;")?;

            if sorted_tagged_fields.is_empty() {
                writeln!(w, "let mut unknown_value = vec![0; size as usize];")?;
                writeln!(w, "buf.try_copy_to_slice(&mut unknown_value)?;")?;
                write!(w, "unknown_tagged_fields.insert(tag as i32, unknown_value);")?;
                Ok(())
            } else {
                write!(w, "match tag ")?;
                w.block(|w| {
                    for (&k, field) in &sorted_tagged_fields {
                        let var_name = if field.map_key {
                            "key_field"
                        } else {
                            &field.name
                        };
                        write!(w, "{} => ", k)?;
                        w.block(|w| {
                            let tagged_field_versions = field.tagged_versions.intersect(field.versions);
                            write_version_cond(w, valid_versions, tagged_field_versions, |w| {
                                let valid_versions = valid_versions.intersect(tagged_field_versions);
                                if !field.type_.has_compact_form() {
                                    write!(w, "{} = {}.decode(buf)?;", var_name, field.type_.name(false))?;
                                } else {
    
                                    write_version_cond(w, valid_versions, field.flexible_versions, |w| {
                                        write!(w, "{} = {}.decode(buf)?;", var_name, field.type_.name(true))?;
                                        Ok(())
                                    }, |w| {
                                        write!(w, "{} = {}.decode(buf)?;", var_name, field.type_.name(false))?;
                                        Ok(())
                                    }, false, false)?;
                                }
                                Ok(())
                            }, |w| {
                                writeln!(w, "error!({:?}, tag, version);", "Tag {} is not valid for version {}")?;
                                write!(w, "return Err(DecodeError);")?;
                                Ok(())
                            }, false, false)?;
                            Ok(())
                        })?;
                        writeln!(w, ",")?;
                    }
                    write!(w, "_ => ")?;
                    w.block(|w| {
                        writeln!(w, "let mut unknown_value = vec![0; size as usize];")?;
                        writeln!(w, "buf.try_copy_to_slice(&mut unknown_value)?;")?;
                        write!(w, "unknown_tagged_fields.insert(tag as i32, unknown_value);")?;
                        Ok(())
                    })
                })
            }
        })
    }, |_| Ok(()), false, true)?;
    writeln!(w)?;
    Ok(())
}

fn write_struct_def<W: Write>(
    w: &mut CodeWriter<W>,
    name: &str,
    fields: &[FieldSpec],
    entity_types: &mut BTreeSet<EntityType>,
    valid_versions: VersionSpec,
    flexible_msg_versions: VersionSpec,
) -> Result<WrittenStruct, Error> {
    let mut prepared_fields = Vec::new();
    let mut map_key = None;

    let num_map_keys = fields.iter().filter(|field| field.map_key).count();

    for field in fields {
        let type_ = prepare_field_type(w, &field.type_, field, entity_types, valid_versions, flexible_msg_versions)?;

        if field.map_key && num_map_keys == 1 {
            map_key = Some(Box::new(type_.clone()))
        }

        let name = field.name.to_snake_case();
        let optional = !field.nullable_versions.is_none();
        let versions = field.versions.intersect(valid_versions);
        let flexible_versions = field.flexible_versions.unwrap_or(flexible_msg_versions);
        let tagged_versions = field.tagged_versions;
        let nullable_versions = field.nullable_versions;

        assert!(flexible_msg_versions.contains(flexible_versions));
        assert!(flexible_versions.contains(tagged_versions));

        let default = if let Some(default) = &field.default {
            let default_str = match default {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };

            match default_str.as_str() {
                "null" => {
                    assert!(nullable_versions.contains(field.versions));
                    PreparedDefault::Null
                },
                "true" | "false" => {
                    assert!(type_ == PreparedType::Primitive(PrimitiveType::Bool));
                    PreparedDefault::Boolean(default_str == "true")
                },
                _ => match &type_ {
                    PreparedType::Primitive(prim) => parse_primitive_default(*prim, default_str, &type_),
                    PreparedType::Entity(entity_type) => parse_primitive_default(entity_type.inner, default_str, &type_),
                    _ => panic!("Unexpected default value {:?} for {}", default_str, type_.rust_name()),
                }
            }
        } else {
            type_.default()
        };

        prepared_fields.push(PreparedField {
            name, optional, type_,
            versions,
            tag: field.tag,
            tagged_versions,
            _nullable_versions: field.nullable_versions,
            default,
            ignorable: field.ignorable.unwrap_or(false),
            _entity_type: field.entity_type.clone(),
            map_key: field.map_key && num_map_keys == 1,
            about: field.about.clone(),
            flexible_versions,
        });
    }

    writeln!(w, "/// Valid versions: {}", valid_versions)?;
    writeln!(w, "#[derive(Debug, Clone)]")?;
    write!(w, "pub struct {} ", name)?;
    w.block(|w| {
        for prepared_field in &prepared_fields {
            if prepared_field.map_key { continue; }
            writeln!(w, "/// {}", prepared_field.about)?;
            writeln!(w, "/// ")?;
            writeln!(w, "/// Supported API versions: {}", prepared_field.versions)?;
            if prepared_field.optional {
                writeln!(w, "pub {}: Option<{}>,", prepared_field.name, prepared_field.type_.rust_name())?;
            } else {
                writeln!(w, "pub {}: {},", prepared_field.name, prepared_field.type_.rust_name())?;
            }
            writeln!(w)?;
        }

        if !flexible_msg_versions.is_none() {
            writeln!(w, "/// Other tagged fields")?;
            writeln!(w, "pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,")?;
        }

        Ok(())
    })?;
    writeln!(w)?;
    writeln!(w)?;

    if map_key.is_some() {
        write!(w, "impl MapEncodable for {} ", name)?;
    } else {
        write!(w, "impl Encodable for {} ", name)?;
    }
    w.block(|w| {
        if let Some(key) = &map_key {
            writeln!(w, "type Key = {};", key.rust_name())?;
            write!(w, "fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> ")?;
        } else {
            write!(w, "fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> ")?;
        }
        w.block(|w| {
            for prepared_field in &prepared_fields {
                write_encode_field(w, prepared_field, valid_versions, false)?;
            }
            write_encode_tag_buffer(w, &prepared_fields, valid_versions, flexible_msg_versions, false)?;
            write!(w, "Ok(())")?;
            Ok(())
        })?;
        writeln!(w)?;
        if map_key.is_some() {
            write!(w, "fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> ")?;
        } else {
            write!(w, "fn compute_size(&self, version: i16) -> Result<usize, EncodeError> ")?;
        }
        w.block(|w| {
            writeln!(w, "let mut total_size = 0;")?;
            for prepared_field in &prepared_fields {
                write_encode_field(w, prepared_field, valid_versions, true)?;
            }
            write_encode_tag_buffer(w, &prepared_fields, valid_versions, flexible_msg_versions, true)?;
            write!(w, "Ok(total_size)")?;
            Ok(())
        })?;
        Ok(())
    })?;
    writeln!(w)?;
    writeln!(w)?;

    if map_key.is_some() {
        write!(w, "impl MapDecodable for {} ", name)?;
    } else {
        write!(w, "impl Decodable for {} ", name)?;
    }
    w.block(|w| {
        if let Some(key) = &map_key {
            writeln!(w, "type Key = {};", key.rust_name())?;
            write!(w, "fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> ")?;
        } else {
            write!(w, "fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> ")?;
        }
        w.block(|w| {
            for prepared_field in &prepared_fields {
                write_decode_field(w, prepared_field, valid_versions)?;
            }
            if !flexible_msg_versions.is_none() {
                writeln!(w, "let mut unknown_tagged_fields = BTreeMap::new();")?;
                write_decode_tag_buffer(w, &prepared_fields, valid_versions, flexible_msg_versions)?;
            }
            if map_key.is_some() {
                write!(w, "Ok((key_field, Self ")?;
            } else {
                write!(w, "Ok(Self ")?;
            }
            w.block(|w| {
                for prepared_field in &prepared_fields {
                    if !prepared_field.map_key {
                        writeln!(w, "{},", prepared_field.name)?;
                    }
                }

                if !flexible_msg_versions.is_none() {
                    writeln!(w, "unknown_tagged_fields,")?;
                }

                Ok(())
            })?;
            if map_key.is_some() {
                write!(w, "))")?;
            } else {
                write!(w, ")")?;
            }
            Ok(())
        })
    })?;
    writeln!(w)?;
    writeln!(w)?;

    write!(w, "impl Default for {} ", name)?;
    w.block(|w| {
        write!(w, "fn default() -> Self ")?;
        w.block(|w| {
            write!(w, "Self ")?;
            w.block(|w| {
                for prepared_field in &prepared_fields {
                    if !prepared_field.map_key {
                        writeln!(w, "{}: {},", prepared_field.name, prepared_field.default.gen_default(
                            prepared_field.optional,
                            prepared_field.type_.is_entity()
                        ))?;
                    }
                }

                if !flexible_msg_versions.is_none() {
                    writeln!(w, "unknown_tagged_fields: BTreeMap::new(),")?;
                }

                Ok(())
            })
        })
    })?;
    writeln!(w)?;
    writeln!(w)?;

    write!(w, "impl Message for {} ", name)?;
    w.block(|w| {
        let range = valid_versions.range().expect("Valid versions should be bounded.");
        writeln!(w, "const VERSIONS: VersionRange = VersionRange {{ min: {}, max: {} }};", range.start(), range.end())?;
        Ok(())
    })?;
    writeln!(w)?;
    writeln!(w)?;

    Ok(WrittenStruct {
        name: name.into(),
        map_key,
    })
}

fn write_file_header<W: Write>(
    w: &mut CodeWriter<W>,
) -> Result<(), Error> {
    writeln!(w, "//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.")?;
    writeln!(w, "#![allow(unused)]")?;
    writeln!(w)?;
    writeln!(w, "use std::borrow::Borrow;")?;
    writeln!(w, "use std::collections::BTreeMap;")?;
    writeln!(w)?;
    writeln!(w, "use bytes::Bytes;")?;
    writeln!(w, "use log::error;")?;
    writeln!(w)?;
    writeln!(w, "use franz_protocol::{{")?;
    writeln!(w, "    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,")?;
    writeln!(w, "    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{{ByteBuf, ByteBufMut}}")?;
    writeln!(w, "}};")?;
    writeln!(w)?;
    writeln!(w)?;
    Ok(())
}

pub fn generate(
    output_path: &str,
    spec: Spec,
    entity_types: &mut BTreeSet<EntityType>,
) -> Result<(String, String), Error> {
    let struct_name = spec.name.clone();
    let module_name = struct_name.to_snake_case();

    let file_name = format!("{}/{}.rs", output_path, module_name);
    let mut file = CodeWriter::new(BufWriter::new(File::create(file_name)?));

    let valid_versions = spec.valid_versions;
    let flexible_msg_versions = spec.flexible_versions.unwrap_or_default();

    write_file_header(&mut file)?;
    for common_struct in &spec.common_structs {
        write_struct_def(&mut file, &common_struct.name, &common_struct.fields, entity_types, valid_versions, flexible_msg_versions)?;
    }
    write_struct_def(&mut file, &struct_name, &spec.fields, entity_types, valid_versions, flexible_msg_versions)?;

    if let Some(api_key) = spec.api_key {
        write!(&mut file, "impl HeaderVersion for {} ", struct_name)?;
        file.block(|w| {
            write!(w, "fn header_version(version: i16) -> i16 ")?;
            w.block(|w| {
                match (spec.type_, api_key) {
                    // ApiVersionsResponse always includes a v0 header.
                    // See KIP-511 for details.
                    (SpecType::Response, 18) => {
                        write!(w, "0")?;
                    },
                    (SpecType::Request, _) => {
                        write_version_cond(w, valid_versions, flexible_msg_versions, |w| {
                            write!(w, "2")?;
                            Ok(())
                        }, |w| {
                            // Version 0 of ControlledShutdownRequest has a non-standard request header
                            // which does not include clientId.  Version 1 of ControlledShutdownRequest
                            // and later use the standard request header.
                            if api_key == 7 {
                                write!(w, "if version == 0 {{ 0 }} else {{ 1 }}")?;
                            } else {
                                write!(w, "1")?;
                            }
                            Ok(())
                        }, false, false)?;
                    },
                    (SpecType::Response, _) => {
                        write_version_cond(w, valid_versions, flexible_msg_versions, |w| {
                            write!(w, "1")?;
                            Ok(())
                        }, |w| {
                            write!(w, "0")?;
                            Ok(())
                        }, false, false)?;
                    },
                    _ => unreachable!(),
                }
                Ok(())
            })
        })?;
        writeln!(&mut file)?;
        writeln!(&mut file)?;
    }

    Ok((module_name, struct_name))
}
