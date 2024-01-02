use egui::DragValue;

use super::app_state::NewspupPage;
use super::app_state::Round;
use super::score_models::new_scores_table;
use super::score_models::ScoreDay;
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
            self.scores = new_scores_table();
            for _ in 0..self.num_players as i32 {
                self.scores
                    .get_mut(&Round::Fri)
                    .expect("has Round key")
                    .push(ScoreDay::default());
                self.scores
                    .get_mut(&Round::Sat)
                    .expect("has Round key")
                    .push(ScoreDay::default());
                self.scores
                    .get_mut(&Round::Sun)
                    .expect("has Round key")
                    .push(ScoreDay::default());
            }

            self.page = NewspupPage::Scores(Round::Fri);
        }
    }
}
