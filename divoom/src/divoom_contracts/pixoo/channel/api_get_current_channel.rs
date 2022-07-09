#[doc = include_str!("./api_get_current_channel.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomChannelType;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Channel/GetIndex",
    DivoomPixooCommandChannelGetCurrentChannelRequest
);

// Response
define_pixoo_command_response!(
    DivoomPixooCommandChannelGetCurrentChannelResponse,
    DivoomPixooCommandChannelGetCurrentChannelResponsePayload,
    DivoomChannelType
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelGetCurrentChannelResponsePayload {
    pub select_index: i32,
}

impl DivoomPixooCommandChannelGetCurrentChannelResponsePayload {
    pub fn destructive_into(self) -> DivoomChannelType {
        match self.select_index {
            0 => DivoomChannelType::Clock,
            1 => DivoomChannelType::CloudChannel,
            2 => DivoomChannelType::Visualizer,
            3 => DivoomChannelType::CustomPage,
            _ => DivoomChannelType::Raw(self.select_index),
        }
    }
}
