use crate::util;

use util::JsonType;
                /*
Reasons for using "&**value" ->

                    JsonType::OBJECT{name,value} => match *value {
   |                                                           ------
   |                                                           |
   |                                                           this expression has type `Box<Json>`
   |                                                           help: consider dereferencing the boxed value: `**value`
25 |                         JsonType::STRING(val_str) => println!("obj - {} {}", name, val_str)

JsonType::OBJECT{name,value} => match **value {
   |                                                           ^^^^^^^ help: consider borrowing here: `&**value`
25 |                         JsonType::STRING(val_str) => println!("obj - {} {}", name, val_str),
   |                                          ------- data moved here
26 |                         value => println!("obj - {} {}", name, value.print())
   |                         ----- ...and here

*/
// LEARNT - To ignore a field use ", ..", BUT NOTE THAT IT MUST BE AT END, and NO TRAILING commas

pub struct IplLeagueMatch {
    pub team1: String,
    pub team2: String,
    pub result: Option<String>,
    pub venue: Option<String>,
    pub date: String
}

impl IplLeagueMatch {
    fn new(team1: String, team2: String, date: String) -> IplLeagueMatch {
        IplLeagueMatch {
            team1: team1,
            team2: team2,
            result: None,
            venue: None,
            date: date
        }
    }
}

pub fn get_league_matches(json: &JsonType) -> Vec<IplLeagueMatch> {
    let mut all_matches = Vec::new();
    const TBC_TEAM_NAME: &str = "TBC";

    match json {
        JsonType::ARRAY(json_arr) => {
            for entry in json_arr {
                let team1 = entry.get("0").expect("Team name not found");
                let team2 = entry.get("1").expect("Team name not found");
                let res = match entry.get("res") {
                    Some(res_obj) => match res_obj {
                        JsonType::OBJECT{ value, .. } => match &**value {
                            JsonType::STRING(result_str) => String::from(result_str),
                            _value => String::new()
                        },
                        _res_obj => String::new()
                    },
                    None => String::new()
                };
                let venue = match entry.get("venue") {
                    Some(ven_obj) => match ven_obj {
                        JsonType::OBJECT{ value, .. } => match &**value {
                            JsonType::STRING(venue_str) => String::from(venue_str),
                            _value => String::new()
                        },
                        _ven_obj => String::new()
                    },
                    None => String::new()
                };
                let date = match entry.get("date") {
                    Some(date_obj) => match date_obj {
                        JsonType::OBJECT{ value, .. } => match &**value {
                            JsonType::STRING(date_str) => String::from(date_str),
                            _value => String::new()
                        },
                        _date_obj => String::new()
                    },
                    None => String::new()
                };

                let team1_name = match team1 {
                    JsonType::OBJECT{ value, .. } => match &**value {
                        JsonType::STRING(team1_name) => String::from(team1_name),
                        _value => String::new()
                    },
                    team1 => String::new()
                };
                let team2_name = match team2 {
                    JsonType::OBJECT{ value, .. } => match &**value {
                        JsonType::STRING(team2_name) => String::from(team2_name),
                        _value => String::new()
                    },
                    team2 => String::new()
                };

                // CONDITIONS   //
                if team1_name.is_empty() || team1_name.eq(TBC_TEAM_NAME) ||
                    team2_name.is_empty() || team2_name.eq(TBC_TEAM_NAME) ||
                    date.is_empty() {
                        continue;
                    }
                // CONDITIONS   //

                all_matches.push( IplLeagueMatch {
                    team1: team1_name,
                    team2: team2_name,
                    date: date,
                    venue: if venue.is_empty() { None } else { Some(venue) },
                    result: if res.is_empty() { None } else { Some(res) },
                } )
            }
        },
        json => {
            panic!("Expected JSON array, but found {:?}", json);
        }
    }

    // return JsonType::ARRAY( all_matches );
    return all_matches;   // all_matches is a vector here
}

