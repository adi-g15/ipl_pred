use crate::algo::max_element;
use crate::decl::{IplLeagueMatch, IplScoreBoard, Teams};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (js_namespace = console)]
    fn log(msg: &str);

    #[wasm_bindgen (js_namespace = console, js_name = log)]
    fn log_usize(num: usize);

    #[wasm_bindgen (js_namespace = console)]
    fn error(msg: &str);
}

#[allow(non_upper_case_globals)]
static mut total_iterations: usize = 0;
#[allow(non_upper_case_globals)]
pub static mut minimum_wins_qualification: u8 = 14; // say a team won 14 of it, is the maximum possible wins, initialising with maximum

fn recurse(
    matches: &[IplLeagueMatch],
    index: usize,
    points_table: &mut IplScoreBoard,
    all_pos_bucket: &mut [HashSet<[u8; 8]>; 10],
) {
    // true means `a` wins, false means `a` loses
    unsafe {
        total_iterations += 1;
    }

    if index % 10 == 0 {
        if all_pos_bucket[index / 10].insert(points_table.scores) == false {
            return; // no need to recurse further; this path already tried
        }
    }

    if index == matches.len() {
        // league matches complete (`after` the last match)
        points_table.total_possibilities += 1;

        let mut original_scores: [(u8, usize); 4] = [(0u8, 0); 4]; // original top 4 scores and indices

        let (current_maximum, max_index) = max_element(&points_table.scores);
        original_scores[0] = (current_maximum, max_index);
        points_table.scores[max_index] = 0;

        let (current_maximum, max_index) = max_element(&points_table.scores);
        original_scores[1] = (current_maximum, max_index);
        points_table.scores[max_index] = 0;

        let (current_maximum, max_index) = max_element(&points_table.scores);
        original_scores[2] = (current_maximum, max_index);
        points_table.scores[max_index] = 0;

        let (current_maximum, max_index) = max_element(&points_table.scores);
        original_scores[3] = (current_maximum, max_index);
        points_table.scores[max_index] = 0;

        // we got the 4th lowest score from top
        let lowest_qualifying_score = current_maximum;
        unsafe {
            minimum_wins_qualification = minimum_wins_qualification.min(lowest_qualifying_score);
        }

        // restore original values
        for i in &original_scores {
            let org = *i;
            points_table.scores[org.1] = org.0;
        }

        // chose which teams qualified
        for i in 0..8 {
            if points_table.scores[i as usize] >= lowest_qualifying_score {
                points_table.total_qualifications[i as usize] += 1;
            }
        }

        return;
    }

    match matches[index].winner {
        Some(winner) => {
            points_table.team_won(winner);
            recurse(matches, index + 1, points_table, all_pos_bucket);
        }
        None => {
            points_table.team_won(matches[index].team1);
            recurse(matches, index + 1, points_table, all_pos_bucket);

            points_table.team_lost(matches[index].team1);
            points_table.team_won(matches[index].team2);
            recurse(matches, index + 1, points_table, all_pos_bucket);
            points_table.team_lost(matches[index].team2);
        }
    };
}

/*
Only call with force_find_till_end == true, when you are not mad

Instead, use
`extra_matches_to_compute`
to tell how many `non-completed` matches to compute
*/
pub fn chance_calculator(
    matches: Vec<IplLeagueMatch>,
    force_find_till_end: bool,
    extra_matches_to_compute: u8,
) -> HashMap<Teams, f64> {
    log("Inside chance calculator");

    unsafe {
        total_iterations = 0;
    }
    let mut points_table = IplScoreBoard::new();
    let mut all_pos_bucket: [HashSet<[u8; 8]>; 10] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

    // BUG REGION - These three lines will cause a panic
    // recurse(&matches[0..5], 0, &mut points_table, &mut all_pos);
    // recurse(&matches[5..10], 0, &mut points_table, &mut all_pos);
    // recurse(&matches[10..15], 0, &mut points_table, &mut all_pos);
    // BUG REGION

    let mut finished_matches_count = 0u8;
    for i in &matches {
        if i.winner == None {
            break;
        }
        finished_matches_count += 1;
    }

    log(&format!("Already finished matches: {}", finished_matches_count));

    let end_index = if force_find_till_end {
        matches.len()
    } else {
        (finished_matches_count as usize + extra_matches_to_compute as usize).min(matches.len())
    };

    // Calling this in WASM fails
    // let now = time::Instant::now();
    log("Going to recurse");
    recurse(
        &matches[0..end_index],
        0,
        &mut points_table,
        &mut all_pos_bucket,
    );
    log("Completed recurse");

    log(&format!("\nTill {} matches;", end_index));
    unsafe {
        log(&format!("Total Iterations: {}", total_iterations));
    }
    log("\nQualify Possiblities");
    let mut i = 0;
    for total_qualified in &points_table.total_qualifications {
        let team_val = match Teams::n(i) {
            Some(team_enum) => team_enum,
            None => panic!("Unknown value {} for conversion into Teams enum", i),
        };

        log(&format!(
            "{:?} -> {} %",
            team_val,
            100f64 * ((*total_qualified as f64) / points_table.total_possibilities as f64)
        ));
        i += 1;
    }

    unsafe {
        log(&format!("Minimum points needed to qualify: {}", 2 * minimum_wins_qualification));
    }

    log(&format!("\n{:?}\n", (points_table)));

    let mut diff_possibilities = Vec::new();
    for all_pos in &all_pos_bucket {
        diff_possibilities.push(all_pos.len());
    }

    log(&format!("भिन्न संभावनाएं : {:?}", diff_possibilities));

    log("\n#CSK 2021 😁\n");

    let mut final_possibilities = HashMap::new();

    let mut i = 0;
    for total_qualified in &points_table.total_qualifications {
        let team_enum = match Teams::n(i) {
            Some(team_enum) => team_enum,
            None => panic!("Unknown value {} for conversion into Teams enum", i),
        };

        final_possibilities.insert(
            team_enum,
            100f64 * (*total_qualified as f64 / points_table.total_possibilities as f64),
        );
        i += 1;
    }
    assert_eq!(i, 8); // ie. the loop ran AT MAX 7 times

    final_possibilities
}
