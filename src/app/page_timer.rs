//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use crate::app::timer_state::TimerState;

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        let app_time = ui.ctx().input(|i| i.time);

        // timer display
        match self.timer_state {
            // timer setup screen
            TimerState::Stopped => {
                ui.heading(format!("{:.1}", self.selected_duration));

                if ui.button("3:00 — Frantic").clicked() {
                    self.selected_duration = 5.; // 180
                }
                if ui.button("4:00 — Standard").clicked() {
                    self.selected_duration = 10.; // 240
                }
                if ui.button("5:00 — Relaxed").clicked() {
                    self.selected_duration = 15.; // 300
                }

                if ui.button("Start").clicked() {
                    self.timer_state = TimerState::Started {
                        start_time: app_time,
                        duration: self.selected_duration,
                    };
                    dbg!(self.timer_state);
                }
            }

            TimerState::Started {
                start_time,
                duration,
            } => {
                // app was relaunched while a timer was running; reset the timer
                if start_time > app_time {
                    self.timer_state = TimerState::Stopped;
                }

                let timer_elapsed = app_time - start_time;
                let seconds_remaining = duration - timer_elapsed;

                ui.heading(format!("{seconds_remaining:.1}"));

                if seconds_remaining <= 0. {
                    self.timer_state = TimerState::TimeUp;
                }

                if ui.button("Pause").clicked() {
                    self.timer_state = TimerState::Paused(seconds_remaining);
                }
            }

            TimerState::Paused(seconds_remaining) => {
                ui.heading(format!("{seconds_remaining:.1} (Paused)"));

                if ui.button("Resume").clicked() {
                    if let TimerState::Paused(seconds_remaining) = self.timer_state {
                        self.timer_state = TimerState::Started {
                            start_time: app_time,
                            duration: seconds_remaining,
                        };
                    }
                }
                if ui.button("Reset").clicked() {
                    self.timer_state = TimerState::Stopped;
                }
            }

            TimerState::TimeUp => {
                ui.heading("Time's up!");

                if ui.button("Reset").clicked() {
                    self.timer_state = TimerState::Stopped;
                }
            }
        }

        ui.ctx().request_repaint();
    }
}
