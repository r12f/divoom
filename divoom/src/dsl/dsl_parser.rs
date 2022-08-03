use crate::dsl::dsl_resource_loader::*;
use crate::dsl::dsl_syntax::*;
use crate::dsl::DivoomDslOperation;
use crate::{DivoomAPIError, DivoomAPIResult};
use clap::Parser;

pub struct DivoomDslParser {}

impl DivoomDslParser {
    pub fn parse(command_input: &str) -> DivoomAPIResult<DivoomDslOperation> {
        let words = match shellwords::split(command_input) {
            Err(e) => return Err(DivoomAPIError::ParameterError(e.to_string())),
            Ok(v) => v,
        };

        let command_with_words = ["divoom".to_string()].into_iter().chain(words);
        let command: DivoomDeviceCommand =
            match DivoomDeviceCommand::try_parse_from(command_with_words) {
                Err(e) => return Err(DivoomAPIError::ParameterError(e.to_string())),
                Ok(v) => v,
            };

        let mut resource_loader = DivoomDslOperationNoOpResourceLoader::new();

        #[cfg(feature = "animation-builder")]
        if let DivoomDeviceCommand::Animation(animation_command) = &command {
            if let DivoomDeviceAnimationCommand::Image(animation_image_command) = animation_command
            {
                if let DivoomDeviceImageAnimationCommand::RenderGif { file_path, .. } =
                    animation_image_command
                {
                    resource_loader = DivoomDslOperationFileResourceLoader::new(file_path);
                } else if let DivoomDeviceImageAnimationCommand::RenderFiles {
                    file_pattern,
                    random,
                    prefetch_count,
                    ..
                } = animation_image_command
                {
                    resource_loader = DivoomDslOperationGlobResourceLoader::new(
                        file_pattern,
                        *random,
                        *prefetch_count,
                    );
                }
            }
        }

        Ok(DivoomDslOperation::new(command, resource_loader))
    }
}
