#[doc = include_str!("./api_set_device_screen_power_state.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomDeviceScreenPowerState;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/OnOffScreen",
    DivoomPixooCommandSystemSetScreenPowerStateRequest,
    DivoomPixooCommandSystemSetScreenPowerStateRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemSetScreenPowerStateRequestPayload {
    pub on_off: i32,
}

impl DivoomPixooCommandSystemSetScreenPowerStateRequestPayload {
    pub fn new(
        power_state: DivoomDeviceScreenPowerState,
    ) -> DivoomPixooCommandSystemSetScreenPowerStateRequestPayload {
        DivoomPixooCommandSystemSetScreenPowerStateRequestPayload {
            on_off: match power_state {
                DivoomDeviceScreenPowerState::Off => 0,
                DivoomDeviceScreenPowerState::On => 1,
                DivoomDeviceScreenPowerState::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandSystemSetScreenPowerStateResponse);
