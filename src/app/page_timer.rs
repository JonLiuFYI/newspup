//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use crate::app::timer_state::TimerState;

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        const TIMER_DURATION: f64 = 10.; // TODO: allow choosing a time
        let app_time = ui.ctx().input(|i| i.time);

        // timer display
        match self.timer_state {
            TimerState::Started(start_time) => {
                // app was relaunched while a timer was running; reset the timer
                if start_time > app_time {
                    self.timer_state = TimerState::Stopped;
                }

                let elapsed_time = app_time - start_time;
                let seconds_remaining = TIMER_DURATION - elapsed_time;

                ui.heading(format!("{seconds_remaining:.1}"));

                if seconds_remaining <= 0. {
                    self.timer_state = TimerState::TimeUp;
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
            self.timer_state = TimerState::Started(app_time);
            dbg!(self.timer_state);
        }
        if timer_reset_btn.clicked() {
            self.timer_state = TimerState::Stopped;
        }

        ui.label(format!("(Elapsed: {app_time:.1})"));

        ui.ctx().request_repaint();
    }
}
