use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub(crate) enum NewspupPage {
    Start,
    Scores(Round),
    Timer,
    Menu,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub enum Round {
    Fri,
    Sat,
    Sun,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub enum RoundSubpage {
    ArticlePts = 0,
    PhotoPts,
    CenterpiecePts,
    WhitespaceSize,
    MoodPenalty,
    LeftoverPenalty,
    AdDollars,
    ShowScores,
}

impl Display for RoundSubpage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoundSubpage::ArticlePts => write!(f, "Points from Articles"),
            RoundSubpage::PhotoPts => write!(f, "Points from Photos"),
            RoundSubpage::CenterpiecePts => write!(f, "Points from Centerpiece"),
            RoundSubpage::WhitespaceSize => write!(f, "Size of Largest Whitespace"),
            RoundSubpage::MoodPenalty => write!(f, "Penalty for Mood"),
            RoundSubpage::LeftoverPenalty => write!(f, "Penalty for Leftovers"),
            RoundSubpage::AdDollars => write!(f, "Dollars from Ads"),
            RoundSubpage::ShowScores => write!(f, "Scoreboard"),
        }
    }
}

pub const SUBPAGES: [RoundSubpage; 8] = [
    RoundSubpage::ArticlePts,
    RoundSubpage::PhotoPts,
    RoundSubpage::CenterpiecePts,
    RoundSubpage::WhitespaceSize,
    RoundSubpage::MoodPenalty,
    RoundSubpage::LeftoverPenalty,
    RoundSubpage::AdDollars,
    RoundSubpage::ShowScores,
];

/// Track the current score subpage for each round. Tracked as a `usize` that keys to `SUBPAGES`.
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug, Default)]
pub struct CurrentRoundSubpage {
    fri: usize,
    sat: usize,
    sun: usize,
}

impl Index<Round> for CurrentRoundSubpage {
    type Output = usize;

    fn index(&self, round: Round) -> &Self::Output {
        match round {
            Round::Fri => &self.fri,
            Round::Sat => &self.sat,
            Round::Sun => &self.sun,
        }
    }
}

impl IndexMut<Round> for CurrentRoundSubpage {
    fn index_mut(&mut self, round: Round) -> &mut Self::Output {
        match round {
            Round::Fri => &mut self.fri,
            Round::Sat => &mut self.sat,
            Round::Sun => &mut self.sun,
        }
    }
}
