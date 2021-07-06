use std::fs::{self, File};
use std::io::Write;
use std::collections::{BTreeSet, BTreeMap};

use failure::Error;

mod code_writer;
pub mod expr;
mod spec;
mod parse;
mod generate;

use spec::SpecType;

pub fn run() -> Result<(), Error> {
    const INPUT_PATH: &str = "messages";
    const OUTPUT_PATH: &str = "src/network/messages";

    // Clear output directory
    for file in fs::read_dir(OUTPUT_PATH)? {
        let file = file?;
        if file.file_type()?.is_file() {
            let path = file.path();
            if path.extension() == Some("rs".as_ref()) {
                fs::remove_file(path)?;
            }
        }
    }

    // Find input files
    let mut input_file_paths = Vec::new();
    for file in fs::read_dir(INPUT_PATH)? {
        let file = file?;
        if file.file_type()?.is_file() {
            let path = file.path();
            if path.extension() == Some("json".as_ref()) {
                input_file_paths.push(path);
            }
        }
    }
    input_file_paths.sort();
    let mut entity_types = BTreeSet::new();
    let mut request_types = BTreeMap::new();
    let mut response_types = BTreeMap::new();

    let module_path = format!("{}.rs", OUTPUT_PATH);
    let mut module_file = File::create(&module_path)?;

    writeln!(module_file, "//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.")?;
    writeln!(module_file)?;
    writeln!(module_file, "use franz_protocol::{{NewType, Request, StrBytes}};")?;
    writeln!(module_file)?;

    for input_file_path in &input_file_paths {

        let spec = parse::parse(&input_file_path)?;
        let spec_meta = (spec.type_, spec.api_key);

        let (module_name, struct_name) = generate::generate(OUTPUT_PATH, spec, &mut entity_types)?;

        match spec_meta {
            (SpecType::Request, Some(k)) => {
                request_types.insert(k, struct_name.clone());
            },
            (SpecType::Response, Some(k)) => {
                response_types.insert(k, struct_name.clone());
            },
            _ => {},
        }

        writeln!(module_file, "pub mod {};", module_name)?;
        writeln!(module_file, "pub use {}::{};", module_name, struct_name)?;
        writeln!(module_file)?;
    }

    for (api_key, request_type) in request_types {
        let response_type = response_types.remove(&api_key).expect("Every request type has a response type");
        writeln!(module_file, "impl Request for {} {{", request_type)?;
        writeln!(module_file, "    const KEY: i16 = {};", api_key)?;
        writeln!(module_file, "    type Response = {};", response_type)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file)?;
    }

    for entity_type in entity_types {
        let mut derives = vec!["Debug", "Clone", "Eq", "PartialEq", "Ord", "PartialOrd", "Hash", "Default"];
        if entity_type.inner.is_copy() {
            derives.push("Copy");
        }

        let rust_name = entity_type.inner.rust_name();

        writeln!(module_file, "#[derive({})]", derives.join(", "))?;
        writeln!(module_file, "pub struct {}(pub {});\n", entity_type.name, rust_name)?;
        writeln!(module_file, "impl From<{}> for {} {{", rust_name, entity_type.name)?;
        writeln!(module_file, "    fn from(other: {}) -> Self {{ Self(other) }}", rust_name)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl From<{}> for {} {{", entity_type.name, rust_name)?;
        writeln!(module_file, "    fn from(other: {}) -> Self {{ other.0 }}", entity_type.name)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl std::borrow::Borrow<{}> for {} {{", rust_name, entity_type.name)?;
        writeln!(module_file, "    fn borrow(&self) -> &{} {{ &self.0 }}", rust_name)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl std::ops::Deref for {} {{", entity_type.name)?;
        writeln!(module_file, "    type Target = {};", rust_name)?;
        writeln!(module_file, "    fn deref(&self) -> &Self::Target {{ &self.0 }}")?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl std::cmp::PartialEq<{}> for {} {{", rust_name, entity_type.name)?;
        writeln!(module_file, "    fn eq(&self, other: &{}) -> bool {{ &self.0 == other }}", rust_name)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl std::cmp::PartialEq<{}> for {} {{", entity_type.name, rust_name)?;
        writeln!(module_file, "    fn eq(&self, other: &{}) -> bool {{ self == &other.0 }}", entity_type.name)?;
        writeln!(module_file, "}}")?;
        writeln!(module_file, "impl NewType<{}> for {} {{}}", rust_name, entity_type.name)?;
        writeln!(module_file)?;
    }

    Ok(())
}
