#[doc = include_str!("./api_select_custom_page.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/SetCustomPageIndex",
    DivoomPixooCommandChannelSelectCustomPageRequest,
    DivoomPixooCommandChannelSelectCustomPageRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelSelectCustomPageRequestPayload {
    pub custom_page_index: i32,
}

impl DivoomPixooCommandChannelSelectCustomPageRequestPayload {
    pub fn new(custom_page_index: i32) -> DivoomPixooCommandChannelSelectCustomPageRequestPayload {
        DivoomPixooCommandChannelSelectCustomPageRequestPayload { custom_page_index }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandChannelSelectCustomPageResponse);
