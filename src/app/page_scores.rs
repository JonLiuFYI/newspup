use super::app_state::Round;
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_scores(&mut self, round: Round, ui: &mut egui::Ui) {
        let round = match round {
            Round::Fri => "Friday",
            Round::Sat => "Saturday",
            Round::Sun => "Sunday",
        };
        ui.label(format!("Round {round}"));
    }
}
