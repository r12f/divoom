#[doc = include_str!("./api_select_cloud_channel.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomCloudChannelType;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Channel/CloudIndex",
    DivoomPixooCommandChannelSelectCloudChannelRequest,
    DivoomPixooCommandChannelSelectCloudChannelRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandChannelSelectCloudChannelRequestPayload {
    // 0: Recommend gallery, 1: Favourite, 2: Subscribe artist
    pub index: i32,
}

impl DivoomPixooCommandChannelSelectCloudChannelRequestPayload {
    pub fn new(
        channel_type: DivoomCloudChannelType,
    ) -> DivoomPixooCommandChannelSelectCloudChannelRequestPayload {
        DivoomPixooCommandChannelSelectCloudChannelRequestPayload {
            index: match channel_type {
                DivoomCloudChannelType::Gallery => 0,
                DivoomCloudChannelType::Fav => 1,
                DivoomCloudChannelType::Artist => 2,
                DivoomCloudChannelType::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandChannelSelectCloudChannelResponse);
