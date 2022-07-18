use tiny_skia::{BlendMode, FilterQuality, Pixmap, PixmapPaint, Transform};

pub struct DivoomAnimationFrameBuilder<'a> {
    pub frame: &'a mut Pixmap,
}

impl DivoomAnimationFrameBuilder<'_> {
    pub fn new(frame: &mut Pixmap) -> DivoomAnimationFrameBuilder {
        DivoomAnimationFrameBuilder { frame }
    }

    pub fn draw_frame_transform(
        mut self,
        frame: &Pixmap,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        rotation: Option<f32>,
        opacity: Option<f32>,
        blend: Option<BlendMode>,
    ) -> Self {
        let width_value = width.unwrap_or(frame.width());
        let height_value = height.unwrap_or(frame.height());
        let scale_x = if width_value == frame.width() {
            1.0
        } else {
            width_value as f32 / frame.width() as f32
        };
        let scale_y = if height_value == frame.height() {
            1.0
        } else {
            height_value as f32 / frame.height() as f32
        };
        let transform = Transform::from_scale(scale_x, scale_y)
            .post_concat(Transform::from_rotate(rotation.unwrap_or(0.0)));

        let mut paint = PixmapPaint::default();
        paint.opacity = opacity.unwrap_or(1.0);
        paint.blend_mode = blend.unwrap_or(BlendMode::default());
        paint.quality = FilterQuality::Bicubic;

        self.frame.draw_pixmap(
            x.unwrap_or(0),
            y.unwrap_or(0),
            frame.as_ref(),
            &paint,
            transform.clone(),
            None,
        );

        self
    }
}
