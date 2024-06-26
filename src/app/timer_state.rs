//! Timer state and data
//! This file is part of Newspup. Copyright © 2023-2024 JonLiuFYI
//! SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub enum TimerState {
    /// timer is stopped; track selected timer duration
    Stopped,
    /// timer is running, started at `start_time`, will last for `duration` seconds
    Started {
        start_time: f64,
        duration: f64,
    },
    /// timer is paused; remembers seconds remaining when timer was paused
    Paused(f64),
    TimeUp,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub struct MinSec {
    pub min: f64,
    pub sec: f64,
}

impl From<MinSec> for f64 {
    fn from(minsec: MinSec) -> Self {
        minsec.min * 60. + minsec.sec
    }
}

impl From<f64> for MinSec {
    fn from(seconds: f64) -> Self {
        MinSec {
            min: seconds.div_euclid(60.),
            sec: seconds % 60.,
        }
    }
}
