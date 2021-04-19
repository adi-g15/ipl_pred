mod ipl;
mod decl;
mod algo;
mod util;

use wasm_bindgen::prelude::*;
use decl::{ JsonType };

/*
We can simply use `web_sys` crate, then we won't need these manual bindings
*/
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);

    /*Since console.log is overloaded for many cases,
    we can bind it with multiple signatures,
    though, remember we need to put the `js_name = log` to ensure we always call "log" in javascript
    since there is no function named log_u32 by default in javascript*/
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(num: u32);
}

#[wasm_bindgen]
pub fn get_chances(json_string: &str) -> String {
    log( "Inside get_chances rust" );
    let json = match JsonType::parse(json_string.as_bytes()) {
        Ok(json_obj) => json_obj,
        Err(err) => {
            panic!("Invalid json: {:?}", err)
        }
    };

    log( &format!( "Parsed JSON: {}", json.print()) );
    let scores = ipl::chance_calculator(ipl::get_league_matches(&json), false, 20);
    log( "Calculated chances" );

    let mut final_json = JsonType::new();

    for score in &scores {
        final_json.add(
            JsonType::OBJECT{
                name: util::get_enum_name(score.0),
                value: Box::new(JsonType::NUMBER(*score.1))
            }
        );
    }

    log( "Added scores, now returning..." );

    final_json.print()
}
