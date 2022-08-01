use image::codecs::gif::{GifEncoder, Repeat};
use image::{Frame, RgbaImage};
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
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum DivoomFileAnimationSourceType {
    LocalFile,
    LocalFolder,
    Url,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomFileAnimationSourceType, LocalFile: "file", LocalFolder: "folder", Url: "url");

impl DivoomImageAnimation {
    pub fn save_gif<W: Write>(&self, image: W) -> anyhow::Result<()> {
        let mut encoder = GifEncoder::new(image);
        encoder.set_repeat(Repeat::Infinite)?;

        for frame_data in self.frames.values() {
            let frame_image = RgbaImage::from_fn(self.size, self.size, |x, y| {
                // In image crate pixel enumeration, x means width not height, and y means height not width. It is different from other image format.
                let pixel_start = 3 * (x + y * self.size) as usize;
                image::Rgba::from([
                    frame_data[pixel_start],
                    frame_data[pixel_start + 1],
                    frame_data[pixel_start + 2],
                    255,
                ])
            });
            encoder.encode_frame(Frame::new(frame_image))?
        }

        Ok(())
    }
}
