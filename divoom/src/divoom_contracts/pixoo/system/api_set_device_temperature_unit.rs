#[doc = include_str!("./api_set_device_temperature_unit.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceTemperatureUnit;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/SetDisTempMode",
    DivoomPixooCommandSystemSetTemperatureUnitRequest,
    DivoomPixooCommandSystemSetTemperatureUnitRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetTemperatureUnitRequestPayload {
    /// 0: Celsius, 1: Fahrenheit
    pub mode: i32,
}

impl DivoomPixooCommandSystemSetTemperatureUnitRequestPayload {
    pub fn new(
        unit: DivoomDeviceTemperatureUnit,
    ) -> DivoomPixooCommandSystemSetTemperatureUnitRequestPayload {
        DivoomPixooCommandSystemSetTemperatureUnitRequestPayload {
            mode: match unit {
                DivoomDeviceTemperatureUnit::Celsius => 0,
                DivoomDeviceTemperatureUnit::Fahrenheit => 1,
                DivoomDeviceTemperatureUnit::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetTemperatureUnitResponse);
