use crate::animation::animation_frame_builder::DivoomAnimationFrameBuilder;
use crate::animation::DivoomDrawFitMode;
use crate::dto::*;
use std::collections::BTreeMap;
use std::iter::once;
use std::time::Duration;
use tiny_skia::{BlendMode, Pixmap};

/// ## Animation builder
/// This class holds a series of tiny_skia::Pixmap inside with the same size to provide an interface for building animations.
pub struct DivoomAnimationBuilder {
    canvas_size: u32,
    speed: Duration,
    frames: Vec<Pixmap>,
}

// Ctor and basic functions
impl DivoomAnimationBuilder {
    /// Create a new animation builder with canvas size and speed.
    ///
    /// For Divoom devices, only 16, 32 and 64 pixels canvas are allowed. When other value is specified, we will return failure.
    pub fn new(canvas_size: u32, speed: Duration) -> DivoomAPIResult<DivoomAnimationBuilder> {
        if canvas_size != 16 && canvas_size != 32 && canvas_size != 64 {
            return Err(DivoomAPIError::ParameterError(format!(
                "Invalid canvas size: {}. Only 16, 32 and 64 are supported.",
                canvas_size
            )));
        }

        Ok(DivoomAnimationBuilder {
            canvas_size,
            speed,
            frames: vec![],
        })
    }

    /// Return canvas size
    pub fn canvas_size(&self) -> u32 {
        self.canvas_size
    }

    /// Return a builder for the specified frame.
    ///
    /// If the frame is not created or any previous frame is not created for the specified index, we will create all of them and return the builder
    /// for the last frame.
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
    /// Draw a series of frames to the canvas.
    /// - Frames doesn't need to be having the same size, as they will be drawn separately one by one.
    /// - If we have more frames than what we have internally, the new frames will be automatically created.
    pub fn draw_frames(mut self, frames: &[Pixmap], start_frame_index: usize) -> Self {
        for (frame_offset, frame) in frames.iter().enumerate() {
            let target_frame_index = start_frame_index + frame_offset;
            let frame_builder = self.build_frame(target_frame_index);
            frame_builder.draw_frame(frame);
        }

        self
    }

    /// Draw a series of frames to the canvas with `DivoomDrawFitMode` option and other options.
    /// With `DivoomDrawFitMode` option, we provided a few ways to calculate the position and size of each frame to simplify the usage.
    pub fn draw_frames_fit(
        mut self,
        frames: &[Pixmap],
        start_frame_index: usize,
        fit: DivoomDrawFitMode,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        for (frame_offset, frame) in frames.iter().enumerate() {
            let target_frame_index = start_frame_index + frame_offset;
            let frame_builder = self.build_frame(target_frame_index);
            frame_builder.draw_frame_fit(frame, fit, rotation, opacity, blend);
        }

        self
    }

    /// Draw a series of frames to the canvas with specified position and size.
    pub fn draw_frames_sized(
        mut self,
        frames: &[Pixmap],
        start_frame_index: usize,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        for (frame_offset, frame) in frames.iter().enumerate() {
            let target_frame_index = start_frame_index + frame_offset;
            let frame_builder = self.build_frame(target_frame_index);
            frame_builder.draw_frame_sized(frame, x, y, width, height, rotation, opacity, blend);
        }

        self
    }

    /// Draw a series of frames to the canvas with specified position and scale on X and Y axis.
    pub fn draw_frames_scaled(
        mut self,
        frames: &[Pixmap],
        start_frame_index: usize,
        x: i32,
        y: i32,
        scale_x: f32,
        scale_y: f32,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        for (frame_offset, frame) in frames.iter().enumerate() {
            let target_frame_index = start_frame_index + frame_offset;
            let frame_builder = self.build_frame(target_frame_index);
            frame_builder
                .draw_frame_scaled(frame, x, y, scale_x, scale_y, rotation, opacity, blend);
        }

        self
    }
}

