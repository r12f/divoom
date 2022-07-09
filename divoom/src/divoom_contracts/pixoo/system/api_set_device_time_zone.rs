#[doc = include_str!("./api_set_device_time_zone.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Sys/TimeZone",
    DivoomPixooCommandSystemSetTimeZoneRequest,
    DivoomPixooCommandSystemSetTimeZoneRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetTimeZoneRequestPayload {
    pub time_zone_value: String,
}

impl DivoomPixooCommandSystemSetTimeZoneRequestPayload {
    pub fn new(time_zone: String) -> DivoomPixooCommandSystemSetTimeZoneRequestPayload {
        DivoomPixooCommandSystemSetTimeZoneRequestPayload {
            time_zone_value: time_zone,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetTimeZoneResponse);
