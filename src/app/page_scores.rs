use egui::DragValue;

use super::app_state::Round;
use super::score_models::ScoreColumn;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        if ui.button("Dump scores").clicked() {
            dbg!(&self.scores.fri);
            dbg!(&self.scores.sat);
            dbg!(&self.scores.sun);
        }

        // TODO: extract as method for Scoreboard
        let round_scores: &mut Vec<ScoreColumn> = match &round {
            Round::Fri => &mut self.scores.fri,
            Round::Sat => &mut self.scores.sat,
            Round::Sun => &mut self.scores.sun,
        };

        for player_score in round_scores {
            ui.label("Player");
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
