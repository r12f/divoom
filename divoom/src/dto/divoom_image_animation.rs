use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

/// Definition of image animations.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomImageAnimation {
    /// Size of canvas. Only 16, 32, 64 are supported
    pub size: u32,

    /// The total number of frames in entire animation
    pub frame_count: usize,

    /// Animation play speed, in ms.
    pub speed_in_ms: i32,

    /// Offset to frame data map.
    /// We use this format, because Divoom support only updating a single frame at a time, so we don't need to use vector and update all frames.
    pub frames: BTreeMap<u32, DivoomImageAnimationFrameData>,
}

pub const DIVOOM_IMAGE_ANIMATION_ID_AUTO: i32 = -1;

/// The data of this frame.
///
/// The data format looks like below, which goes row by row and column by column, from left to right and top to down,
/// e.g.: (0, 0), (0, 1), (0, 2), ..., (1, 0), (1, 1), (1, 2), ..., (2, 0), (2, 1), (2, 2), ...
///
/// Hex data format looks like below:
/// ```text
/// RR GG BB RR GG BB RR GG BB ......
/// ```
pub type DivoomImageAnimationFrameData = Vec<u8>;

/// Type of the image animations file source.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomFileAnimationSourceType {
    LocalFile,
    LocalFolder,
    Url,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomFileAnimationSourceType, LocalFile: "file", LocalFolder: "folder", Url: "url");

impl DivoomImageAnimation {
    #[cfg(feature = "resource-format-gif")]
    pub fn save_gif<W: Write>(&self, image: W) -> anyhow::Result<()> {
        let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
        let mut encoder = gif::Encoder::new(image, self.size as u16, self.size as u16, color_map)?;
        encoder.set_repeat(gif::Repeat::Infinite)?;
        for (_, frame_data) in &self.frames {
            let mut frame = gif::Frame::from_rgb(self.size as u16, self.size as u16, &frame_data);
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }
}