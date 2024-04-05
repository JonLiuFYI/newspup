#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub enum TimerState {
    Stopped,
    Started(f64),
    Paused,
    TimeUp,
}
