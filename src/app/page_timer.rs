//! Timer page
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

use egui::DragValue;

use crate::{app::timer_state::TimerState, consts::TIMER_SEC_STEP};

use super::{timer_state::MinSec, NewspupApp};

impl NewspupApp {
    pub(crate) fn page_timer(&mut self, ui: &mut egui::Ui) {
        let app_time = ui.ctx().input(|i| i.time);

        // timer display
        match self.timer_state {
            // timer setup screen
            TimerState::Stopped => {
                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.timer_select.min)
                            .clamp_range(0.0..=10.0)
                            .speed(0.02)
                            .max_decimals(0),
                    );
                    ui.label(":");
                    ui.add(
                        DragValue::new(&mut self.timer_select.sec)
                            .clamp_range(0.0..=3.0)
                            .speed(0.02)
                            .max_decimals(0)
                            .custom_formatter(|seconds, _| {
                                format!("{:02}", seconds * TIMER_SEC_STEP)
                            })
                            .custom_parser(|inp| {
                                inp.parse::<f64>().and_then(|s| Ok(s.div_euclid(15.))).ok()
                            }),
                    );
                });

                if ui.button("3:00 — Frantic").clicked() {
                    self.timer_select = MinSec { min: 3., sec: 0. };
                }
                if ui.button("4:00 — Standard").clicked() {
                    self.timer_select = MinSec { min: 4., sec: 0. };
                }
                if ui.button("5:00 — Relaxed").clicked() {
                    self.timer_select = MinSec { min: 5., sec: 0. };
                }

                if ui.button("Start").clicked() {
                    self.timer_state = TimerState::Started {
                        start_time: app_time,
                        duration: self.timer_select.min * 60.
                            + self.timer_select.sec * TIMER_SEC_STEP,
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

                let time_remaining: MinSec = seconds_remaining.into();
                ui.heading(format!(
                    "{}:{:04.1}",
                    time_remaining.min, time_remaining.sec
                ));

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
