mod algo;
mod decl;
mod ipl;
mod ipl_with_wasm;
mod util;

use decl::JsonType;
use wasm_bindgen::prelude::*;

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
pub fn get_chances(json_string: &str, mut extra_matches_to_compute: i8) -> String {
    log("Inside get_chances rust");

    let compute_for_whole_ipl: bool = extra_matches_to_compute == -1;

    if extra_matches_to_compute < 0 {
        extra_matches_to_compute = 0;   // to make it valid
    }

    let json = match JsonType::parse(json_string.as_bytes()) {
        Ok(json_obj) => json_obj,
        Err(err) => {
            panic!("Invalid json: {:?}", err)
        }
    };

    let league_matches = ipl::get_league_matches(&json);
    log("Got league matches");

    let scores = ipl_with_wasm::chance_calculator(
        league_matches,
        compute_for_whole_ipl,
        extra_matches_to_compute as u8  // invariant: it is always >= 0, as when it's less than 0 is already handled
    );

    log("Calculated chances");
    let mut final_json = JsonType::new();

    for score in &scores {
        final_json.add(JsonType::OBJECT {
            name: util::get_enum_name(score.0),
            value: Box::new(JsonType::NUMBER(*score.1)),
        });
    }

    unsafe {
        final_json.add(JsonType::OBJECT {
            name: "min_qual".to_string(),
            value: Box::new( JsonType::NUMBER( (2*ipl_with_wasm::minimum_wins_qualification) as f64 ) )
        });
    }

    final_json.print()
}
