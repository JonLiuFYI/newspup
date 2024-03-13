//! Structs and impls for scorekeeping
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

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

impl ScoreColumn {
    /// Sum of points for this ScoreColumn, *excluding* whitespace.
    pub fn sum_no_whitespace(&self) -> f32 {
        self.article_pts + self.photo_pts + self.centerpiece_pts
            - self.mood_penalty
            - self.leftover_penalty
    }
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

pub trait CalcWhitespace {
    fn calc_whitespace(&self, player: usize) -> f32;
}

impl CalcWhitespace for Vec<ScoreColumn> {
    /// Calculate the points this player should get for their largest whitespace
    ///
    /// Implementation note: currently assumes three players. TODO: 2- and 1-player
    fn calc_whitespace(&self, player: usize) -> f32 {
        // get player's whitespace size
        let player_whitespace = self[player].whitespace_size as i32;

        let smallest_whitespace = self
            .iter()
            .map(|sc| sc.whitespace_size)
            .reduce(f32::min)
            .expect("whitespace_size should never be NaN") as i32;
        let largest_whitespace = self
            .iter()
            .map(|sc| sc.whitespace_size)
            .reduce(f32::max)
            .expect("whitespace_size should never be NaN") as i32;

        // 3-player whitespace scoring
        // If tied, each gets the award. +3 to everyone is possible.
        if player_whitespace == smallest_whitespace {
            3.
        } else if player_whitespace == largest_whitespace {
            -1.
        } else {
            1.
        }
    }
}
