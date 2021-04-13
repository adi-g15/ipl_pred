use std::fs;
use json_minimal::Json;

pub type JsonType = Json;

pub fn json_from_file(filepath: &str) -> Option<JsonType> {
    let content = fs::read_to_string(filepath).expect("Failed to read the String");

    // initialise and return the json object here
    // match Json::parse(content.as_bytes()){
    //     Ok(parsed) => parsed,
    //     Err(err) => {
    //         println!("Error -> {1} {0}", err.0, err.1);
    //         Json::new()
    //     }
    // };
    Some(Json::parse(content.as_bytes()).expect("Failed to parse JSON content"))
}
