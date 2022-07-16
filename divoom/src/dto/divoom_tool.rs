use super::divoom_dto_common::*;
use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// Countdown tool action
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomToolCountdownAction {
    Stop,
    Start,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomToolCountdownAction, Stop: "stop", Start: "start");

/// Noise tool action
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomToolNoiseAction {
    Stop,
    Start,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomToolNoiseAction, Stop: "stop", Start: "start");

/// Stopwatch tool action
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomToolStopwatchAction {
    Stop,
    Start,
    Reset,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomToolStopwatchAction, Stop: "stop", Start: "start", Reset: "reset");
