use std::fs;
use std::path::Path;

use failure::Error;
use json_comments::StripComments;
use serde_json::Value;

use super::spec::Spec;

pub fn parse(path: &Path) -> Result<Spec, Error> {
    let buf = fs::read(path)?;
    let stripped = StripComments::new(buf.as_slice());
    let mut original_json: Value = serde_json::from_reader(stripped)?;

    // the tag field is sometimes encoded as a string instead of a number
    // this workaround allows comparison and simple parsing
    convert_tag_to_number(&mut original_json).expect("could not convert tag fields");

    let spec: Spec = serde_json::from_value(original_json.clone())?;
    let parsed_json = serde_json::to_value(&spec)?;

    // will only panic if the expected json has unknown fields
    assert_json_diff::assert_json_include!(
        actual: parsed_json,
        expected: original_json
    );

    Ok(spec)
}

fn convert_tag_to_number(object: &mut Value) -> Option<()> {
    if let Some(fields) = object.as_object_mut()?.get_mut("fields") {
        for field in fields.as_array_mut()? {
            if let Some(tag) = field.as_object_mut()?.get_mut("tag") {
                if tag.is_string() {
                    let new_value = tag.as_str()?.parse::<i32>().ok()?;
                    *tag = Value::Number(new_value.into());
                }
            }
            convert_tag_to_number(field)?;
        }
    }
    Some(())
}
