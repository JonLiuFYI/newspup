#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub(crate) enum NewspupPage {
    Start,
    Scores(Round),
    Timer,
    Menu,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone, Eq, Hash)]
pub(crate) enum Round {
    Fri,
    Sat,
    Sun,
}
