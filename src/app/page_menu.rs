use super::app_state::{CurrentRoundSubpage, NewspupPage};
use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_menu(&mut self, ui: &mut egui::Ui) {
        ui.heading("Menu");
        if ui.button("New Game").clicked() {
            self.page = NewspupPage::Start;
            self.subpage = CurrentRoundSubpage::default();
        }
        ui.label("Theme");
        egui::widgets::global_dark_light_mode_buttons(ui);
        ui.separator();
        ui.label("About text goes here");
    }
}
