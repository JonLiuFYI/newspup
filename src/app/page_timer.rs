//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use egui::Button;

use crate::app::timer_state::TimerState;

use super::NewspupApp;

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        let app_time = ui.ctx().input(|i| i.time);
        let mut seconds_remaining: f64 = 0.;

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
                seconds_remaining = duration - timer_elapsed;

                ui.heading(format!("{seconds_remaining:.1}"));

                if seconds_remaining <= 0. {
                    self.timer_state = TimerState::TimeUp;
                }
                self.timer_controls(ui, seconds_remaining, app_time);
            }
            TimerState::TimeUp => {
                ui.heading("Time's up!");
                self.timer_controls(ui, seconds_remaining, app_time);
            }
            TimerState::Paused(seconds_remaining) => {
                ui.heading(format!("{seconds_remaining:.1} (Paused)"));
                self.timer_controls(ui, seconds_remaining, app_time);
            }
        }

        ui.ctx().request_repaint();
    }

    fn timer_controls(&mut self, ui: &mut egui::Ui, seconds_remaining: f64, app_time: f64) {
        let pause_btn = ui.add_enabled(
            matches!(self.timer_state, TimerState::Started { .. }),
            Button::new("Pause"),
        );
        let resume_btn = ui.add_enabled(
            matches!(self.timer_state, TimerState::Paused(_)),
            Button::new("Resume"),
        );
        let reset_btn = ui.add_enabled(
            self.timer_state == TimerState::TimeUp
                || matches!(self.timer_state, TimerState::Paused(_)),
            Button::new("Reset"),
        );

        if pause_btn.clicked() {
            self.timer_state = TimerState::Paused(seconds_remaining);
        }
        if resume_btn.clicked() {
            if let TimerState::Paused(seconds_remaining) = self.timer_state {
                self.timer_state = TimerState::Started {
                    start_time: app_time,
                    duration: seconds_remaining,
                };
            }
        }
        if reset_btn.clicked() {
            self.timer_state = TimerState::Stopped;
        }

        ui.label(format!("(Elapsed: {app_time:.1})"));
    }
}
