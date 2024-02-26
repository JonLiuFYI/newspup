//! Structs and impls for scorekeeping

use std::ops::{Index, IndexMut};

use super::app_state::Round;

/// One day's worth of scores for one player, divided by category
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Default, Debug)]
pub struct ScoreColumn {
    pub article_pts: f32,
    pub photo_pts: f32,
    pub centerpiece_pts: f32,
    /// This is the **number of squares** of the largest whitespace, **not** the points!
    pub whitespace_size: f32,
    pub mood_penalty: f32,
    pub leftover_penalty: f32,
    pub ad_dollars: f32,
}

/// Full score table, grouped by day, then player
///
/// TODO: use something better than a Vec - avoidable allocation
///
/// Pseudocode model:
/// ```
/// // {
/// //     fri -> Vec containing a ScoreColumn for each player,
/// //     sat -> Vec containing a ScoreColumn for each player,
/// //     sun -> Vec containing a ScoreColumn for each player
/// // }
/// ```
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Default, Debug)]
pub struct Scoreboard {
    fri: Vec<ScoreColumn>,
    sat: Vec<ScoreColumn>,
    sun: Vec<ScoreColumn>,
}

impl Scoreboard {
    pub fn from_num_players(num_players: usize) -> Self {
        let vsc: Vec<ScoreColumn> = vec![ScoreColumn::default(); num_players];

        Self {
            fri: vsc.clone(),
            sat: vsc.clone(),
            sun: vsc,
        }
    }
}

impl Index<Round> for Scoreboard {
    type Output = Vec<ScoreColumn>;

    fn index(&self, index: Round) -> &Self::Output {
        match index {
            Round::Fri => &self.fri,
            Round::Sat => &self.sat,
            Round::Sun => &self.sun,
        }
    }
}

impl IndexMut<Round> for Scoreboard {
    fn index_mut(&mut self, index: Round) -> &mut Self::Output {
        match index {
            Round::Fri => &mut self.fri,
            Round::Sat => &mut self.sat,
            Round::Sun => &mut self.sun,
        }
    }
}

// pub trait ScoreCalc {
//     /// list of points each player gained this round, ordered by player order
//     fn points(&self) -> Vec<i32>;

//     /// list of dollars each player gained this round, ordered by player order
//     fn dollars(&self) -> Vec<i32>;
// }

// impl ScoreCalc for Vec<ScoreColumn> {
//     fn points(&self) -> Vec<i32> {
//         todo!()
//     }

//     fn dollars(&self) -> Vec<i32> {
//         self.iter().map(|sc| sc.ad_dollars as i32).collect()
//     }
// }
