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

        // subpage contents
        if SUBPAGES[self.subpage[round]] == RoundSubpage::ShowScores {
            for (p, scorecol) in &mut self.scores[round].iter_mut().enumerate() {
                // TODO: temp for figuring out score calc. Also still need to calc whitespace
                let score_sum_no_ws =
                    scorecol.article_pts + scorecol.photo_pts + scorecol.centerpiece_pts
                        - scorecol.mood_penalty
                        - scorecol.leftover_penalty;
                let score_whitespace: f32 = 0.; // TODO
                let score_sum = score_sum_no_ws + score_whitespace;

                ui.label(&self.names[p]);
                ui.label(format!(
                    "Round: {} pts, ${}",
                    score_sum, scorecol.ad_dollars
                ));
            }
        } else {
            for (i, player_score) in &mut self.scores[round].iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&self.names[i]);

                    ui.add(
                        subpage_router(&SUBPAGES[self.subpage[round]], player_score)
                            .clamp_range(0.0..=50.0)
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
        RoundSubpage::ArticlePts => DragValue::new(&mut player_score.article_pts).suffix(" points"),

        RoundSubpage::PhotoPts => DragValue::new(&mut player_score.photo_pts).suffix(" points"),

        RoundSubpage::CenterpiecePts => {
            DragValue::new(&mut player_score.centerpiece_pts).suffix(" points")
        }

        RoundSubpage::WhitespaceSize => {
            DragValue::new(&mut player_score.whitespace_size).suffix(" squares")
        }

        RoundSubpage::MoodPenalty => {
            DragValue::new(&mut player_score.mood_penalty).suffix(" points lost")
        }

        RoundSubpage::LeftoverPenalty => {
            DragValue::new(&mut player_score.leftover_penalty).suffix(" unused")
        }

        RoundSubpage::AdDollars => DragValue::new(&mut player_score.ad_dollars).suffix(" dollars"),

        // ShowScores is already handled
        _ => DragValue::new(&mut player_score.ad_dollars).prefix(" YOU SHOULD NEVER SEE THIS!"),
    }
}
