mod ipl;
mod util;
mod decl;
mod algo;
use decl::{ Teams };

fn main() {
    let matches_json = util::json_from_file("./data/matches.json");
    let mut matches = ipl::get_league_matches(&matches_json);

    // matches.resize(10, decl::IplLeagueMatch {
    //     team1: Teams::CSK,
    //     team2: Teams::CSK,
    //     result: None,
    //     venue: None,
    //     date: String::new()
    // });

    println!("Total League Matches = {}", matches.len());

    ipl::chance_calculator(matches);
}
