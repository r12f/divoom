#[doc = include_str!("./api_set_device_high_light_mode.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceHighLightMode;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetHighLightMode",
    DivoomPixooCommandSystemSetHighLightModeRequest,
    DivoomPixooCommandSystemSetHighLightModeRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetHighLightModeRequestPayload {
    /// 0: close; 1: open;
    pub mode: i32,
}

impl DivoomPixooCommandSystemSetHighLightModeRequestPayload {
    pub fn new(
        mode: DivoomDeviceHighLightMode,
    ) -> DivoomPixooCommandSystemSetHighLightModeRequestPayload {
        DivoomPixooCommandSystemSetHighLightModeRequestPayload { mode: mode.into() }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetHighLightModeResponse);
