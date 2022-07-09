#[doc = include_str!("./api_set_device_white_balance.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetWhiteBalance",
    DivoomPixooCommandSystemSetWhiteBalanceRequest,
    DivoomPixooCommandSystemSetWhiteBalanceRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetWhiteBalanceRequestPayload {
    pub r_value: i32,
    pub g_value: i32,
    pub b_value: i32,
}

impl DivoomPixooCommandSystemSetWhiteBalanceRequestPayload {
    pub fn new(r: i32, g: i32, b: i32) -> DivoomPixooCommandSystemSetWhiteBalanceRequestPayload {
        DivoomPixooCommandSystemSetWhiteBalanceRequestPayload {
            r_value: r,
            g_value: g,
            b_value: b,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetWhiteBalanceResponse);
