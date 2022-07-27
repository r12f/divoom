use crate::{DivoomAPIError, DivoomAPIResult};
use std::fs::File;
use std::io::{BufReader, Read};
use tiny_skia::Pixmap;

/// Load resources into a series of `tiny_skia::Pixmap`, so we can use them to build the animations.
pub struct DivoomAnimationResourceLoader {}

impl DivoomAnimationResourceLoader {
    /// Load png resource from local file
    pub fn png_file(file_path: &str) -> DivoomAPIResult<Pixmap> {
        let frame = Pixmap::load_png(file_path)?;
        Ok(frame)
    }

    /// Load png resource from a memory buffer
    pub fn png_buf(buf: &[u8]) -> DivoomAPIResult<Pixmap> {
        let frame = Pixmap::decode_png(buf)?;
        Ok(frame)
    }

    /// Load png resource from Read trait
    pub fn png<R: Read>(reader: R) -> DivoomAPIResult<Pixmap> {
        let mut buffer = Vec::new();
        let mut buf_reader = BufReader::new(reader);
        buf_reader.read_to_end(&mut buffer)?;
        DivoomAnimationResourceLoader::png_buf(&buffer)
    }

    /// Load jpeg resource from local file
    #[cfg(feature = "resource-format-jpeg")]
    pub fn jpeg_file(file_path: &str) -> DivoomAPIResult<Pixmap> {
        let file = File::open(file_path)?;
        DivoomAnimationResourceLoader::jpeg(file)
    }

    /// Load jpeg resource from Read trait
    #[cfg(feature = "resource-format-jpeg")]
    pub fn jpeg<R: Read>(reader: R) -> DivoomAPIResult<Pixmap> {
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(reader));
        let pixels = decoder.decode()?;

        let metadata = decoder.info().unwrap();
        let mut frame = Pixmap::new(metadata.width as u32, metadata.height as u32).unwrap();

        match metadata.pixel_format {
            jpeg_decoder::PixelFormat::L8 => {
                let mut frame_data_index = 0;
                let frame_data = frame.data_mut();
                for gray in pixels {
                    frame_data[frame_data_index] = gray;
                    frame_data[frame_data_index + 1] = gray;
                    frame_data[frame_data_index + 2] = gray;
                    frame_data[frame_data_index + 3] = 0xFF;
                    frame_data_index += 4;
                }
            }

            // From search, JPEG doesn't look like supports 16 bits image at all.
            jpeg_decoder::PixelFormat::L16 => Err(DivoomAPIError::ResourceDecodeError(
                "Unsupported file format: JPEG with 16-bits grayscale.".to_string(),
            ))?,

            jpeg_decoder::PixelFormat::RGB24 => {
                let mut frame_data_index = 0;
                let frame_data = frame.data_mut();
                for rgb in pixels.chunks(3) {
                    frame_data[frame_data_index] = rgb[0];
                    frame_data[frame_data_index + 1] = rgb[1];
                    frame_data[frame_data_index + 2] = rgb[2];
                    frame_data[frame_data_index + 3] = 0xFF;
                    frame_data_index += 4;
                }
            }

            jpeg_decoder::PixelFormat::CMYK32 => {
                let mut frame_data_index = 0;
                let frame_data = frame.data_mut();
                for pixel in pixels.chunks(4) {
                    // Reversed RGB to CMYK formula from https://www.codeproject.com/Articles/4488/XCmyk-CMYK-to-RGB-Calculator-with-source-code.
                    // - Black   = minimum(1-Red,1-Green,1-Blue)
                    // - Cyan    = (1-Red-Black)/(1-Black)
                    // - Magenta = (1-Green-Black)/(1-Black)
                    // - Yellow  = (1-Blue-Black)/(1-Black)
                    let (c, m, y, k) = (
                        pixel[0] as f32 / 255.0,
                        pixel[1] as f32 / 255.0,
                        pixel[2] as f32 / 255.0,
                        pixel[3] as f32 / 255.0,
                    );
                    frame_data[frame_data_index] = ((1.0 - c) * (1.0 - k) * 255.0) as u8;
                    frame_data[frame_data_index + 1] = ((1.0 - m) * (1.0 - k) * 255.0) as u8;
                    frame_data[frame_data_index + 2] = ((1.0 - y) * (1.0 - k) * 255.0) as u8;
                    frame_data[frame_data_index + 3] = 0xFF;
                    frame_data_index += 4;
                }
            }
        };

        Ok(frame)
    }

    /// Load gif resource from local file
    #[cfg(feature = "resource-format-gif")]
    pub fn gif_file(file_path: &str) -> DivoomAPIResult<Vec<Pixmap>> {
        let input = File::open(file_path)?;
        DivoomAnimationResourceLoader::gif(input)
    }

    /// Load gif resource from Read trait
    #[cfg(feature = "resource-format-gif")]
    pub fn gif<R: Read>(reader: R) -> DivoomAPIResult<Vec<Pixmap>> {
        let mut frames = vec![];

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(reader)?;
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

#[cfg(feature = "resource-format-gif")]
impl From<gif::DecodingError> for DivoomAPIError {
    fn from(err: gif::DecodingError) -> Self {
        DivoomAPIError::ResourceDecodeError(err.to_string())
    }
}

#[cfg(feature = "resource-format-jpeg")]
impl From<jpeg_decoder::Error> for DivoomAPIError {
    fn from(err: jpeg_decoder::Error) -> Self {
        DivoomAPIError::ResourceDecodeError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_resource_loader_can_load_png_file() {
        let frame =
            DivoomAnimationResourceLoader::png_file("test_data/animation_builder_tests/logo.png")
                .unwrap();

        let non_zero_bits_count = frame.data().as_ref().iter().filter(|x| **x != 0u8).count();
        assert_ne!(non_zero_bits_count, 0);
    }

    #[test]
    fn divoom_resource_loader_can_load_jpeg_grayscale_file() {
        let frame = DivoomAnimationResourceLoader::jpeg_file(
            "test_data/animation_builder_tests/logo_grayscale.jpg",
        )
        .unwrap();

        let non_zero_bits_count = frame.data().as_ref().iter().filter(|x| **x != 0u8).count();
        assert_ne!(non_zero_bits_count, 0);
    }

    #[test]
    fn divoom_resource_loader_can_load_jpeg_rgb_file() {
        let frame = DivoomAnimationResourceLoader::jpeg_file(
            "test_data/animation_builder_tests/logo_rgb.jpg",
        )
        .unwrap();

        let non_zero_bits_count = frame.data().as_ref().iter().filter(|x| **x != 0u8).count();
        assert_ne!(non_zero_bits_count, 0);
    }

    #[test]
    fn divoom_resource_loader_can_load_jpeg_cmyk_file() {
        let frame = DivoomAnimationResourceLoader::jpeg_file(
            "test_data/animation_builder_tests/logo_cmyk.jpg",
        )
        .unwrap();

        let non_zero_bits_count = frame.data().as_ref().iter().filter(|x| **x != 0u8).count();
        assert_ne!(non_zero_bits_count, 0);
    }

    #[test]
    fn divoom_resource_loader_can_load_gif_file() {
        let frames =
            DivoomAnimationResourceLoader::gif_file("test_data/animation_builder_tests/logo.gif")
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
