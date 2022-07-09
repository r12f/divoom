use rgb::RGB8;
use std::str::FromStr;

/// Font types
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomFontType {
    Scrollable,
    NotScrollable,
    Raw(i32),
}

impl FromStr for DivoomFontType {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "scroll" => Ok(DivoomFontType::Scrollable),
            "noscroll" => Ok(DivoomFontType::NotScrollable),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomFontType: {}", x))?;
                Ok(DivoomFontType::Raw(parsed))
            }
        }
    }
}

/// Font info
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomFontInfo {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub charset: String,
    pub font_type: DivoomFontType,
}

/// Text animation scrolling direction
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomTextAnimationScrollDirection {
    Left,
    Right,
}

impl FromStr for DivoomTextAnimationScrollDirection {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "left" => Ok(DivoomTextAnimationScrollDirection::Left),
            "right" => Ok(DivoomTextAnimationScrollDirection::Right),
            _ => Err(format!(
                "Invalid value for DivoomTextAnimationScrollDirection: {}",
                v
            )),
        }
    }
}

/// Text animation text alignment
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomTextAnimationAlign {
    Left,
    Middle,
    Right,
}

impl FromStr for DivoomTextAnimationAlign {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "left" => Ok(DivoomTextAnimationAlign::Left),
            "middle" => Ok(DivoomTextAnimationAlign::Middle),
            "right" => Ok(DivoomTextAnimationAlign::Right),
            _ => Err(format!("Invalid value for DivoomTextAnimationAlign: {}", v)),
        }
    }
}

/// Text animation definition
#[derive(Debug, PartialOrd, PartialEq)]
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
