use crate::decl::{IplLeagueMatch, IplScoreBoard, JsonType, Teams};
use crate::{ipl, util};
use std::collections::HashSet;
use std::time;

struct RecursionStats {
    time_elapsed: f32,
    total_iterations: u64,
}

// i want a copy of matches vector here
fn chance_calculator_after_n_matches(
    mut matches: Vec<IplLeagueMatch>,
    after_num_finished_matches: usize,
    all_pos_buckets: &mut [HashSet<[u8; 8]>; 10],
) -> (RecursionStats, IplScoreBoard) {
    for i in after_num_finished_matches..matches.len() {
        matches[i].winner = None;
    }

    println!(
        "Calculating for Finished Matches: {}",
        after_num_finished_matches
    );

    let mut score_board = IplScoreBoard::new();

    let now = time::Instant::now();
    unsafe {
        ipl::total_iterations = 0;
    }

    ipl::recurse(&matches, 0, &mut score_board, all_pos_buckets);

    #[allow(unused_mut)]
    let mut total_iterations: u64;
    let time_elapsed = now.elapsed().as_secs_f32();

    unsafe {
        println!(
            "After {0} matches results\n\"{0}_msg\": \"Time elapsed: {1}s; Total Iterations: {2}\",",
            after_num_finished_matches,
            time_elapsed,
            ipl::total_iterations
        );

        total_iterations = ipl::total_iterations as u64;
    }

    (
        RecursionStats {
            time_elapsed: time_elapsed,
            total_iterations: total_iterations,
        },
        score_board,
    )
}

pub fn sync_graph_values(matches: Vec<IplLeagueMatch>) {
    let mut graph_data = util::json_from_file("data/graph_data.json");

    let mut should_compute_for_after_match = Vec::with_capacity(matches.len());
    should_compute_for_after_match.resize(matches.len(), true);

    match graph_data {
        JsonType::JSON(ref all_matches) => {
            for entry in all_matches {
                match entry {
                    JsonType::OBJECT { name, .. } => {
                        if name.contains("_msg") {
                            continue;
                        }
                        match name.parse::<u8>() {
                            Ok(match_number) => {
                                should_compute_for_after_match[match_number as usize] = false;
                            }
                            Err(_) => (),
                        }
                    }
                    _ => (), // ignore non-objects
                }
            }
            // all_matches
        }
        _ => panic!(
            "Please first ensure that data/graph_data.json exists and has a top level JSON array"
        ),
    }

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
    let mut finished_matches_count = 0usize;
    for i in &matches {
        if i.winner == None {
            break;
        }
        finished_matches_count += 1;
    }

    println!("Finished Matches: {}", finished_matches_count);

    let lower_limit: usize = 0;
    let upper_limit: usize = finished_matches_count+1;

    for after_num_matches in (lower_limit..upper_limit).rev() {
        if !should_compute_for_after_match[after_num_matches] {
            println!("Skipping for Finished Matches: {}", after_num_matches);
            continue;
        }

        for bucket in &mut all_pos_bucket {
            bucket.clear();
        }

        let (recursion_stats, points_table) = chance_calculator_after_n_matches(
            matches.clone(),
            after_num_matches,
            &mut all_pos_bucket,
        );

        let mut i = 0;

        graph_data.add(JsonType::OBJECT {
            name: format!("{}_msg", after_num_matches),
            value: Box::new(JsonType::STRING(format!(
                "Time elapsed: {}s; Total Iterations: {}",
                recursion_stats.time_elapsed, recursion_stats.total_iterations
            ))),
        });

        unsafe {
            graph_data.add(JsonType::OBJECT {
                name: format!("{}_min_q", after_num_matches),
                value: Box::new(JsonType::NUMBER((2 * ipl::minimum_wins_qualification) as f64))
            });
        }

        let mut qualification_data = Vec::new();

        // printing qualifications
        println!("\"{}\": {{", after_num_matches);
        for total_qualified in &points_table.total_qualifications {
            let team_val = match Teams::n(i) {
                Some(team_enum) => team_enum,
                None => panic!("Unknown value {} for conversion into Teams enum", i),
            };

            println!(
                "\t\"{}\": {},",
                util::get_enum_name(team_val),
                100f64 * ((*total_qualified as f64) / points_table.total_possibilities as f64)
            );

            qualification_data.push(JsonType::OBJECT {
                name: util::get_enum_name(team_val),
                value: Box::new(JsonType::NUMBER(
                    100f64 * ((*total_qualified as f64) / points_table.total_possibilities as f64),
                )),
            });

            i += 1;
        }
        println!("}}\n\n\n");

        graph_data.add(JsonType::OBJECT {
            name: after_num_matches.to_string(),
            value: Box::new(JsonType::JSON(qualification_data)),
        });
    }

    match util::write_json_to_file(&graph_data, "data/graph_data.json") {
        Ok(_) => (),
        Err(err) => panic!("UNABLE TO WRITE GRAPH_DATA\n{:?}", err)
    }
}
