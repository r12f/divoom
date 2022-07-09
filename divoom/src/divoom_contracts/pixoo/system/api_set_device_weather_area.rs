#[doc = include_str!("./api_set_device_weather_area.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Sys/LogAndLat",
    DivoomPixooCommandSystemSetWeatherAreaRequest,
    DivoomPixooCommandSystemSetWeatherAreaRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetWeatherAreaRequestPayload {
    pub longitude: String,
    pub latitude: String,
}

impl DivoomPixooCommandSystemSetWeatherAreaRequestPayload {
    pub fn new(
        longitude: String,
        latitude: String,
    ) -> DivoomPixooCommandSystemSetWeatherAreaRequestPayload {
        DivoomPixooCommandSystemSetWeatherAreaRequestPayload {
            longitude,
            latitude,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetWeatherAreaResponse);