// Export function
impl DivoomAnimationBuilder {
    /// Create the final animation that is used for being sent to the device, and one animation builder can be reused to create multiple animations.
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
        frame
            .pixels()
            .iter()
            .flat_map(|p| {
                once((p.red() as u32 * p.alpha() as u32 / 255) as u8)
                    .chain(once((p.green() as u32 * p.alpha() as u32 / 255) as u8))
                    .chain(once((p.blue() as u32 * p.alpha() as u32 / 255) as u8))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::animation::*;
    use crate::{test_utils, DivoomAPIError};
    use std::time::Duration;
    use tiny_skia::{BlendMode, Pixmap};

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
                DivoomAPIError::ParameterError(_) => (),
                _ => panic!("Incorrect error code!"),
            },
        }
    }

    #[test]
    fn divoom_animation_builder_can_get_canvas() {
        let mut builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        let frame_builder = builder.build_frame(0);
        let frame = frame_builder.canvas();
        assert_eq!(frame.width(), 64);
    }

    #[test]
    #[allow(unused_mut, unused_assignments)]
    fn divoom_animation_builder_can_get_canvas_mut() {
        let mut builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        let mut frame_builder = builder.build_frame(0);
        let mut frame = frame_builder.canvas_mut();
        assert_eq!(frame.width(), 64);

        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        frame_builder = frame_builder.draw_frame(&frames[0]);
    }

    fn divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
        resource_file_path: &str,
        reference_file_path: &str,
    ) {
        let frame = DivoomAnimationResourceLoader::from_image_file(resource_file_path).unwrap();
        divoom_animation_builder_can_build_single_frame_animation_from_resource(&frame, reference_file_path);
    }

    fn divoom_animation_builder_can_build_single_frame_animation_from_resource(
        frame: &Pixmap,
        reference_file_path: &str,
    ) {
        let mut builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        builder.build_frame(0).draw_frame_fit(&frame, DivoomDrawFitMode::Stretch, 0.0, 1.0, BlendMode::default());

        let animation = builder.build();
        test_utils::assert_animation_equal_with_baseline(&animation, reference_file_path);
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation_from_png_file() {
        divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
            "test_data/animation_builder_tests/input/logo.png",
            "test_data/animation_builder_tests/expected_single_frame_animation_from_png_file.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation_from_jpeg_grayscale_file() {
        divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
            "test_data/animation_builder_tests/input/logo_grayscale.jpg",
            "test_data/animation_builder_tests/expected_single_frame_animation_from_jpeg_grayscale_file.gif"
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation_from_jpeg_rgb_file() {
        divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
            "test_data/animation_builder_tests/input/logo_rgb.jpg",
            "test_data/animation_builder_tests/expected_single_frame_animation_from_jpeg_rgb_file.gif"
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation_from_jpeg_cmyk_file() {
        divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
            "test_data/animation_builder_tests/input/logo_cmyk.jpg",
            "test_data/animation_builder_tests/expected_single_frame_animation_from_jpeg_cmyk_file.gif"
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation_from_bmp_file() {
        divoom_animation_builder_can_build_single_frame_animation_from_resource_file(
            "test_data/animation_builder_tests/input/logo.bmp",
            "test_data/animation_builder_tests/expected_single_frame_animation_from_bmp_file.gif"
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_single_frame_animation() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        divoom_animation_builder_can_build_single_frame_animation_from_resource(
            &frames[0],
            "test_data/animation_builder_tests/expected_single_frame_animation_from_gif_file.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_animation_with_fit() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(32, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_fit(
                &frames,
                0,
                DivoomDrawFitMode::Center,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .draw_frames_fit(
                &frames,
                1,
                DivoomDrawFitMode::Stretch,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .draw_frames_fit(
                &frames,
                2,
                DivoomDrawFitMode::FitX,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .draw_frames_fit(
                &frames,
                3,
                DivoomDrawFitMode::FitY,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .build();

        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_animation_with_fit.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_animation_with_rotation() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(32, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_fit(
                &frames,
                0,
                DivoomDrawFitMode::Center,
                45.0,
                1.0,
                BlendMode::default(),
            )
            .build();

        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_animation_with_rotation.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_animation_with_opacity() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(32, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_fit(
                &frames,
                0,
                DivoomDrawFitMode::Center,
                0.0,
                0.5,
                BlendMode::default(),
            )
            .build();

        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_animation_with_opacity.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_animation_with_sized() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(32, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_sized(&frames, 0, 6, 6, 28, 18, 0.0, 1.0, BlendMode::default())
            .build();

        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_animation_with_sized.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_animation_with_scaled() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-0.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 1);

        let builder = DivoomAnimationBuilder::new(32, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_scaled(&frames, 0, 6, 6, 1.2, 0.8, 0.0, 1.0, BlendMode::default())
            .build();

        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_animation_with_scaled.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_build_multi_frame_animation() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo-16-rotate-4-frames.gif",
        )
        .unwrap();
        assert_eq!(frames.len(), 4);

        let builder = DivoomAnimationBuilder::new(16, Duration::from_millis(100)).unwrap();
        let animation = builder.draw_frames(&frames, 0).build();
        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_multi_frames_animation.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_crop_animation() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo.gif",
        )
            .unwrap();

        let builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        let animation = builder.draw_frames(&frames, 0).build();
        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_cropped_animation.gif",
        );
    }

    #[test]
    fn divoom_animation_builder_can_downscale_animation() {
        let frames = DivoomAnimationResourceLoader::from_gif_file(
            "test_data/animation_builder_tests/input/logo.gif",
        )
        .unwrap();

        let builder = DivoomAnimationBuilder::new(64, Duration::from_millis(100)).unwrap();
        let animation = builder
            .draw_frames_fit(
                &frames,
                0,
                DivoomDrawFitMode::Stretch,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .build();
        test_utils::assert_animation_equal_with_baseline(
            &animation,
            "test_data/animation_builder_tests/expected_downscaled_animation.gif",
        );
    }
}
