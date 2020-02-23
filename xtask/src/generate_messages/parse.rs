use std::path::Path;
use std::fs;

use failure::Error;
use json_comments::StripComments;
use assert_json_diff::assert_json_eq;

use super::spec::Spec;

pub fn parse(path: &Path) -> Result<Spec, Error> {
    let buf = fs::read(path)?;
    let stripped = StripComments::new(buf.as_slice());
    let original_json: serde_json::Value = serde_json::from_reader(stripped)?;
    let spec: Spec = serde_json::from_value(original_json.clone())?;
    let parsed_json = serde_json::to_value(&spec)?;

    assert_json_eq!(original_json, parsed_json);

    Ok(spec)
}
