use std::fs;
use json_minimal::Json;
use crate::decl::JsonType;
use std::fmt;

pub fn json_from_file(filepath: &str) -> JsonType {
    let content = fs::read_to_string(filepath)
                      .expect("Failed to read the String");

    Json::parse(content.as_bytes())
            .expect("Failed to parse JSON content")
}

pub fn get_enum_name<T: fmt::Debug>(val: T) -> String{
    format!("{:?}", val)
}
