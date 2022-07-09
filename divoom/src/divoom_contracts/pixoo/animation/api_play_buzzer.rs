#[doc = include_str!("./api_play_buzzer.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/PlayBuzzer",
    DivoomPixooCommandAnimationPlayBuzzerRequest,
    DivoomPixooCommandAnimationPlayBuzzerRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandAnimationPlayBuzzerRequestPayload {
    pub active_time_in_cycle: i32,
    pub off_time_in_cycle: i32,
    pub play_total_time: i32,
}

impl DivoomPixooCommandAnimationPlayBuzzerRequestPayload {
    pub fn new(
        play_total_time: i32,
        active_time_in_cycle: i32,
        off_time_in_cycle: i32,
    ) -> DivoomPixooCommandAnimationPlayBuzzerRequestPayload {
        DivoomPixooCommandAnimationPlayBuzzerRequestPayload {
            active_time_in_cycle,
            off_time_in_cycle,
            play_total_time,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandAnimationPlayBuzzerResponse);
