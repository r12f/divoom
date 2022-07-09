#[doc = include_str!("./api_select_channel.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomChannelType;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/SetIndex",
    DivoomPixooCommandChannelSelectChannelRequest,
    DivoomPixooCommandChannelSelectChannelRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelSelectChannelRequestPayload {
    /// 0: Faces, 1: Cloud Channel, 2: Visualizer, 3: Custom
    pub select_index: i32,
}

impl DivoomPixooCommandChannelSelectChannelRequestPayload {
    pub fn new(
        channel_type: DivoomChannelType,
    ) -> DivoomPixooCommandChannelSelectChannelRequestPayload {
        DivoomPixooCommandChannelSelectChannelRequestPayload {
            select_index: match channel_type {
                DivoomChannelType::Clock => 0,
                DivoomChannelType::CloudChannel => 1,
                DivoomChannelType::Visualizer => 2,
                DivoomChannelType::CustomPage => 3,
                DivoomChannelType::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandChannelSelectChannelResponse);
