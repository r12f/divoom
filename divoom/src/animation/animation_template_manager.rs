use crate::animation::animation_template::{
    DivoomAnimationTemplate, DivoomAnimationTemplateConfig,
};
use crate::{DivoomAnimationBuilder, DivoomAPIError, DivoomAPIResult, DivoomDrawFitMode, DivoomImageAnimation};
use std::collections::HashMap;
use tiny_skia::BlendMode;
use crate::animation::animation_template_renderer::DivoomAnimationTemplateRenderer;

pub struct DivoomAnimationTemplateManager {
    templates: HashMap<String, DivoomAnimationTemplate>,
    renderer: DivoomAnimationTemplateRenderer,
}

impl DivoomAnimationTemplateManager {
    pub fn new(template_configs: &[DivoomAnimationTemplateConfig], resource_dir: String) -> DivoomAPIResult<Self> {
        let parsed_template_result: DivoomAPIResult<Vec<DivoomAnimationTemplate>> =
            template_configs
                .iter()
                .map(DivoomAnimationTemplate::from_config)
                .collect();

        let mut manager = DivoomAnimationTemplateManager {
            templates: HashMap::new(),
            renderer: DivoomAnimationTemplateRenderer::new(resource_dir),
        };

        for template in parsed_template_result? {
            manager
                .templates
                .entry(template.name().to_string())
                .or_insert(template);
        }

        Ok(manager)
    }

    pub fn render_template(
        &self,
        template_name: &str,
        parameters: &HashMap<String, String>,
    ) -> DivoomAPIResult<DivoomImageAnimation> {
        let template = match self.templates.get(template_name) {
            None => return Err(DivoomAPIError::ParameterError(format!("Template not found. Please check the template name and try again."))),
            Some(v) => v,
        };

        let evaled_template = template.eval(parameters)?;
        let mut animation_builder = DivoomAnimationBuilder::new(evaled_template.canvas_size, evaled_template.speed)?;
        for evaled_frame in evaled_template.frames {
            let rendered_frame = self.renderer.render(&evaled_frame)?;
            animation_builder.new_frame().draw_frame_fit(
                &rendered_frame,
                DivoomDrawFitMode::Stretch,
                0.0,
                1.0,
                BlendMode::default());
        }

        let animation = animation_builder.build();
        Ok(animation)
    }
}
