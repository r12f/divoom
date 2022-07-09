#[doc = include_str!("./api_set_scoreboard_tool.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Tools/SetScoreBoard",
    DivoomPixooCommandToolSetScoreboardRequest,
    DivoomPixooCommandToolSetScoreboardRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandToolSetScoreboardRequestPayload {
    pub blue_score: i32,
    pub red_score: i32,
}

impl DivoomPixooCommandToolSetScoreboardRequestPayload {
    pub fn new(
        blue_score: i32,
        red_score: i32,
    ) -> DivoomPixooCommandToolSetScoreboardRequestPayload {
        DivoomPixooCommandToolSetScoreboardRequestPayload {
            blue_score,
            red_score,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandToolSetScoreboardResponse);
