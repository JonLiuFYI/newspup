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
    fn sum_no_whitespace(&self) -> f32 {
        self.article_pts + self.photo_pts + self.centerpiece_pts
            - self.mood_penalty
            - self.leftover_penalty
    }

    /// Get the ad_dollars. Lower bound is 0.
    ///
    /// Wait, is negative dollars ever possible?
    pub fn round_dollars(&self) -> f32 {
        if self.ad_dollars >= 0. {
            self.ad_dollars
        } else {
            0.
        }
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

    /// Get a player's total points up to the given round.
    pub fn total_pts_up_to(&self, round: Round, player: usize) -> f32 {
        self.fri.calc_round_score(player)
            + match round {
                Round::Fri => 0.,
                Round::Sat => self.sat.calc_round_score(player),
                Round::Sun => self.sat.calc_round_score(player) + self.sun.calc_round_score(player),
            }
    }

    /// Get a player's total dollars up to the given round.
    pub fn total_dollars_up_to(&self, round: Round, player: usize) -> f32 {
        self.fri[player].round_dollars()
            + match round {
                Round::Fri => 0.,
                Round::Sat => self.sat[player].round_dollars(),
                Round::Sun => self.sat[player].round_dollars() + self.sun[player].round_dollars(),
            }
    }

    /// Which player (if any) gets a penalty based on low money?
    pub fn sunday_dollars_loser(&self) -> Option<usize> {
        let num_players = self.fri.len();

        // 3+ players: if one player has the least money, they're penalized
        // penalty: eliminated
        if num_players >= 3 {
            let min_dollars = (0..num_players)
                .map(|p| self.total_dollars_up_to(Round::Sun, p))
                .reduce(f32::min)
                .expect("dollar count should never be NaN");

            let min_dollar_players: Vec<(usize, f32)> = (0..num_players)
                .map(|p| (p, self.total_dollars_up_to(Round::Sun, p)))
                .filter(|x| x.1 == min_dollars)
                .collect();

            if min_dollar_players.len() == 1 {
                Some(min_dollar_players[0].0)
            } else {
                None
            }
        }
        // 2 players: penalized if behind by $5 or more
        // penalty: -10 points
        else if num_players == 2 {
            let first_player_dollar_lead =
                self.total_dollars_up_to(Round::Sun, 0) - self.total_dollars_up_to(Round::Sun, 1);
            if first_player_dollar_lead >= 5. {
                Some(1)
            } else if first_player_dollar_lead <= -5. {
                Some(0)
            } else {
                None
            }
        }
        // 1 player: $11 or less
        // penalty: failure
        else {
            if self.total_dollars_up_to(Round::Sun, 0) <= 11. {
                Some(0)
            } else {
                None
            }
        }
    }

    /// Which players have the highest score, ignoring the eliminated player?
    /// Implementation note: assumes 3+ players
    pub fn winners(&self, loser: Option<usize>) -> Vec<usize> {
        let num_players = self.fri.len();
        let max_pts = (0..num_players)
            .map(|p| self.total_pts_up_to(Round::Sun, p))
            .reduce(f32::max)
            .expect("pts should never be NaN");

        (0..num_players)
            .map(|p| (p, self.total_pts_up_to(Round::Sun, p)))
            .filter_map(|(p, pts)| {
                if pts == max_pts && (loser.is_none() || loser.is_some_and(|loser| p != loser)) {
                    Some(p)
                } else {
                    None
                }
            })
            .collect()
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

pub trait CalcScore {
    fn calc_round_whitespace(&self, player: usize) -> f32;
    fn calc_round_score(&self, player: usize) -> f32;
}

impl CalcScore for Vec<ScoreColumn> {
    /// Calculate the points this player should get for their largest whitespace
    fn calc_round_whitespace(&self, player: usize) -> f32 {
        // get player's whitespace size
        let player_whitespace = self[player].whitespace_size as i32;

        // whitespace scoring for 2 and 3+ players
        if self.len() >= 2 {
            let smallest_whitespace =
                self.iter()
                    .map(|sc| sc.whitespace_size)
                    .reduce(f32::min)
                    .expect("whitespace_size should never be NaN") as i32;
            let largest_whitespace =
                self.iter()
                    .map(|sc| sc.whitespace_size)
                    .reduce(f32::max)
                    .expect("whitespace_size should never be NaN") as i32;

            // If tied, each gets the award. +3 to everyone is possible.
            if player_whitespace == smallest_whitespace {
                3.
            } else if player_whitespace == largest_whitespace {
                -1.
            } else {
                1.
            }
        }
        // whitespace scoring for 1 player
        else {
            match player_whitespace {
                0 | 1 => 3.,
                2 | 3 => 2.,
                4 | 5 => 1.,
                6 | 7 => 0.,
                _ => -1.,
            }
        }
    }

    /// Calculate the total score this player gained this round: whitespace plus
    /// the rest of their ScoreColumn. Lower bound is 0.
    fn calc_round_score(&self, player: usize) -> f32 {
        let sc = self[player];
        (sc.sum_no_whitespace() + self.calc_round_whitespace(player)).max(0.)
    }
}
