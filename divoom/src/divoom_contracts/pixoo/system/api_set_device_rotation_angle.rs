#[doc = include_str!("./api_set_device_rotation_angle.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceRotationAngle;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetScreenRotationAngle",
    DivoomPixooCommandSystemSetRotationAngleRequest,
    DivoomPixooCommandSystemSetRotationAngleRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetRotationAngleRequestPayload {
    /// 0:normal; 1:90; 2:180; 3:270
    pub mode: i32,
}

impl DivoomPixooCommandSystemSetRotationAngleRequestPayload {
    pub fn new(
        mode: DivoomDeviceRotationAngle,
    ) -> DivoomPixooCommandSystemSetRotationAngleRequestPayload {
        DivoomPixooCommandSystemSetRotationAngleRequestPayload {
            mode: match mode {
                DivoomDeviceRotationAngle::None => 0,
                DivoomDeviceRotationAngle::Rotate90 => 1,
                DivoomDeviceRotationAngle::Rotate180 => 2,
                DivoomDeviceRotationAngle::Rotate270 => 3,
                DivoomDeviceRotationAngle::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetRotationAngleResponse);
