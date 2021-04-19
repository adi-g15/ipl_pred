use crate::{ ipl, util };
use crate::decl::{ IplLeagueMatch, IplScoreBoard, Teams };
use std::collections::HashSet;
use std::time;

pub fn sync_graph_values(mut matches: Vec<IplLeagueMatch>) {
    let graph_data = util::json_from_file("data/graph_data.json");
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

    #[allow(non_upper_case_globals)]
    const lower_limit: usize = 10;
    let upper_limit: usize = finished_matches_count+1;
    // let upper_limit: usize = finished_matches_count+1;

    for after_num_matches  in lower_limit..upper_limit {
        let mut points_table = IplScoreBoard::new();
        for i in after_num_matches..matches.len() {
            matches[i].winner = None;
        }

        let now = time::Instant::now();
        unsafe {
            ipl::max_i = 0;
        }

        ipl::recurse(
            &matches,
            0,
            &mut points_table,
            &mut all_pos_bucket,
        );

        let time_elapsed = now.elapsed().as_secs_f32();

        unsafe {
            println!("After {0} matches results\n\"{0}_msg\": \"Time elapsed: {1}s; Total Iterations: {2}\",", after_num_matches, time_elapsed, ipl::max_i);
        }
        let mut i = 0;
        println!("\"{}\": {{", after_num_matches);
        for total_qualified in &points_table.total_qualifications {
            let team_val = match Teams::n(i) {
                Some(team_enum) => team_enum,
                None => panic!("Unknown value {} for conversion into Teams enum", i)
            };

            println!(
                "\t\"{:?}\": {},",
                team_val,
                100f64 * ((*total_qualified as f64) / points_table.total_possibilities as f64)
            );
            i += 1;
        }
        println!("}}\n\n\n");
    }
}
