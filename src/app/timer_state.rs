#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Debug)]
pub enum TimerState {
    Stopped,
    Started,
    Paused,
    TimeUp,
}
