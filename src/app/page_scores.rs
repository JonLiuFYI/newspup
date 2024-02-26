use egui::{Align, DragValue, Layout, RichText};

use crate::consts::SUBPAGE_ICONS;

use super::app_state::{Round, RoundSubpage, SUBPAGES};
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

        ui.vertical_centered(|ui| {
            if SUBPAGES[subpage_num] != RoundSubpage::ShowScores {
                ui.heading(format!("{}", SUBPAGES[subpage_num]));
            } else {
                ui.heading(format!("{} is over!", round));
            }
        });

        // subpage score inputs
        for (i, player_score) in &mut self.scores[round].iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(&self.names[i]);

                if SUBPAGES[self.subpage[round]] == RoundSubpage::ShowScores {
                    ui.label(format!("Round: {} pts, ${}", "TEMP", player_score.ad_dollars));
                } else {
                    // TODO: figure out how to extract this sin into a "router" method
                    ui.add(
                        match SUBPAGES[self.subpage[round]] {
                            RoundSubpage::ArticlePts => {
                                DragValue::new(&mut player_score.article_pts).suffix(" points")
                            }

                            RoundSubpage::PhotoPts => {
                                DragValue::new(&mut player_score.photo_pts).suffix(" points")
                            }

                            RoundSubpage::CenterpiecePts => {
                                DragValue::new(&mut player_score.centerpiece_pts).suffix(" points")
                            }

                            RoundSubpage::WhitespaceSize => {
                                DragValue::new(&mut player_score.whitespace_size).suffix(" squares")
                            }

                            RoundSubpage::MoodPenalty => {
                                DragValue::new(&mut player_score.mood_penalty)
                                    .suffix(" points lost")
                            }

                            RoundSubpage::LeftoverPenalty => {
                                DragValue::new(&mut player_score.leftover_penalty).suffix(" unused")
                            }

                            RoundSubpage::AdDollars => {
                                DragValue::new(&mut player_score.ad_dollars).suffix(" dollars")
                            }

                            // ShowScores is handled outside of this match block
                            RoundSubpage::ShowScores => {
                                DragValue::new(&mut player_score.ad_dollars)
                                    .prefix(" YOU SHOULD NEVER SEE THIS!")
                            }
                        }
                        .clamp_range(0.0..=50.0)
                        .speed(0.02)
                        .max_decimals(0),
                    );
                }
            });
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
