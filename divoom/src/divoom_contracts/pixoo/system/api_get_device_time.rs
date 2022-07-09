#[doc = include_str!("./api_get_device_time.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Device/GetDeviceTime",
    DivoomPixooCommandSystemGetDeviceTimeRequest
);

// Response
define_pixoo_command_response!(
    DivoomPixooCommandSystemGetDeviceTimeResponse,
    DivoomPixooCommandSystemGetDeviceTimeResponsePayload,
    u64
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemGetDeviceTimeResponsePayload {
    #[serde(rename = "UTCTime")]
    pub utc_time: u64,
    pub local_time: String,
}

impl DivoomPixooCommandSystemGetDeviceTimeResponsePayload {
    pub fn destructive_into(self) -> u64 {
        self.utc_time
    }
}
