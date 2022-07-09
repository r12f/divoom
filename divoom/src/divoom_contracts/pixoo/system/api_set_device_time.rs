#[doc = include_str!("./api_set_device_time.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetUTC",
    DivoomPixooCommandSystemSetSystemTimeRequest,
    DivoomPixooCommandSystemSetSystemTimeRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetSystemTimeRequestPayload {
    pub utc: u64,
}

impl DivoomPixooCommandSystemSetSystemTimeRequestPayload {
    pub fn new(utc: u64) -> DivoomPixooCommandSystemSetSystemTimeRequestPayload {
        DivoomPixooCommandSystemSetSystemTimeRequestPayload { utc }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetSystemTimeResponse);
