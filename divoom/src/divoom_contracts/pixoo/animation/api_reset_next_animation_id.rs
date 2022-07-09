#[doc = include_str!("./api_reset_next_animation_id.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Draw/ResetHttpGifId",
    DivoomPixooCommandAnimationResetNextAnimationIdRequest
);

// Response
define_pixoo_command_response_without_payload!(
    DivoomPixooCommandAnimationResetNextAnimationIdResponse
);
