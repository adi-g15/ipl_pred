mod ipl;
mod decl;
mod algo;
mod util;

use wasm_bindgen::prelude::*;
use decl::{ JsonType };

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_chances(json_string: &str) -> String {
    let json = match JsonType::parse(json_string.as_bytes()) {
        Ok(json_obj) => json_obj,
        Err(err) => {
            panic!("Invalid json: {:?}", err)
        }
    };

    alert("Fine");
    let scores = ipl::chance_calculator(ipl::get_league_matches(&json), false, 20);

    let mut final_json = JsonType::new();

    for score in &scores {
        final_json.add(
            JsonType::OBJECT{
                name: util::get_enum_name(score.0),
                value: Box::new(JsonType::NUMBER(*score.1))
            }
        );
    }

    final_json.print()
}
