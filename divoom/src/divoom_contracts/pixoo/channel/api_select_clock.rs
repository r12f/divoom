#[doc = include_str!("./api_select_clock.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/SetClockSelectId",
    DivoomPixooCommandChannelSelectClockRequest,
    DivoomPixooCommandChannelSelectClockRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelSelectClockRequestPayload {
    pub clock_id: i32,
}

impl DivoomPixooCommandChannelSelectClockRequestPayload {
    pub fn new(clock_id: i32) -> DivoomPixooCommandChannelSelectClockRequestPayload {
        DivoomPixooCommandChannelSelectClockRequestPayload { clock_id }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandChannelSelectClockResponse);
