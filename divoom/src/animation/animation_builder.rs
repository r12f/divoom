use std::collections::BTreeMap;
use std::iter::once;
use std::time::Duration;
use crate::dto::*;
use crate::animation::animation_builder_error::*;
use crate::animation::animation_frame_builder::DivoomAnimationFrameBuilder;
use tiny_skia::{BlendMode, Pixmap};

pub struct DivoomAnimationBuilder {
    canvas_size: u32,
    speed: Duration,

    pub frames: Vec<Pixmap>,
}

// Ctor and basic functions
impl DivoomAnimationBuilder {
    pub fn new(
        canvas_size: u32,
        speed: Duration,
    ) -> DivoomAnimationBuilderResult<DivoomAnimationBuilder> {
        if canvas_size != 16 && canvas_size != 32 && canvas_size != 64 {
            return Err(DivoomAnimationBuilderError::UnsupportedCanvasSize);
        }

        Ok(DivoomAnimationBuilder {
            canvas_size,
            speed,
            frames: vec![],
        })
    }

    pub fn canvas_size(&self) -> u32 {
        self.canvas_size
    }

    pub fn build_frame(&mut self, index: usize) -> DivoomAnimationFrameBuilder {
        while index + 1 > self.frames.len() {
            let pixmap = Pixmap::new(self.canvas_size, self.canvas_size).unwrap();
            self.frames.push(pixmap);
        }

        DivoomAnimationFrameBuilder::new(&mut self.frames[index])
    }
}

// Draw functions
impl DivoomAnimationBuilder {
    pub fn draw_frames(self, frames: &Vec<Pixmap>, start_frame_index: usize) -> Self {
        self.draw_frames_transform(
            frames,
            start_frame_index,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn draw_frames_transform(
        mut self,
        frames: &Vec<Pixmap>,
        start_frame_index: usize,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        rotation: Option<f32>,
        opacity: Option<f32>,
        blend: Option<BlendMode>,
    ) -> Self {
        for frame_offset in 0..frames.len() {
            let target_frame_index = start_frame_index + frame_offset;
            let frame_builder = self.build_frame(target_frame_index);
            frame_builder.draw_frame_transform(
                &frames[frame_offset],
                x,
                y,
                width,
                height,
                rotation,
                opacity,
                blend,
            );
        }

        self
    }
}

// Export function
impl DivoomAnimationBuilder {
    pub fn build(&self) -> DivoomImageAnimation {
        let mut animation = DivoomImageAnimation {
            size: self.canvas_size,
            frame_count: self.frames.len(),
            speed_in_ms: self.speed.as_millis() as i32,
            frames: BTreeMap::new(),
        };

        for (index, frame) in self.frames.iter().enumerate() {
            let frame_buffer = DivoomAnimationBuilder::build_divoom_animation_frame_buffer(frame);
            animation.frames.entry(index as u32).or_insert(frame_buffer);
        }

        animation
    }

    fn build_divoom_animation_frame_buffer(frame: &Pixmap) -> DivoomImageAnimationFrameData {
        let divoom_frame_buffer: Vec<u8> = frame.pixels().iter().flat_map(|p| once(p.red()).chain(once(p.green())).chain(once(p.blue()))).collect();
        let encoded_buffer = base64::encode(divoom_frame_buffer);
        encoded_buffer
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::animation::*;
    use crate::test_utils;

    #[test]
    fn divoom_animation_builder_can_be_created() {
        let builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        assert_eq!(builder.canvas_size(), 64);
        assert_eq!(builder.speed, Duration::from_millis(100));
    }

    #[test]
    fn divoom_animation_builder_should_fail_with_incorrect_canvas_size() {
        let result = DivoomAnimationBuilder::new(15, Duration::from_millis(100));
        match result {
            Ok(_) => panic!("Canvas size is incorrect and we shall not create builder here."),
            Err(e) => match e {
                DivoomAnimationBuilderError::UnsupportedCanvasSize => (),
                _ => panic!("Incorrect error code!"),
            },
        }
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation() {
        let frames =
            DivoomAnimationResourceLoader::gif("test_data/animation_builder_tests/logo-16-0.gif")
                .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(16, Duration::from_millis(100)).unwrap();
        let animation = builder.draw_frames(&frames, 0).build();
        test_utils::assert_object_equal_with_baseline(&animation, "test_data/animation_builder_tests/single_frame_animation_expected.json");
    }

    #[test]
    fn divoom_animation_builder_can_build_multi_frame_animation() {
        let frames =
            DivoomAnimationResourceLoader::gif("test_data/animation_builder_tests/logo-16-rotate-4-frames.gif")
                .unwrap();
        assert_eq!(frames.len(), 4);

        let builder = DivoomAnimationBuilder::new(16, Duration::from_millis(100)).unwrap();
        let animation = builder.draw_frames(&frames, 0).build();
        test_utils::assert_object_equal_with_baseline(&animation, "test_data/animation_builder_tests/multi_frames_animation_expected.json");
    }
}
