use crate::animation::animation_template::{
    DivoomAnimationTemplate, DivoomAnimationTemplateConfig,
};
use crate::DivoomAPIResult;
use std::collections::HashMap;

pub struct DivoomAnimationTemplateManager {
    templates: HashMap<String, DivoomAnimationTemplate>,
}

impl DivoomAnimationTemplateManager {
    pub fn new(template_configs: &[DivoomAnimationTemplateConfig]) -> DivoomAPIResult<Self> {
        let parsed_template_result: DivoomAPIResult<Vec<DivoomAnimationTemplate>> =
            template_configs
                .iter()
                .map(DivoomAnimationTemplate::from_config)
                .collect();

        let mut manager = DivoomAnimationTemplateManager {
            templates: HashMap::new(),
        };

        for template in parsed_template_result? {
            manager
                .templates
                .entry(template.name().to_string())
                .or_insert(template);
        }

        Ok(manager)
    }
}
