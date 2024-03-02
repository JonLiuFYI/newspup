//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        ui.label("Timer");
    }
}
