#[doc = include_str!("./api_send_text_animation.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::{DivoomTextAnimation, DivoomTextAnimationAlign, DivoomTextAnimationScrollDirection};
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Draw/SendHttpText",
    DivoomPixooCommandAnimationSendTextAnimationRequest,
    DivoomPixooCommandAnimationSendTextAnimationRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DivoomPixooCommandAnimationSendTextAnimationRequestPayload {
    #[serde(rename = "TextId")]
    pub text_id: i32,

    pub x: i32,
    pub y: i32,

    /// 0: Scroll left, 1: Scroll right
    pub dir: i32,

    /// 0-7: font id in app. Divoom only has 8 fonts.
    pub font: i32,

    #[serde(rename = "TextWidth")]
    pub text_width: i32,

    /// scroll speed of each step in ms.
    pub speed: i32,

    #[serde(rename = "TextString")]
    pub text_string: String,

    /// Font color. E.g.: #FFFF00.
    pub color: String,

    /// 1: left, 2: middle, 3: right
    pub align: i32,
}

impl DivoomPixooCommandAnimationSendTextAnimationRequestPayload {
    pub fn new(
        text: DivoomTextAnimation,
    ) -> DivoomPixooCommandAnimationSendTextAnimationRequestPayload {
        DivoomPixooCommandAnimationSendTextAnimationRequestPayload {
            text_id: text.text_id,
            x: text.x,
            y: text.y,
            dir: match text.scroll_direction {
                DivoomTextAnimationScrollDirection::Left => 0,
                DivoomTextAnimationScrollDirection::Right => 1,
                DivoomTextAnimationScrollDirection::Raw(n) => n,
            },
            font: text.font_index,
            text_width: text.text_width,
            speed: text.speed_in_ms,
            text_string: text.text_string,
            color: format!(
                "#{:02X}{:02X}{:02X}",
                text.color.r, text.color.g, text.color.b
            ),
            align: match text.align {
                DivoomTextAnimationAlign::Left => 1,
                DivoomTextAnimationAlign::Middle => 2,
                DivoomTextAnimationAlign::Right => 3,
                DivoomTextAnimationAlign::Raw(n) => n,
            },
        }
    }
}

// Response
define_pixoo_command_response_without_payload!(
    DivoomPixooCommandAnimationSendTextAnimationResponse
);
