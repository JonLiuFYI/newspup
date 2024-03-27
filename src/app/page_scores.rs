//! Page that accepts score inputs and displays scoreboard
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use egui::{Align, DragValue, Layout, RichText};

use crate::consts::SUBPAGE_ICONS;

use super::app_state::{Round, RoundSubpage, SUBPAGES};
use super::score_models::CalcScore;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        // immutable copy for clarity so there's less [] nesting
        let subpage_num = self.subpage[round];

        // subpage header
        ui.horizontal_top(|ui| {
            if ui.button(RichText::new("←").heading()).clicked() && self.subpage[round] > 0 {
                self.subpage[round] -= 1;
            }

            ui.horizontal(|ui| {
                ui.heading("•".repeat(subpage_num));
                ui.heading(SUBPAGE_ICONS[subpage_num]);
                ui.heading("•".repeat(7 - subpage_num));
            });

            // TODO: rework subpage handling to not depend on magic numbers. enum-iterator crate?
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                if ui.button(RichText::new("→").heading()).clicked() && self.subpage[round] < 7 {
                    self.subpage[round] += 1;
                }
            });
        });

        // score category heading
        ui.vertical_centered(|ui| {
            if SUBPAGES[subpage_num] != RoundSubpage::ShowScores {
                ui.heading(format!("{}", SUBPAGES[subpage_num]));
            } else {
                ui.heading(format!("{} is over!", round));
            }
        });

        // subpage contents
        if SUBPAGES[self.subpage[round]] == RoundSubpage::ShowScores {
            let loser = self.scores.sunday_dollars_loser();
            for (p, scorecol) in self.scores[round].iter().enumerate() {
                let round_pts = self.scores[round].calc_round_score(p);
                let round_dollars = scorecol.round_dollars();

                let total_pts = self.scores.total_pts_up_to(round, p);
                let total_dollars = self.scores.total_dollars_up_to(round, p);

                ui.heading(&self.names[p]);

                // sunday penalty for 3+ players
                if round == Round::Sun && loser.is_some_and(|loser| p == loser) {
                    ui.label(
                        egui::RichText::new("Bankrupt! Least money of all players.")
                            .small()
                            .color(ui.visuals().error_fg_color),
                    );
                }
                
                ui.label(format!("Round: {} pts, ${}", round_pts, round_dollars));
                ui.label(format!("Total: {} pts, ${}", total_pts, total_dollars));
            }
        } else {
            for (i, player_score) in &mut self.scores[round].iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&self.names[i]);

                    ui.add(
                        subpage_router(&SUBPAGES[self.subpage[round]], player_score)
                            .speed(0.02)
                            .max_decimals(0),
                    );
                });
            }
        }

        if cfg!(debug_assertions) {
            ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                if ui.button("Dump scoreboard").clicked() {
                    dbg!(&self.scores[Round::Fri]);
                    dbg!(&self.scores[Round::Sat]);
                    dbg!(&self.scores[Round::Sun]);
                }
            });
        }
    }
}

/// Show the DragValue for the given RoundSubpage
fn subpage_router<'a>(
    subpage: &'a RoundSubpage,
    player_score: &'a mut super::score_models::ScoreColumn,
) -> DragValue<'a> {
    match subpage {
        RoundSubpage::ArticlePts => DragValue::new(&mut player_score.article_pts)
            .suffix(" points")
            .clamp_range(0.0..=50.0),

        RoundSubpage::PhotoPts => DragValue::new(&mut player_score.photo_pts)
            .suffix(" points")
            .clamp_range(0.0..=50.0),

        RoundSubpage::CenterpiecePts => DragValue::new(&mut player_score.centerpiece_pts)
            .suffix(" points")
            .clamp_range(-2.0..=50.0),

        RoundSubpage::WhitespaceSize => DragValue::new(&mut player_score.whitespace_size)
            .suffix(" squares")
            .clamp_range(0.0..=50.0),

        RoundSubpage::MoodPenalty => DragValue::new(&mut player_score.mood_penalty)
            .suffix(" points lost")
            .clamp_range(0.0..=50.0),

        RoundSubpage::LeftoverPenalty => DragValue::new(&mut player_score.leftover_penalty)
            .suffix(" unused")
            .clamp_range(0.0..=50.0),

        RoundSubpage::AdDollars => DragValue::new(&mut player_score.ad_dollars)
            .suffix(" dollars")
            .clamp_range(0.0..=50.0),

        // ShowScores is already handled
        _ => DragValue::new(&mut player_score.ad_dollars).prefix(" YOU SHOULD NEVER SEE THIS!"),
    }
}
