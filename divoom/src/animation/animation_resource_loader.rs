use crate::animation::animation_builder_error::DivoomAnimationBuilderResult;
use std::fs::File;
use tiny_skia::Pixmap;

pub struct DivoomAnimationResourceLoader {}

impl DivoomAnimationResourceLoader {
    pub fn gif(file_path: &str) -> DivoomAnimationBuilderResult<Vec<Pixmap>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_resource_loader_can_load_gif_file() {
        let frames =
            DivoomAnimationResourceLoader::gif("test_data/animation_builder_tests/logo.gif")
                .unwrap();
        assert_eq!(frames.len(), 1);

        let non_zero_bits: Vec<&u8> = frames[0]
            .data()
            .as_ref()
            .into_iter()
            .filter(|x| **x != 0u8)
            .collect();
        assert_ne!(non_zero_bits.len(), 0);
    }
}
