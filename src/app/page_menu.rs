//! "Menu" page for global app actions and administrivia
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

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

        ui.heading("About Newspup");
        ui.label("Score counter and timer for Fit to Print");
        ui.label("Copyright © 2023-2024 JonLiuFYI");
        // TODO: link to repo
        ui.label("Newspup is free & open source software under the terms of the GNU Affero General Public License, version 3 or later.");
        ui.hyperlink_to("Source code ↗", "https://anexcellent.website/TODO");
        ui.hyperlink_to("GNU AGPLv3 ↗", "https://www.gnu.org/licenses/agpl-3.0");
        ui.hyperlink_to("Third Party Licenses ↗", "licenses.html");
    }
}
