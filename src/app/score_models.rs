//! Structs and impls for scorekeeping

use std::collections::HashMap;

use super::app_state::Round;

/// One day's worth of scores for one player, divided by category
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Default, Debug)]
pub struct ScoreDay {
    pub articles: f32,
    pub photos: f32,
    pub centerpiece: f32,
    pub whitespace: f32,
    pub mood: f32,
    pub leftovers: f32,
    pub ads: f32,
}

// /// Full-game (all three rounds) score table for one player
// #[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Default, Debug)]
// pub struct PlayerScore {
//     pub fri: ScoreDay,
//     pub sat: ScoreDay,
//     pub sun: ScoreDay,
// }

/// Produce a new blank HashMap to represent all players' scores for all rounds.
///
/// Pseudocode model:
/// ```
/// // {
/// //     Round::Fri -> Vec containing a ScoreDay for each player,
/// //     Round::Sat -> Vec containing a ScoreDay for each player,
/// //     Round::Sun -> Vec containing a ScoreDay for each player
/// // }
/// ```
pub fn new_scores_table() -> HashMap<Round, Vec<ScoreDay>> {
    let mut out = HashMap::with_capacity(3);
    out.insert(Round::Fri, Vec::new());
    out.insert(Round::Sat, Vec::new());
    out.insert(Round::Sun, Vec::new());

    out
}
