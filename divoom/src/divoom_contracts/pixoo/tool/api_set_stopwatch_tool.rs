#[doc = include_str!("./api_set_stopwatch_tool.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomToolStopwatchAction;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Tools/SetStopWatch",
    DivoomPixooCommandToolSetStopwatchRequest,
    DivoomPixooCommandToolSetStopwatchRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandToolSetStopwatchRequestPayload {
    /// 0: stop, 1: start, 2: reset
    pub status: i32,
}

impl DivoomPixooCommandToolSetStopwatchRequestPayload {
    pub fn new(
        action: DivoomToolStopwatchAction,
    ) -> DivoomPixooCommandToolSetStopwatchRequestPayload {
        DivoomPixooCommandToolSetStopwatchRequestPayload {
            status: action.into(),
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandToolSetStopwatchResponse);
