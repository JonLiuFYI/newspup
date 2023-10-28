#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub(crate) enum NewspupPage {
    Start,
    Scores(Round),
    Timer,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
pub(crate) enum Round {
    Fri,
    Sat,
    Sun,
}
