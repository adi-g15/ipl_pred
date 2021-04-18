mod ipl;
mod util;
mod decl;
mod algo;
use decl::{ Teams };

fn main() {
    let matches_json = util::json_from_file("./data/matches.json");
    let mut matches = ipl::get_league_matches(&matches_json);

    println!("Total League Matches = {}", matches.len());

    ipl::chance_calculator(matches);
}
