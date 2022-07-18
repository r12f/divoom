#[doc = include_str!("./api_send_image_animation_frame.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomImageAnimation;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Draw/SendHttpGif",
    DivoomPixooCommandAnimationSendImageAnimationFrameRequest,
    DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload {
    pub pic_num: i32,
    pub pic_width: i32,
    pub pic_offset: i32,
    pub pic_id: i32,
    pub pic_speed: i32,
    pub pic_data: String,
}

impl DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload {
    pub fn create_frames(
        id: i32,
        animation: DivoomImageAnimation,
    ) -> Vec<DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload> {
        animation
            .frames
            .into_iter()
            .map(
                |entry| DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload {
                    pic_num: animation.frame_count as i32,
                    pic_width: animation.size as i32,
                    pic_offset: entry.0 as i32,
                    pic_id: id,
                    pic_speed: animation.speed_in_ms,
                    pic_data: entry.1,
                },
            )
            .collect()
    }
}

// Response
define_pixoo_command_response_without_payload!(
    DivoomPixooCommandAnimationSendImageAnimationFrameResponse
);
