#[doc = include_str!("./api_set_device_mirror_mode.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceMirrorMode;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetMirrorMode",
    DivoomPixooCommandSystemSetMirrorModeRequest,
    DivoomPixooCommandSystemSetMirrorModeRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetMirrorModeRequestPayload {
    /// 0:disable; 1:enable
    pub mode: i32,
}

impl DivoomPixooCommandSystemSetMirrorModeRequestPayload {
    pub fn new(
        mode: DivoomDeviceMirrorMode,
    ) -> DivoomPixooCommandSystemSetMirrorModeRequestPayload {
        DivoomPixooCommandSystemSetMirrorModeRequestPayload { mode: mode.into() }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetMirrorModeResponse);
