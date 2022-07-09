#[doc = include_str!("./api_select_visualizer.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/SetEqPosition",
    DivoomPixooCommandChannelSelectVisualizerRequest,
    DivoomPixooCommandChannelSelectVisualizerRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelSelectVisualizerRequestPayload {
    pub eq_position: i32,
}

impl DivoomPixooCommandChannelSelectVisualizerRequestPayload {
    pub fn new(eq_position: i32) -> DivoomPixooCommandChannelSelectVisualizerRequestPayload {
        DivoomPixooCommandChannelSelectVisualizerRequestPayload { eq_position }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandChannelSelectVisualizerResponse);
