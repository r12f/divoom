use crate::animation::animation_builder::DivoomAnimationBuilder;
use tiny_skia::Pixmap;

// Draw functions
impl DivoomAnimationBuilder {
    pub fn draw_animation(self, animation: &DivoomAnimationBuilder, start_frame_index: usize) -> Self {
        self.draw_animation_transform(animation, 0, None, None, None, None, None)
    }

    pub fn draw_animation_at(
        self,
        start_frame_index: usize,
        animation: &DivoomAnimationBuilder,
        offset_x: Option<u32>,
        offset_y: Option<u32>,
    ) -> Self {
        self.draw_animation_transform(animation, start_frame_index, offset_x, offset_y, None, None, None)
    }

    pub fn draw_animation_transform(
        self,
        animation: &DivoomAnimationBuilder,
        start_frame_index: usize,
        offset_x: Option<u32>,
        offset_y: Option<u32>,
        scale_x: Option<f32>,
        scale_y: Option<f32>,
        rotation: Option<f32>,
    ) -> Self {
        self
    }
}
