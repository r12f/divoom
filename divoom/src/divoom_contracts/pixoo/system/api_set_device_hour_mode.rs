#[doc = include_str!("./api_set_device_hour_mode.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceHourMode;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetTime24Flag",
    DivoomPixooCommandSystemSetHourModeRequest,
    DivoomPixooCommandSystemSetHourModeRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetHourModeRequestPayload {
    /// 0:12-hour; 1:24-hour
    pub mode: i32,
}

impl DivoomPixooCommandSystemSetHourModeRequestPayload {
    pub fn new(mode: DivoomDeviceHourMode) -> DivoomPixooCommandSystemSetHourModeRequestPayload {
        DivoomPixooCommandSystemSetHourModeRequestPayload {
            mode: match mode {
                DivoomDeviceHourMode::Hour12 => 0,
                DivoomDeviceHourMode::Hour24 => 1,
                DivoomDeviceHourMode::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetHourModeResponse);
