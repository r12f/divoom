use crate::{DivoomAPIError, DivoomAPIResult};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use tiny_skia::Pixmap;
use image::io::Reader as ImageReader;

/// Load resources into a series of `tiny_skia::Pixmap`, so we can use them to build the animations.
pub struct DivoomAnimationResourceLoader {}

impl DivoomAnimationResourceLoader {
    pub fn from_image_file(file_path: &str) -> DivoomAPIResult<Pixmap> {
        let image_file = File::open(file_path)?;
        DivoomAnimationResourceLoader::from_image(image_file)
    }

    pub fn from_image<R: Read>(reader: R) -> DivoomAPIResult<Pixmap> {
        let mut buf_reader = BufReader::new(reader);

        let mut image_buffer: Vec<u8> = Vec::new();
        buf_reader.read_to_end(&mut image_buffer)?;

        DivoomAnimationResourceLoader::from_image_buf(&image_buffer)
    }

    pub fn from_image_buf(buf: &[u8]) -> DivoomAPIResult<Pixmap> {
        let image = ImageReader::new(Cursor::new(buf)).with_guessed_format()?.decode()?;
        let image_rgba8 = image.into_rgba8();

        let (width, height) = (image_rgba8.width(), image_rgba8.height());
        let mut frame = Pixmap::new(width as u32, height as u32).unwrap();
        frame.data_mut().copy_from_slice(&image_rgba8);

        Ok(frame)
    }

    /// Load gif resource from local file
    #[cfg(feature = "resource-format-gif")]
    pub fn from_gif_file(file_path: &str) -> DivoomAPIResult<Vec<Pixmap>> {
        let input = File::open(file_path)?;
        DivoomAnimationResourceLoader::from_gif(input)
    }

    /// Load gif resource from Read trait
    #[cfg(feature = "resource-format-gif")]
    pub fn from_gif<R: Read>(reader: R) -> DivoomAPIResult<Vec<Pixmap>> {
        let mut frames = vec![];

        let decoder = GifDecoder::new(reader)?;
        for frame in decoder.into_frames().collect_frames()? {
            let mut frame_pixmap = Pixmap::new(
                frame.buffer().width() as u32,
                frame.buffer().height() as u32,
            )
            .unwrap();
            assert_eq!(frame_pixmap.data().len(), frame.buffer().len());
            frame_pixmap
                .data_mut()
                .copy_from_slice(frame.into_buffer().as_raw());
            frames.push(frame_pixmap);
        }

        Ok(frames)
    }
}

#[cfg(feature = "resource-format-gif")]
impl From<image::ImageError> for DivoomAPIError {
    fn from(err: image::ImageError) -> Self {
        DivoomAPIError::ResourceDecodeError(err.to_string())
    }
}