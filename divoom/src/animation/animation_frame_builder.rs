use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use tiny_skia::{BlendMode, FilterQuality, Pixmap, PixmapPaint, Transform};

/// Fit mode when drawing the animation frame
#[derive(Debug, Copy, Clone)]
pub enum DivoomDrawFitMode {
    /// Draw the image in the center
    Center,

    /// Stretch both width and height of the image and doesn't maintain the radio
    Stretch,

    /// Maintaining the ratio and stretch the image to make the width the same as the canvas
    FitX,

    /// Maintaining the ratio and stretch the image to make the height the same as the canvas
    FitY,
}

impl_divoom_dto_enum_traits_without_raw!(DivoomDrawFitMode, Center: "center", Stretch: "stretch", FitX: "fitX", FitY: "fixY");

/// Builder of each animation frame
pub struct DivoomAnimationFrameBuilder<'a> {
    frame: &'a mut Pixmap,
}

impl DivoomAnimationFrameBuilder<'_> {
    /// Create an new builder on top of a mutable Pixmap reference.
    pub fn new(frame: &mut Pixmap) -> DivoomAnimationFrameBuilder {
        DivoomAnimationFrameBuilder { frame }
    }

    /// Return the internal Pixmap
    pub fn canvas(&self) -> &Pixmap {
        self.frame
    }

    /// Return the mutable reference of the internal Pixmap. This allow us to directly use tiny_skia APIs to do more complicated things.
    pub fn canvas_mut(&mut self) -> &mut Pixmap {
        self.frame
    }

    /// Draw an Pixmap onto the current canvas.
    pub fn draw_frame(self, frame: &Pixmap) -> Self {
        self.draw_frame_fit(
            frame,
            DivoomDrawFitMode::Center,
            0.0,
            1.0,
            BlendMode::default(),
        )
    }

    /// Draw an Pixmap onto the current canvas with fit options and other options, like rotation, opacity and blend mode.
    pub fn draw_frame_fit(
        self,
        frame: &Pixmap,
        fit: DivoomDrawFitMode,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        let (mut x, mut y, mut draw_width, mut draw_height) =
            (0i32, 0i32, frame.width(), frame.height());
        let frame_ratio: f32 = frame.width() as f32 / frame.height() as f32;

        match fit {
            DivoomDrawFitMode::Center => {
                x = (self.frame.width() as i32 - draw_width as i32) / 2;
                y = (self.frame.height() as i32 - draw_height as i32) / 2;
            }

            DivoomDrawFitMode::FitX => {
                draw_width = self.frame.width();
                draw_height = (draw_width as f32 / frame_ratio).round() as u32;
                y = ((self.frame.height() - draw_height) / 2) as i32;
            }

            DivoomDrawFitMode::FitY => {
                draw_height = self.frame.height();
                draw_width = (draw_height as f32 * frame_ratio).round() as u32;
                x = (self.frame.width() as i32 - draw_width as i32) / 2;
            }

            DivoomDrawFitMode::Stretch => {
                draw_width = self.frame.width();
                draw_height = self.frame.height();
            }
        }

        self.draw_frame_sized(
            frame,
            x,
            y,
            draw_width,
            draw_height,
            rotation,
            opacity,
            blend,
        )
    }

    /// Draw an Pixmap onto the current canvas with position and size specified.
    pub fn draw_frame_sized(
        self,
        frame: &Pixmap,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        let scale_x = if width == frame.width() {
            1.0
        } else {
            width as f32 / frame.width() as f32
        };
        let scale_y = if height == frame.height() {
            1.0
        } else {
            height as f32 / frame.height() as f32
        };

        self.draw_frame_scaled(frame, x, y, scale_x, scale_y, rotation, opacity, blend)
    }

    /// Draw an Pixmap onto the current canvas with position and scale on X and Y axis specified.
    pub fn draw_frame_scaled(
        self,
        frame: &Pixmap,
        x: i32,
        y: i32,
        scale_x: f32,
        scale_y: f32,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> Self {
        let transform = Transform::from_rotate_at(
            rotation,
            x as f32 + (frame.width() as f32 / 2.0),
            y as f32 + (frame.height() as f32 / 2.0),
        )
        .post_concat(Transform::from_scale(scale_x, scale_y));

        let paint = PixmapPaint {
            opacity,
            blend_mode: blend,
            quality: FilterQuality::Bicubic,
        };

        self.frame
            .draw_pixmap(x, y, frame.as_ref(), &paint, transform, None);

        self
    }
}
