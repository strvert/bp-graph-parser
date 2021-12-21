pub mod parser;

use anyhow::{anyhow, Result};
use parser::ast::Objects;

/// Parses the serialized text representing the Object and stores the information in the Object
/// structure.
pub fn read_object(objects_code: &str) -> Result<Objects> {
    match parser::object::objects(objects_code) {
        Ok(obj) => {
            if obj.0.len() != 0 {
                Err(anyhow!("Text is left after parsing is complete"))
            } else {
                Ok(obj.1)
            }
        }
        Err(err) => Err(anyhow!("parse error: {}", err)),
    }
}

/// Parses the serialized text representing the Object and returns it as JSON.
pub fn to_json(objects_code: &str, pretty: bool) -> Result<String> {
    let obj = read_object(objects_code)?;
    if pretty {
        Ok(serde_json::to_string_pretty(&obj)?)
    } else {
        Ok(serde_json::to_string(&obj)?)
    }
}
