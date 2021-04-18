use json_minimal::Json;
use std::fmt;

pub type JsonType = Json;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Teams {
    CSK = 0,
    MI = 1,
    RCB = 2,
    SRH = 3,
    PBKS = 4,
    KKR = 5,
    DC = 6,
    RR = 7
}

#[derive(Clone)]
pub struct IplLeagueMatch {
    pub team1: Teams,
    pub team2: Teams,
    pub winner: Option<Teams>,  // if its already decided, it will have the winner team
    pub venue: Option<String>,
    pub date: String
}

impl fmt::Debug for IplLeagueMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IplLeagueMatch")
            .field("team1", &self.team1)
            .field("team2", &self.team2)
            .field("winner", &self.winner)
            .finish()
    }
}

#[derive(Debug)]
pub struct IplScoreBoard {
    pub total_possibilities: u32,
    pub total_qualifications: [u32; 8], // index as in Teams enums
    pub scores: [u8; 8] // scores, in index, as is Teams enum
}

impl IplScoreBoard {
    pub fn new() -> IplScoreBoard {
        IplScoreBoard {
            scores: [0u8; 8],
            total_qualifications: [0u32; 8],
            total_possibilities: 0
        }
    }

    // WHY CAN'T IT WORK WITHOUT BORROWING HERE
    pub fn team_won(&mut self, t: Teams) {
        self.scores[t as usize] += 1;
    }
    pub fn team_lost(&mut self, t: Teams) {
        self.scores[t as usize] -= 1;
    }
}
