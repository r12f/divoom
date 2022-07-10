#[doc = include_str!("./api_play_gif_file.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomFileAnimationSourceType;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Device/PlayTFGif",
    DivoomPixooCommandAnimationPlayGifRequest,
    DivoomPixooCommandAnimationPlayGifRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandAnimationPlayGifRequestPayload {
    /// 0: local file path, 1: local folder path, 2: net file path
    pub file_type: i32,
    pub file_name: String,
}

impl DivoomPixooCommandAnimationPlayGifRequestPayload {
    pub fn new(
        file_type: DivoomFileAnimationSourceType,
        file_name: String,
    ) -> DivoomPixooCommandAnimationPlayGifRequestPayload {
        DivoomPixooCommandAnimationPlayGifRequestPayload {
            file_type: match file_type {
                DivoomFileAnimationSourceType::LocalFile => 0,
                DivoomFileAnimationSourceType::LocalFolder => 1,
                DivoomFileAnimationSourceType::Url => 2,
                DivoomFileAnimationSourceType::Raw(n) => n,
            },
            file_name,
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandAnimationPlayGifResponse);
