use crate::{DivoomAPIError, DivoomAPIResult};
use std::fs::File;
use tiny_skia::Pixmap;

/// Load resources into a series of `tiny_skia::Pixmap`, so we can use them to build the animations.
pub struct DivoomAnimationResourceLoader {}

impl DivoomAnimationResourceLoader {
    /// Load from local png file
    pub fn png(file_path: &str) -> DivoomAPIResult<Pixmap> {
        let frame = Pixmap::load_png(file_path)?;
        Ok(frame)
    }

    /// Load from local gif file
    #[cfg(feature = "resource-loader-gif")]
    pub fn gif(file_path: &str) -> DivoomAPIResult<Vec<Pixmap>> {
        let mut frames = vec![];
        let input = File::open(file_path)?;

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(input)?;
        while let Some(frame) = decoder.read_next_frame()? {
            let mut frame_pixmap = Pixmap::new(frame.width as u32, frame.height as u32).unwrap();
            assert_eq!(frame_pixmap.data().len(), frame.buffer.len());
            frame_pixmap.data_mut().copy_from_slice(&frame.buffer[0..]);
            frames.push(frame_pixmap);
        }

        Ok(frames)
    }
}

impl From<png::DecodingError> for DivoomAPIError {
    fn from(err: png::DecodingError) -> Self {
        DivoomAPIError::ResourceDecodeError(err.to_string())
    }
}

impl From<gif::DecodingError> for DivoomAPIError {
    fn from(err: gif::DecodingError) -> Self {
        DivoomAPIError::ResourceDecodeError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_resource_loader_can_load_png_file() {
        let frame =
            DivoomAnimationResourceLoader::png("test_data/animation_builder_tests/logo.png")
                .unwrap();

        let non_zero_bits_count = frame
            .data()
            .as_ref()
            .iter()
            .filter(|x| **x != 0u8)
            .count();
        assert_ne!(non_zero_bits_count, 0);
    }

    #[test]
    fn divoom_resource_loader_can_load_gif_file() {
        let frames =
            DivoomAnimationResourceLoader::gif("test_data/animation_builder_tests/logo.gif")
                .unwrap();
        assert_eq!(frames.len(), 1);

        let non_zero_bits_count = frames[0]
            .data()
            .as_ref()
            .iter()
            .filter(|x| **x != 0u8)
            .count();
        assert_ne!(non_zero_bits_count, 0);
    }
}
