//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use crate::app::timer_state::TimerState;

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        const TIMER_DURATION: f64 = 10.; // TODO: allow choosing a time
        let app_time = ui.ctx().input(|i| i.time);

        // reset start_time if app was relaunched
        if self.start_time.is_some_and(|st| st > app_time) {
            self.start_time = None;
            self.timer_state = TimerState::Stopped;
        }

        // timer display
        // TODO: add value to enum: Started(start_time)
        match self.timer_state {
            TimerState::Started => {
                if let Some(start_time) = self.start_time {
                    let elapsed_time = app_time - start_time;
                    let seconds_remaining = TIMER_DURATION - elapsed_time;

                    ui.heading(format!("{seconds_remaining:.1}"));

                    if seconds_remaining <= 0. {
                        self.timer_state = TimerState::TimeUp;
                    }
                }
            }
            TimerState::Stopped => {
                ui.heading(format!("{TIMER_DURATION:.1}"));
            }
            TimerState::TimeUp => {
                ui.heading("Time's up!");
            }
            _ => {
                ui.label("TEMP");
            }
        }

        // controls
        let timer_start_btn = ui.add_enabled(
            self.timer_state == TimerState::Stopped,
            egui::Button::new("Start"),
        );
        let timer_reset_btn = ui.add_enabled(
            self.timer_state == TimerState::TimeUp || self.timer_state == TimerState::Paused,
            egui::Button::new("Reset"),
        );

        if timer_start_btn.clicked() {
            self.start_time = Some(app_time);
            self.timer_state = TimerState::Started;
            dbg!(self.start_time);
        }
        if timer_reset_btn.clicked() {
            self.timer_state = TimerState::Stopped;
        }

        ui.label(format!("(Elapsed: {app_time:.1})"));

        ui.ctx().request_repaint();
    }
}
