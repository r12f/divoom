#[doc = include_str!("./api_set_device_brightness.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/SetBrightness",
    DivoomPixooCommandSystemSetBrightnessRequest,
    DivoomPixooCommandSystemSetBrightnessRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetBrightnessRequestPayload {
    pub brightness: i32,
}

impl DivoomPixooCommandSystemSetBrightnessRequestPayload {
    pub fn new(brightness: i32) -> DivoomPixooCommandSystemSetBrightnessRequestPayload {
        DivoomPixooCommandSystemSetBrightnessRequestPayload { brightness }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetBrightnessResponse);
