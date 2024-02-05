use egui::DragValue;

use super::app_state::NewspupPage;
use super::app_state::Round;
use super::score_models::ScoreColumn;
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

        for i in 1..=self.num_players as usize {
            ui.label(format!("Player {i}"));
        }

        if ui.button("Start Game").clicked() {
            self.scores = Scoreboard::from_num_players(self.num_players as usize);

            self.page = NewspupPage::Scores(Round::Fri);
        }
    }
}
