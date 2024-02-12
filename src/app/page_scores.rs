use egui::DragValue;

use super::app_state::Round;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        if ui.button("Dump scores").clicked() {
            dbg!(&self.scores[Round::Fri]);
            dbg!(&self.scores[Round::Sat]);
            dbg!(&self.scores[Round::Sun]);
        }

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
    }
}
