use egui::DragValue;

use super::app_state::Round;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        let round = match round {
            Round::Fri => "Friday",
            Round::Sat => "Saturday",
            Round::Sun => "Sunday",
        };
        ui.heading(round);

        if ui.button("Dump scores").clicked() {
            for ps in &self.scores {
                dbg!(ps);
            }
        }

        for ps in &mut self.scores {
            ui.label("Player");
            ui.horizontal(|ui| {
                ui.label("Articles");
                ui.add(
                    DragValue::new(&mut ps.fri.articles)
                        .clamp_range(0.0..=50.0)
                        .speed(0.02)
                        .max_decimals(0),
                );
            });
        }
    }
}
