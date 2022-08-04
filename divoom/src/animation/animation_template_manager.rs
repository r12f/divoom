use crate::animation::animation_template::{
    DivoomAnimationTemplate, DivoomAnimationTemplateConfig,
};
use crate::{DivoomAnimationBuilder, DivoomAPIError, DivoomAPIResult, DivoomDrawFitMode, DivoomImageAnimation};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use log::debug;
use tiny_skia::BlendMode;
use crate::animation::animation_template_renderer::DivoomAnimationTemplateRenderer;

pub struct DivoomAnimationTemplateManager {
    resource_dir: String,
    templates: HashMap<String, DivoomAnimationTemplate>,
    renderer: DivoomAnimationTemplateRenderer,
}

impl DivoomAnimationTemplateManager {
    pub fn new(resource_dir: &str) -> DivoomAPIResult<Self> {
        Ok(DivoomAnimationTemplateManager {
            resource_dir: resource_dir.to_string(),
            templates: HashMap::new(),
            renderer: DivoomAnimationTemplateRenderer::new(resource_dir.to_string()),
        })
    }

    pub fn from_dir(template_folder: &str) -> DivoomAPIResult<Self> {
        let mut manager = DivoomAnimationTemplateManager::new(template_folder)?;
        manager.add_template_in_folder(template_folder)?;
        Ok(manager)
    }

    pub fn add_template_in_folder(&mut self, template_folder: &str) -> DivoomAPIResult<()> {
        debug!("Loading all animation templates in folder: {}", template_folder);

        for entry in fs::read_dir(template_folder)? {
            let entry = match entry {
                Err(_) => continue,
                Ok(v) => v,
            };

            let path = entry.path();
            if !path.is_file() || !path.ends_with(".yaml") {
                continue;
            }

            self.add_template_file(&path)?;
        }

        Ok(())
    }

    pub fn add_template_file(&mut self, template_path: &PathBuf) -> DivoomAPIResult<()> {
        debug!("Loading animation template file: {:?}", template_path);

        let template_name = template_path.file_stem().unwrap().to_os_string().into_string().unwrap();
        let template_file = File::open(template_path)?;
        let template_config: DivoomAnimationTemplateConfig = serde_yaml::from_reader(template_file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        self.add_template_config(template_name, &template_config)?;

        Ok(())
    }

    pub fn add_template_config(&mut self, template_name: String, template_config: &DivoomAnimationTemplateConfig) -> DivoomAPIResult<()> {
        debug!("Adding animation template: Name = {}", template_name);

        let parsed_template_config = DivoomAnimationTemplate::from_config(template_name, template_config, &self.resource_dir)?;
        self.templates.entry(parsed_template_config.name().to_string()).or_insert(parsed_template_config);

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn animation_template_manager_can_render_simple_graph_without_parameter() {
        let mut manager = DivoomAnimationTemplateManager::new("test_data/animation_template_tests/input").unwrap();
        manager.add_template_file(&"test_data/animation_template_tests/input/template_simple.yaml".into()).unwrap();

        let animation = manager.render_template("template_simple", &HashMap::new()).unwrap();
        test_utils::assert_animation_equal_with_baseline(&animation, "test_data/animation_template_tests/expected_generated_simple.png");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn animation_template_manager_can_render_text_without_parameter() {
        // Text are rendered based on fonts and other anti-aliasing/truetype algorithms on each OS. Even we use the same font, the result can be different.
        // Hence we only run the text test on windows, but nowhere else.
        let mut manager = DivoomAnimationTemplateManager::new("test_data/animation_template_tests/input").unwrap();
        manager.add_template_file(&"test_data/animation_template_tests/input/template_simple_text.yaml".into()).unwrap();

        let animation = manager.render_template("template_simple_text", &HashMap::new()).unwrap();
        test_utils::assert_animation_equal_with_baseline(&animation, "test_data/animation_template_tests/expected_generated_simple_text.png");
    }
}