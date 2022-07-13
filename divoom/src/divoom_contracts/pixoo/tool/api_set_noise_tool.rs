#[doc = include_str!("./api_set_noise_tool.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomToolNoiseAction;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Tools/SetNoiseStatus",
    DivoomPixooCommandToolSetNoiseStatusRequest,
    DivoomPixooCommandToolSetNoiseStatusRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandToolSetNoiseStatusRequestPayload {
    /// 0: stop, 1: start
    pub noise_status: i32,
}

impl DivoomPixooCommandToolSetNoiseStatusRequestPayload {
    pub fn new(
        action: DivoomToolNoiseAction,
    ) -> DivoomPixooCommandToolSetNoiseStatusRequestPayload {
        DivoomPixooCommandToolSetNoiseStatusRequestPayload {
            noise_status: action.into(),
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandToolSetNoiseStatusResponse);
