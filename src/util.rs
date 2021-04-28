use crate::decl::{ JsonType, IplLeagueMatch };
use std::fs;
use std::fmt;

#[allow(dead_code)]
pub fn json_from_file(filepath: &str) -> JsonType {
    let content = fs::read_to_string(filepath)
                      .expect("Failed to read the String");

    JsonType::parse(content.as_bytes())
            .expect("Failed to parse JSON content")
}

#[allow(dead_code)]
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

pub fn print_all_matches(matches: &[IplLeagueMatch]) {
    for i in 0..matches.len() {
        println!("{} - {:?} vs {:?}", i, matches[i].team1, matches[i].team2);
    }
}
