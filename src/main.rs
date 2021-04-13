use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
mod util;

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

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} vs {}", self.a, self.b)
    }
}

struct Match {
    a: String, b: String
}

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

    points_table.team_won(&matches[index].a);
    recurse(matches, index+1, points_table, total_possibilities, team_qualifying);

    points_table.team_lost(&matches[index].a);
    points_table.team_won(&matches[index].b);
    recurse(matches, index+1, points_table, total_possibilities, team_qualifying);
    points_table.team_lost(&matches[index].b);
}

fn main() {
    let mut initial_scores = HashMap::new();
    for team in &Teams {
        initial_scores.insert(String::from(*team), 0);
    }

    let mut points_table = IplScoreBoard {
        scores: initial_scores
    };

    let mut matches: Vec<Match> = Vec::new();

    // Read matches here
    let matches_json: util::JsonType = match util::json_from_file("../../../data/matches.json"){
        Some(json_obj) => json_obj,
        None => {
            panic!("JSON data couldn't be loaded");
        }
    };
    matches_json.print();

    match matches_json {
        util::JsonType::ARRAY(all_matches) => {
            // for row in all_matches.iter() {
                println!("Total Matches = {}", all_matches.len());
            // }
        },
        matches_json => {
            panic!("Expected JSON array, but found {:?}", matches_json);
        }
    }
    // for entry in matches_json. {
    //     entry.
    //     matches.push( Match {
    //         // a: String::from()
    //     })
    // }
    return;
    matches.push( Match {
        a: String::from("MI"),
        b: String::from("RCB"),
    });
    matches.push( Match {
        a: String::from("CSK"),
        b: String::from("DC"),
    });

    let mut total_possibilities = 0;
    let mut team_qualifications = HashMap::new();
    recurse(&matches, 0, &mut points_table, &mut total_possibilities, &mut team_qualifications);
    for i in matches {
        println!("{}", i);
    }
}
