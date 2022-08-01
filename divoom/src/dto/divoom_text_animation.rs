use super::divoom_dto_common::*;
use rgb::RGB8;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Font types
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum DivoomFontType {
    Scrollable,
    NotScrollable,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomFontType, Scrollable: "scroll", NotScrollable: "noscroll");

/// Font info
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomFontInfo {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub charset: String,
    pub font_type: DivoomFontType,
}

/// Text animation scrolling direction
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum DivoomTextAnimationScrollDirection {
    Left,
    Right,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomTextAnimationScrollDirection, Left: "left", Right: "right");

/// Text animation text alignment
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum DivoomTextAnimationAlign {
    Left,
    Middle,
    Right,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomTextAnimationAlign, Left: "left", Middle: "middle", Right: "right");

/// Text animation definition
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomTextAnimation {
    /// Text id to create/update. Must be <= 20.
    pub text_id: i32,

    /// Start position x.
    pub x: i32,

    /// Start position y.
    pub y: i32,

    /// Scroll direction.
    pub scroll_direction: DivoomTextAnimationScrollDirection,

    /// 0-7: font id in app. Divoom only has 8 fonts.
    pub font_index: i32,

    /// Text size. Must be >= 16 and <= 64.
    pub text_width: i32,

    /// Speed of each animation step (scroll) in milliseconds.
    pub speed_in_ms: i32,

    /// Text data
    pub text_string: String,

    /// Font color. E.g.: #FFFF00.
    #[serde(deserialize_with = "from_rgb_str")]
    #[serde(serialize_with = "to_rgb_str")]
    pub color: RGB8,

    /// Text align.
    pub align: DivoomTextAnimationAlign,
}

impl DivoomTextAnimation {
    pub fn default() -> DivoomTextAnimation {
        DivoomTextAnimation {
            text_id: 0,
            x: 0,
            y: 0,
            scroll_direction: DivoomTextAnimationScrollDirection::Left,
            font_index: 0,
            text_width: 16,
            speed_in_ms: 100,
            text_string: "".to_string(),
            color: rgb::RGB8::new(255, 255, 255),
            align: DivoomTextAnimationAlign::Left,
        }
    }
}
