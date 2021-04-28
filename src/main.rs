mod ipl;
mod util;
mod decl;
mod algo;
mod graph;

/**
 * THIS PROGRAM LISTS THE PERCENT CHANCES OF YOUR TEAM "QUALIFYING"
 * Kaun playoffs me jeete na jeete koi farak nahi padta hai isko :D
 *
 * Note - Better run in release mode, with "cargo run --release", since this is recursive, and takes GADAR amount of time in debug mode
 *
 * Basic Usage -
 *
 * If you want to compute chances for WHOLE IPL, see the 2nd commented ipl::chance_calculator call
 * If you want to compute chances for particular matches in future, see the other calls to ipl::chance_calculator
*/
fn main() {
    let matches_json = util::json_from_file("./data/matches.json");
    let matches = ipl::get_league_matches(&matches_json);

    println!("Total League Matches = {}", matches.len());

    // Calculate chances till 20 matches after (EXCLUDING already complete matches)
    // ipl::chance_calculator(matches, false, 20);

    // Calculate chances for WHOLE IPL, ie. till 56 matches
    // ipl::chance_calculator(matches, true, 0);   // 2nd arg doesn't matter in this case

    // Calculate chances till 30 matches after (EXCLUDING already complete matches)
    // ipl::chance_calculator(matches, false, 30);

    // Calculate MINIMUM qualification chances for a team
    // ipl::minimum_chances_calculator(matches, decl::Teams::RCB);   // 2nd arg doesn't matter in this case

    // To sync graph_data.json
    graph::sync_graph_values(matches);
}
