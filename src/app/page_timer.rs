//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        const TIMER_DURATION: f64 = 10.; // TODO: allow choosing a time
        let app_time = ui.ctx().input(|i| i.time);

        // reset start_time if app was relaunched
        if self.start_time.is_some_and(|st| st > app_time) {
            self.start_time = None;
        }

        // timer display
        if let Some(start_time) = self.start_time {
            let elapsed_time = app_time - start_time;
            let seconds_remaining = TIMER_DURATION - elapsed_time;
            ui.heading(format!("{seconds_remaining:.1}"));
        } else {
            ui.heading(format!("{TIMER_DURATION:.1}"));
        }

        // controls
        if ui.button("Start").clicked() {
            self.start_time = Some(app_time);
            dbg!(self.start_time);
        }
        ui.label(format!("(Elapsed: {app_time:.1})"));

        ui.ctx().request_repaint();
    }
}
