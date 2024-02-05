#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub(crate) enum NewspupPage {
    Start,
    Scores(Round),
    Timer,
    Menu,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Eq, Hash)]
pub enum Round {
    Fri = 0,
    Sat,
    Sun,
}
