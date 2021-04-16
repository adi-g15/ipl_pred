mod ipl;
mod util;
use ipl_pred;

fn main() {
    let matches_json = util::json_from_file("./data/matches.json").expect("JSON data couldn't be loaded");
    let mut matches = ipl::get_league_matches(&matches_json);

    println!("Total League Matches = {}", matches.len());

    ipl_pred::chance_calculator(matches);
}
