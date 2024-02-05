//! Structs and impls for scorekeeping

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
    pub fri: Vec<ScoreColumn>,
    pub sat: Vec<ScoreColumn>,
    pub sun: Vec<ScoreColumn>,
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
