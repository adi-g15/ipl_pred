use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
mod util;
mod ipl;

static Teams: [&str; 8] = [
    "CSK",
    "MI",
    "RCB",
    "SRH",
    "PBKS",
    "KKR",
    "DC",
    "RR"
];

type Match = ipl::IplLeagueMatch;

struct IplScoreBoard {
    scores: HashMap<String,i8>
}

impl IplScoreBoard {
    // WHY CAN'T IT WORK WITHOUT BORROWING HERE
    fn team_won(&mut self, t: &str) {
        self.scores.entry(t.to_string()).and_modify(|e| { *e += 1 });
    }
    fn team_lost(&mut self, t: &str) {
        self.scores.entry(t.to_string()).and_modify(|e| { *e -= 1 });
    }
}

fn recurse(matches: &[Match], index: usize, points_table: &mut IplScoreBoard, total_possibilities: &mut i32, team_qualifying: &mut HashMap<String,i32>) {   // true means `a` wins, false means `a` loses
    return;
    if index == matches.len() {
        // league matches complete (`after` the last match)
        *total_possibilities += 1;
        println!("Possibility: ");
        for row in points_table.scores.iter() {
            println!("\t{}: {}\n", row.0, row.1);
        }

        let mut lowest_qualifying_score = -1;
        let mut original_positions = [-1,-1,-1,-1];
        for row in points_table.scores.iter_mut() {
            if lowest_qualifying_score < *row.1  {
                lowest_qualifying_score = *row.1;
                // TODO - HANDLE SETTING ORIGNIAL VALUE TO 0, AND RESTORING IT
                // original_positions
            }
        }
        // points_table.scores[]
        for row in points_table.scores.iter_mut() {
            if lowest_qualifying_score < *row.1  {
                lowest_qualifying_score = *row.1;
            }
            *row.1 = 0;
        }
        for row in points_table.scores.iter_mut() {
            if lowest_qualifying_score < *row.1  {
                lowest_qualifying_score = *row.1;
            }
            *row.1 = 0;
        }
        for row in points_table.scores.iter_mut() {
            if lowest_qualifying_score < *row.1  {
                lowest_qualifying_score = *row.1;
            }
            *row.1 = 0;
        }
        // team_qualifying

        // sort matches accordingly
        // points_table.scores
        return;
    }

    points_table.team_won(&matches[index].team1);
    recurse(matches, index+1, points_table, total_possibilities, team_qualifying);

    points_table.team_lost(&matches[index].team1);
    points_table.team_won(&matches[index].team2);
    recurse(matches, index+1, points_table, total_possibilities, team_qualifying);
    points_table.team_lost(&matches[index].team2);
}

fn main() {
    let mut initial_scores = HashMap::new();
    for team in &Teams {
        initial_scores.insert(String::from(*team), 0);
    }

    let mut points_table = IplScoreBoard {
        scores: initial_scores
    };

    let matches_json = util::json_from_file("./data/matches.json").expect("JSON data couldn't be loaded");

    let mut matches = ipl::get_league_matches(&matches_json);

    println!("Total League Matches = {}", matches.len());

    let mut total_possibilities = 0;
    let mut team_qualifications = HashMap::new();
    recurse(&matches, 0, &mut points_table, &mut total_possibilities, &mut team_qualifications);
    for i in matches {
        print!("{} vs {}", i.team1, i.team2);
        println!(" -> {}", match i.result{ Some(result) => result, None => i.date});
    }
}
