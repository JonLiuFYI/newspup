#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub enum TimerState {
    /// timer is stopped; track selected timer duration
    Stopped(f64),
    /// timer is running, started at `start_time`, will last for `duration` seconds
    Started {
        start_time: f64,
        duration: f64,
    },
    /// timer is paused; remembers seconds remaining when timer was paused
    Paused(f64),
    TimeUp,
}
