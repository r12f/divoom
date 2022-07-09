#[doc = include_str!("./api_get_next_animation_id.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Draw/GetHttpGifId",
    DivoomPixooCommandAnimationGetNextAnimationIdRequest
);

// Response
define_pixoo_command_response!(
    DivoomPixooCommandAnimationGetNextAnimationIdResponse,
    DivoomPixooCommandAnimationGetNextAnimationIdResponsePayload,
    i32
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandAnimationGetNextAnimationIdResponsePayload {
    pub pic_id: i32,
}

impl DivoomPixooCommandAnimationGetNextAnimationIdResponsePayload {
    pub fn destructive_into(self) -> i32 {
        self.pic_id
    }
}
