use std::fs;
use json_minimal::Json;
use crate::decl::JsonType;
use std::fmt;

#[allow(dead_code)]
pub fn json_from_file(filepath: &str) -> JsonType {
    let content = fs::read_to_string(filepath)
                      .expect("Failed to read the String");

    Json::parse(content.as_bytes())
            .expect("Failed to parse JSON content")
}

pub fn write_json_to_file(json_obj: &JsonType, file_path: &str) -> Result<(),String> {
    if ! std::path::Path::new(file_path).exists() {
        return Err(format!("{} doesn't exist !", file_path));
    }

    match fs::write(file_path, json_obj.print()) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

pub fn get_enum_name<T: fmt::Debug>(val: T) -> String{
    format!("{:?}", val)
}
