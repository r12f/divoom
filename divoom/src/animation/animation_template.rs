use crate::{DivoomAPIError, DivoomAPIResult};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::Duration;

lazy_static! {
    static ref TEMPLATE_PARAM_REGEX: Regex = Regex::new(r"\{(\w+)\}").unwrap();
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomAnimationTemplateConfig {
    /// Canvas size
    pub canvas_size: u32,

    /// Animation play speed
    pub speed_in_ms: u64,

    /// SVG file paths that used to describe each animation frame
    pub frames: Vec<String>,
}

pub(crate) struct DivoomAnimationTemplate {
    name: String,
    canvas_size: u32,
    speed: Duration,
    frames: Vec<DivoomAnimationFrameTemplate>,
}

pub(crate) struct DivoomEvaluatedAnimationTemplate {
    pub canvas_size: u32,
    pub speed: Duration,
    pub frames: Vec<String>,
}

impl DivoomAnimationTemplate {
    pub fn from_config(
        name: String,
        config: &DivoomAnimationTemplateConfig,
        resource_dir: &str,
    ) -> DivoomAPIResult<Self> {
        let parsed_templates_result: DivoomAPIResult<Vec<DivoomAnimationFrameTemplate>> = config
            .frames
            .iter()
            .map(|v| DivoomAnimationFrameTemplate::from_file(format!("{}/{}", resource_dir, v)))
            .collect();

        Ok(DivoomAnimationTemplate {
            name,
            canvas_size: config.canvas_size,
            speed: Duration::from_millis(config.speed_in_ms),
            frames: parsed_templates_result?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn eval(
        &self,
        parameters: &HashMap<String, String>,
        per_frame_parameters: &HashMap<usize, HashMap<String, String>>,
    ) -> DivoomAPIResult<DivoomEvaluatedAnimationTemplate> {
        let evaled_template_result: DivoomAPIResult<Vec<String>> =
            self.frames.iter().enumerate().map(|(index, f)| {
                let frame_only_parameters = per_frame_parameters.get(&index);
                f.eval(parameters, frame_only_parameters)
            }).collect();

        Ok(DivoomEvaluatedAnimationTemplate {
            canvas_size: self.canvas_size,
            speed: self.speed,
            frames: evaled_template_result?,
        })
    }
}

pub(crate) struct DivoomAnimationFrameTemplate {
    file_path: String,
    file_content: String,
    param_name_to_pattern_map: HashMap<String, String>,
}

impl DivoomAnimationFrameTemplate {
    pub fn from_file(file_path: String) -> DivoomAPIResult<Self> {
        let file_content = fs::read_to_string(&file_path)?;
        let mut parsed_template = DivoomAnimationFrameTemplate {
            file_path,
            file_content,
            param_name_to_pattern_map: HashMap::new(),
        };

        for param_match in TEMPLATE_PARAM_REGEX.captures_iter(&parsed_template.file_content) {
            let param_name = param_match.get(1).unwrap().as_str().to_string();
            parsed_template
                .param_name_to_pattern_map
                .entry(param_name)
                .or_insert_with(|| param_match.get(0).unwrap().as_str().to_string());
        }

        Ok(parsed_template)
    }

    #[allow(dead_code)]
    pub fn file_path(&self) -> &str { self.file_path.as_ref() }

    pub fn eval(&self, parameters: &HashMap<String, String>, frame_only_parameters: Option<&HashMap<String, String>>) -> DivoomAPIResult<String> {
        let mut evaled_file_content = self.file_content.clone();

        for template_parameter_definition in &self.param_name_to_pattern_map {
            if frame_only_parameters.is_some() {
                if let Some(parameter) = frame_only_parameters.as_ref().unwrap().get(template_parameter_definition.0) {
                    evaled_file_content =
                        evaled_file_content.replace(template_parameter_definition.1, parameter);

                    continue;
                }
            }

            match parameters.get(template_parameter_definition.0) {
                None => {
                    return Err(DivoomAPIError::ParameterError(format!(
                        "Missing template parameter: {}",
                        template_parameter_definition.0
                    )));
                },

                Some(parameter) => {
                    evaled_file_content =
                        evaled_file_content.replace(template_parameter_definition.1, parameter);
                }
            }
        }

        Ok(evaled_file_content)
    }
}
