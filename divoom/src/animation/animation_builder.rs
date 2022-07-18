use crate::animation::animation_builder_error::{
    DivoomAnimationBuilderError, DivoomAnimationBuilderResult,
};
use crate::animation::animation_frame_builder::DivoomAnimationFrameBuilder;
use crate::{DivoomImageAnimation, DivoomImageAnimationFrameData};
use tiny_skia::{BlendMode, Pixmap};

pub struct DivoomAnimationBuilder {
    width: u32,
    height: u32,
    speed_in_ms: i32,

    pub frames: Vec<Pixmap>,
}

// Ctor and basic functions
impl DivoomAnimationBuilder {
    pub fn new(
        canvas_size: u32,
        speed_in_ms: i32,
    ) -> DivoomAnimationBuilderResult<DivoomAnimationBuilder> {
        if canvas_size != 16 && canvas_size != 32 && canvas_size != 64 {
            return Err(DivoomAnimationBuilderError::UnsupportedCanvasSize);
        }

        Ok(DivoomAnimationBuilder {
            width: canvas_size,
            height: canvas_size,
            speed_in_ms,
            frames: vec![],
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn build_frame(&mut self, index: usize) -> DivoomAnimationFrameBuilder {
        while index + 1 < self.frames.len() {
            let pixmap = Pixmap::new(self.width, self.height).unwrap();
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
        let animation = DivoomImageAnimation {
            id: 0,
            size: self.width,
            frame_count: self.frames.len(),
            speed_in_ms: self.speed_in_ms,
            frames: Default::default(),
        };

        animation
    }
}
