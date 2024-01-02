//! Structs and impls for scorekeeping

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

/// Full-game (all three rounds) score table for one player
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Default, Debug)]
pub struct PlayerScore {
    pub fri: ScoreDay,
    pub sat: ScoreDay,
    pub sun: ScoreDay,
}
