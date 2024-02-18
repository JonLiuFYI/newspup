use egui::{Align, DragValue, Layout, RichText};

use crate::consts::SUBPAGE_ICONS;

use super::app_state::{Round, SUBPAGES};
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        // subpage header
        ui.horizontal_top(|ui| {
            if ui.button(RichText::new("←").heading()).clicked() && self.subpage[round] > 0 {
                self.subpage[round] -= 1;
            }

            {
                // subpage_num is just an immutable copy of the real value, so prevent
                // accidentally using it by using a brace scope
                let subpage_num = self.subpage[round];

                ui.horizontal(|ui| {
                    ui.heading("•".repeat(subpage_num));
                    ui.heading(SUBPAGE_ICONS[subpage_num]);
                    ui.heading("•".repeat(7 - subpage_num));
                });
            }

            // TODO: rework subpage handling to not depend on magic numbers. enum-iterator crate?
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                if ui.button(RichText::new("→").heading()).clicked() && self.subpage[round] < 7 {
                    self.subpage[round] += 1;
                }
            });
        });

        ui.vertical_centered(|ui| {
            ui.heading(format!("{}", SUBPAGES[self.subpage[round]]));
        });

        // subpage score inputs
        for (i, player_score) in &mut self.scores[round].iter_mut().enumerate() {
            ui.heading(&self.names[i]);
            ui.horizontal(|ui| {
                ui.label("Articles");
                ui.add(
                    DragValue::new(&mut player_score.article_pts)
                        .clamp_range(0.0..=50.0)
                        .speed(0.02)
                        .max_decimals(0),
                );
            });
        }

        if ui.button("Dump scores").clicked() {
            dbg!(&self.scores[Round::Fri]);
            dbg!(&self.scores[Round::Sat]);
            dbg!(&self.scores[Round::Sun]);
        }
    }
}
