#[doc = include_str!("./api_clear_all_text_area.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Draw/ClearHttpText",
    DivoomPixooCommandAnimationClearAllTextAreaRequest
);

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandAnimationClearAllTextAreaResponse);
