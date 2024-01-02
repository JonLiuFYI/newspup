use egui::DragValue;

use super::app_state::Round;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        if ui.button("Dump scores").clicked() {
            dbg!(&self.scores[&Round::Fri]);
            dbg!(&self.scores[&Round::Sat]);
            dbg!(&self.scores[&Round::Sun]);
        }

        for s in self
            .scores
            .get_mut(&round)
            .expect("score table has key for each round")
        {
            ui.label("Player");
            ui.horizontal(|ui| {
                ui.label("Articles");
                ui.add(
                    DragValue::new(&mut s.articles)
                        .clamp_range(0.0..=50.0)
                        .speed(0.02)
                        .max_decimals(0),
                );
            });
        }
    }
}
