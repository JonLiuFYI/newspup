//! Game setup page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use egui::DragValue;

use super::app_state::NewspupPage;
use super::app_state::Round;
use super::score_models::Scoreboard;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_start(&mut self, ui: &mut egui::Ui) {
        ui.heading("Newspup");
        ui.horizontal(|ui| {
            ui.label("How many players?");
            ui.add(
                DragValue::new(&mut self.num_players)
                    .clamp_range(1.0..=6.0)
                    .speed(0.02)
                    .max_decimals(0),
            );
        });

        for i in 0..self.num_players as usize {
            ui.horizontal(|ui| {
                ui.label(format!("Player {}", i + 1));
                ui.text_edit_singleline(&mut self.names[i]);
            });
            if self.names[i].is_empty() {
                ui.small(egui::RichText::new("⚠ Enter a name").color(ui.visuals().error_fg_color));
            }
        }

        let any_name_empty = self
            .names
            .iter()
            .take(self.num_players as usize)
            .any(std::string::String::is_empty);

        ui.add_enabled_ui(!any_name_empty, |ui| {
            if ui.button("Start Game").clicked() {
                self.scores = Scoreboard::from_num_players(self.num_players as usize);
                self.page = NewspupPage::Scores(Round::Fri);
            }
        });
    }
}
