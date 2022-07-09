/// Countdown tool action
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomToolCountdownAction {
    Stop,
    Start,
    Raw(i32),
}

/// Noise tool action
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomToolNoiseAction {
    Stop,
    Start,
    Raw(i32),
}

/// Stopwatch tool action
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomToolStopwatchAction {
    Stop,
    Start,
    Reset,
    Raw(i32),
}
