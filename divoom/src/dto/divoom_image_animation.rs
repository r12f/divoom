use super::divoom_dto_common::*;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// Definition of image animations.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DivoomImageAnimation {
    /// Id of the Animation to create/update. Returned by `get_next_animation_id()`.
    pub id: i32,

    /// Size of canvas. Only 16, 32, 64 are supported
    pub size: i32,

    /// The total number of frames in entire animation
    pub frame_count: i32,

    /// Animation play speed, in ms.
    pub speed_in_ms: i32,

    /// Offset to frame data map.
    /// We use this format, because Divoom support only updating a single frame at a time, so we don't need to use vector and update all frames.
    pub frames: BTreeMap<i32, DivoomImageAnimationFrameData>,
}

/// The data of this frame. It is a base64 encoded RGB data.
///
/// The decoded data format looks like below, which goes row by row and column by column, from left to right and top to down,
/// e.g.: (0, 0), (0, 1), (0, 2), ..., (1, 0), (1, 1), (1, 2), ..., (2, 0), (2, 1), (2, 2), ...
///
/// Hex data format looks like below:
/// ```text
/// RR GG BB RR GG BB RR GG BB ......
/// ```
pub type DivoomImageAnimationFrameData = String;

/// Type of the image animations file source.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomFileAnimationSourceType {
    LocalFile,
    LocalFolder,
    Url,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomFileAnimationSourceType, LocalFile: "file", LocalFolder: "folder", Url: "url");
