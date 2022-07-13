#[doc = include_str!("./api_set_countdown_tool.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomToolCountdownAction;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Tools/SetTimer",
    DivoomPixooCommandToolSetCountdownRequest,
    DivoomPixooCommandToolSetCountdownRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandToolSetCountdownRequestPayload {
    /// minute of countdown
    pub minute: i32,

    /// second of countdown
    pub second: i32,

    /// 0: stop, 1: start
    pub status: i32,
}

impl DivoomPixooCommandToolSetCountdownRequestPayload {
    pub fn new(
        minute: i32,
        second: i32,
        action: DivoomToolCountdownAction,
    ) -> DivoomPixooCommandToolSetCountdownRequestPayload {
        DivoomPixooCommandToolSetCountdownRequestPayload {
            minute,
            second,
            status: action.into(),
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandToolSetCountdownResponse);
