#[doc = include_str!("./api_get_selected_clock_info.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Channel/GetClockInfo",
    DivoomPixooCommandChannelGetClockInfoRequest
);

// Response
define_pixoo_command_response!(
    DivoomPixooCommandChannelGetClockInfoResponse,
    DivoomPixooCommandChannelGetClockInfoResponsePayload,
    DivoomSelectedClockInfo
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelGetClockInfoResponsePayload {
    pub clock_id: i32,
    pub brightness: i32,
}

impl DivoomPixooCommandChannelGetClockInfoResponsePayload {
    pub fn destructive_into(self) -> DivoomSelectedClockInfo {
        DivoomSelectedClockInfo {
            clock_id: self.clock_id,
            brightness: self.brightness,
        }
    }
}
