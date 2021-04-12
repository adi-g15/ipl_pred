use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp;
use std::fmt;

static myTeams: Vec<i32> = vec![2,4,4,3];

enum Teams {
    CSK,
    MI,
    RCB,
    SRH,
    PBKS,
    KKR,
    DC,
    RR
}

impl fmt::Display for Teams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Teams::CSK => return write!(f, "CSK Team"),
        };
        write!(f, "IPL Team")
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} vs {}", self.a, self.b)
    }
}

impl PartialEq for Teams {
    fn eq(&self, other: &Teams) -> bool {
        return self == other;
    }
}

impl Eq for Teams {}

// impl PartialOrd for Teams {
//     fn partial_cmp(&self, other: &Teams) -> Option<cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Teams {
//     fn cmp(&self, other: &Teams) -> cmp::Ordering {
//         cmp::Ordering::Less
//     }
// }

struct Match {
    a: Teams, b: Teams
}

impl Match {
    // fn switch_teams(&self, )
}

struct IplScoreBoard {
    scores: HashMap<Teams,i8>
}

impl IplScoreBoard {
    fn team_won(&mut self, t: Teams) {
        self.scores.entry(t).and_modify(|e| { *e += 1 });
    }
}

// fn recurse(index: i32, matches: &mut Vec<(Match,bool)>) {   // true means `a` wins, false means `a` loses
//     if(index == matches.len()) {
//         // league matches complete

//         let curr_match = matches[index].0;

//         // sort matches accordingly
//     }

    

//     matches[index].1 = true;
//     recurse(index+1, matches);

//     matches[index].1 = false;
//     recurse(index+1, matches);
// }

fn main() {
    let mut initial_scores = HashMap::new();

    initial_scores.insert(Teams::CSK, 0);
    initial_scores.insert(Teams::DC, 0);
    initial_scores.insert(Teams::KKR, 0);
    initial_scores.insert(Teams::MI, 0);
    initial_scores.insert(Teams::PBKS, 0);
    initial_scores.insert(Teams::RCB, 0);
    initial_scores.insert(Teams::RR, 0);
    initial_scores.insert(Teams::SRH, 0);

    let mut points_table = IplScoreBoard {
        scores: initial_scores
    };

    let mut matches: Vec<Match> = Vec::new();

    // Read matches here
    matches.push( Match {
        a: Teams::MI,
        b: Teams::RCB,
    });
    matches.push( Match {
        a: Teams::MI,
        b: Teams::RCB,
    });

    for i in matches {
        println!("{}", i);
    }
}
