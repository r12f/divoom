use crate::dsl::dsl_syntax::*;
use crate::dto::*;
use crate::{DivoomAPIError, DivoomAPIResult, PixooClient, PixooCommandBuilder};
use std::time::Duration;

#[cfg(feature = "animation-builder")]
use crate::{DivoomAnimationBuilder, DivoomAnimationResourceLoader};

use crate::dsl::DivoomDslOperation;
#[cfg(feature = "animation-builder")]
use tiny_skia::BlendMode;

pub struct DivoomDslRunner<'a> {
    device_client: &'a PixooClient,
    command_builder: Option<PixooCommandBuilder>,
}

impl DivoomDslRunner<'_> {
    pub fn new(device_client: &PixooClient) -> DivoomDslRunner {
        let command_builder = device_client.start_batch();

        DivoomDslRunner {
            device_client,
            command_builder: Some(command_builder),
        }
    }

    pub async fn batch_operations(
        &mut self,
        operations: &[DivoomDslOperation],
    ) -> DivoomAPIResult<()> {
        for operation in operations {
            self.batch_operation(operation).await?;
        }

        Ok(())
    }

    pub async fn batch_operation(&mut self, operation: &DivoomDslOperation) -> DivoomAPIResult<()> {
        match &operation.command {
            DivoomDeviceCommand::Channel(channel_command) => {
                self.batch_channel_command(operation, channel_command)
            }

            DivoomDeviceCommand::System(system_command) => {
                self.batch_system_command(operation, system_command)
            }

            DivoomDeviceCommand::Tool(tool_command) => {
                self.batch_tool_command(operation, tool_command)
            }

            DivoomDeviceCommand::Animation(animation_command) => {
                self.batch_animation_command(operation, animation_command)
                    .await
            }

            DivoomDeviceCommand::Batch(batch_command) => {
                self.batch_batch_command(operation, batch_command)
            }

            DivoomDeviceCommand::Raw { request } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .send_raw_request(request.clone()),
                );
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub async fn execute(mut self) -> DivoomAPIResult<()> {
        self.command_builder.take().unwrap().execute().await
    }

    #[allow(dead_code)]
    pub(crate) fn build(mut self) -> (usize, String) {
        let (_, command_count, payload) = self.command_builder.take().unwrap().build();
        (command_count, payload)
    }

    fn batch_channel_command(
        &mut self,
        _: &DivoomDslOperation,
        command: &DivoomDeviceChannelCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceChannelCommand::Set { channel_type } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .select_channel(*channel_type),
                )
            }

            DivoomDeviceChannelCommand::SetClock { clock_id } => {
                self.command_builder =
                    Some(self.command_builder.take().unwrap().select_clock(*clock_id))
            }

            DivoomDeviceChannelCommand::SetCloudChannel { channel_type } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .select_cloud_channel(*channel_type),
                )
            }

            DivoomDeviceChannelCommand::SetCustomPage { page_index } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .select_custom_page(*page_index),
                )
            }

            DivoomDeviceChannelCommand::SetVisualizer { visualizer_index } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .select_visualizer(*visualizer_index),
                )
            }
        }

        Ok(())
    }

    fn batch_system_command(
        &mut self,
        _: &DivoomDslOperation,
        command: &DivoomDeviceSystemCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceSystemCommand::SetTime { utc } => {
                self.command_builder =
                    Some(self.command_builder.take().unwrap().set_device_time(*utc))
            }

            DivoomDeviceSystemCommand::SetBrightness { brightness } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_brightness(*brightness),
                )
            }

            DivoomDeviceSystemCommand::SetHourMode { mode } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_hour_mode(*mode),
                )
            }

            DivoomDeviceSystemCommand::SetHighLightMode { mode } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_high_light_mode(*mode),
                )
            }

            DivoomDeviceSystemCommand::SetMirrorMode { mode } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_mirror_mode(*mode),
                )
            }

            DivoomDeviceSystemCommand::SetRotationAngle { mode } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_rotation_angle(*mode),
                )
            }

            DivoomDeviceSystemCommand::SetScreenPowerState { power_state } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_screen_power_state(*power_state),
                )
            }

            DivoomDeviceSystemCommand::SetTemperatureUnit { unit } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_temperature_unit(*unit),
                )
            }

            DivoomDeviceSystemCommand::SetTimeZone { time_zone } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_time_zone(time_zone.to_string()),
                )
            }

            DivoomDeviceSystemCommand::SetWeatherArea {
                longitude,
                latitude,
            } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_weather_area(longitude.to_string(), latitude.to_string()),
                )
            }

            DivoomDeviceSystemCommand::SetWhiteBalance { r, g, b } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_device_white_balance(*r, *g, *b),
                )
            }
        }

        Ok(())
    }

    fn batch_tool_command(
        &mut self,
        _: &DivoomDslOperation,
        command: &DivoomDeviceToolCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceToolCommand::Countdown {
                minute,
                second,
                action,
            } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_countdown_tool(*minute, *second, *action),
                )
            }

            DivoomDeviceToolCommand::Noise { action } => {
                self.command_builder =
                    Some(self.command_builder.take().unwrap().set_noise_tool(*action))
            }

            DivoomDeviceToolCommand::Scoreboard {
                blue_score,
                red_score,
            } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_scoreboard_tool(*blue_score, *red_score),
                )
            }

            DivoomDeviceToolCommand::Stopwatch { action } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .set_stopwatch_tool(*action),
                )
            }

            DivoomDeviceToolCommand::Buzzer {
                play_total_time,
                active_time_in_cycle,
                off_time_in_cycle,
            } => {
                self.command_builder = Some(self.command_builder.take().unwrap().play_buzzer(
                    *play_total_time,
                    *active_time_in_cycle,
                    *off_time_in_cycle,
                ));
            }
        }

        Ok(())
    }

    async fn batch_animation_command(
        &mut self,
        operation: &DivoomDslOperation,
        command: &DivoomDeviceAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceAnimationCommand::Gif(gif_animation_command) => {
                self.batch_gif_animation_commands(operation, gif_animation_command)
            }

            DivoomDeviceAnimationCommand::Image(image_animation_command) => {
                self.batch_image_animation_commands(operation, image_animation_command)
                    .await
            }

            DivoomDeviceAnimationCommand::Text(text_animation_command) => {
                self.batch_text_animation_commands(operation, text_animation_command)
            }
        }
    }

    fn batch_gif_animation_commands(
        &mut self,
        _: &DivoomDslOperation,
        command: &DivoomDeviceGifAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceGifAnimationCommand::Play(gif) => {
                if let Some(local_gif_file) = &gif.file {
                    self.command_builder =
                        Some(self.command_builder.take().unwrap().play_gif_file(
                            DivoomFileAnimationSourceType::LocalFile,
                            local_gif_file.clone(),
                        ));
                } else if let Some(local_gif_folder) = &gif.folder {
                    self.command_builder =
                        Some(self.command_builder.take().unwrap().play_gif_file(
                            DivoomFileAnimationSourceType::LocalFolder,
                            local_gif_folder.clone(),
                        ));
                } else if let Some(gif_url) = &gif.url {
                    self.command_builder = Some(
                        self.command_builder
                            .take()
                            .unwrap()
                            .play_gif_file(DivoomFileAnimationSourceType::Url, gif_url.clone()),
                    );
                } else {
                    return Err(DivoomAPIError::ParameterError(
                        "The source of GIF is not set!".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    async fn batch_image_animation_commands(
        &mut self,
        operation: &DivoomDslOperation,
        image_animation_command: &DivoomDeviceImageAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match image_animation_command {
            DivoomDeviceImageAnimationCommand::ResetId => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .reset_next_animation_id(),
                )
            }

            #[cfg(feature = "animation-builder")]
            DivoomDeviceImageAnimationCommand::RenderGif {
                size: canvas_size,
                speed_in_ms,
                fit,
                rotation,
                opacity,
                ..
            } => {
                let animation_builder =
                    DivoomAnimationBuilder::new(*canvas_size, Duration::from_millis(*speed_in_ms))?;
                let gif_file_data = operation.resource_loader.lock().unwrap().as_mut().next()?;
                let gif = DivoomAnimationResourceLoader::from_gif_buf(&gif_file_data.data)?;
                let animation = animation_builder
                    .draw_frames_fit(&gif, 0, *fit, *rotation, *opacity, BlendMode::default())
                    .build();

                let animation_id = self.device_client.get_next_animation_id().await?;
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .send_image_animation(animation_id, animation),
                );
            }

            #[cfg(feature = "animation-builder")]
            DivoomDeviceImageAnimationCommand::RenderFiles {
                size: canvas_size,
                speed_in_ms,
                fit,
                rotation,
                opacity,
                ..
            } => {
                let animation: DivoomImageAnimation;

                let mut animation_builder =
                    DivoomAnimationBuilder::new(*canvas_size, Duration::from_millis(*speed_in_ms))?;

                let file_resource = operation.resource_loader.lock().unwrap().as_mut().next()?;
                if file_resource.name.ends_with(".gif") {
                    let gif = DivoomAnimationResourceLoader::from_gif_buf(&file_resource.data)?;
                    animation = animation_builder
                        .draw_frames_fit(&gif, 0, *fit, *rotation, *opacity, BlendMode::default())
                        .build();
                } else {
                    let image = DivoomAnimationResourceLoader::from_image_buf(&file_resource.data)?;
                    animation_builder.build_frame(0).draw_frame_fit(&image, *fit, *rotation, *opacity, BlendMode::default());
                    animation = animation_builder.build();
                }

                let animation_id = self.device_client.get_next_animation_id().await?;
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .send_image_animation(animation_id, animation),
                );
            }
        }

        Ok(())
    }

    fn batch_text_animation_commands(
        &mut self,
        _: &DivoomDslOperation,
        text_animation_command: &DivoomDeviceTextAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match text_animation_command {
            DivoomDeviceTextAnimationCommand::Clear => {
                self.command_builder =
                    Some(self.command_builder.take().unwrap().clear_all_text_area())
            }

            DivoomDeviceTextAnimationCommand::Set(text_animation) => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .send_text_animation(text_animation.as_animation()),
                )
            }
        }

        Ok(())
    }

    fn batch_batch_command(
        &mut self,
        _: &DivoomDslOperation,
        batch_command: &DivoomDeviceBatchCommand,
    ) -> DivoomAPIResult<()> {
        match batch_command {
            DivoomDeviceBatchCommand::RunUrl { command_url } => {
                self.command_builder = Some(
                    self.command_builder
                        .take()
                        .unwrap()
                        .execute_commands_from_url(command_url.to_string()),
                )
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::dsl_parser::DivoomDslParser;
    use crate::PixooClient;
    use std::{env, fs};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_channel_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations = vec![
            DivoomDslParser::parse("channel set clock").unwrap(),
            DivoomDslParser::parse("channel set cloud").unwrap(),
            DivoomDslParser::parse("channel set visualizer").unwrap(),
            DivoomDslParser::parse("channel set customPage").unwrap(),
            DivoomDslParser::parse("channel set-clock 100").unwrap(),
            DivoomDslParser::parse("channel set-cloud-channel gallery").unwrap(),
            DivoomDslParser::parse("channel set-cloud-channel fav").unwrap(),
            DivoomDslParser::parse("channel set-cloud-channel artist").unwrap(),
            DivoomDslParser::parse("channel set-custom-page 2").unwrap(),
            DivoomDslParser::parse("channel set-visualizer 5").unwrap(),
        ];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/channel_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_system_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations = vec![
            DivoomDslParser::parse("system set-brightness 80").unwrap(),
            DivoomDslParser::parse("system set-time 10000").unwrap(),
            DivoomDslParser::parse("system set-high-light-mode on").unwrap(),
            DivoomDslParser::parse("system set-hour-mode 12h").unwrap(),
            DivoomDslParser::parse("system set-mirror-mode on").unwrap(),
            DivoomDslParser::parse("system set-rotation-angle 90").unwrap(),
            DivoomDslParser::parse("system set-screen-power-state off").unwrap(),
            DivoomDslParser::parse("system set-temperature-unit c").unwrap(),
            DivoomDslParser::parse("system set-time-zone \"America/Los Angeles\"").unwrap(),
            DivoomDslParser::parse("system set-weather-area \"-122.0\" 47.0").unwrap(),
            DivoomDslParser::parse("system set-white-balance 100 150 200").unwrap(),
        ];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/system_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_tool_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations = vec![
            DivoomDslParser::parse("tool countdown start 10 30").unwrap(),
            DivoomDslParser::parse("tool countdown stop").unwrap(),
            DivoomDslParser::parse("tool noise start").unwrap(),
            DivoomDslParser::parse("tool noise stop").unwrap(),
            DivoomDslParser::parse("tool scoreboard 1 2").unwrap(),
            DivoomDslParser::parse("tool stopwatch start").unwrap(),
            DivoomDslParser::parse("tool stopwatch stop").unwrap(),
            DivoomDslParser::parse("tool stopwatch reset").unwrap(),
            DivoomDslParser::parse("tool buzzer").unwrap(),
            DivoomDslParser::parse("tool buzzer 500 -a 100 -o 200").unwrap(),
        ];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(dsl_runner, "test_data/dsl_runner_tests/tool_commands.json");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_animation_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations = vec![
            DivoomDslParser::parse("animation gif play --file d:\\1.gif").unwrap(),
            DivoomDslParser::parse("animation gif play --folder d:\\1").unwrap(),
            DivoomDslParser::parse("animation gif play --url http://example.com/1.gif").unwrap(),
            DivoomDslParser::parse("animation image reset-id").unwrap(),
            DivoomDslParser::parse("animation text clear").unwrap(),
            DivoomDslParser::parse("animation text set 0 -x 0 -y 0 -d left -f 1 -w 32 \"test string\" -r 100 -g 150 -b 150 -a middle").unwrap(),
        ];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/animation_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_batch_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations =
            vec![DivoomDslParser::parse("batch run-url http://example.com/commands.txt").unwrap()];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(dsl_runner, "test_data/dsl_runner_tests/batch_commands.json");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_batch_raw_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        let operations = vec![DivoomDslParser::parse(
            "raw \"{ \\\"Command\\\": \\\"Tools/SetStopWatch\\\", \\\"Status\\\": 1 }\"",
        )
        .unwrap()];
        dsl_runner.batch_operations(&operations).await.unwrap();

        run_dsl_runner_parser_test(dsl_runner, "test_data/dsl_runner_tests/raw_commands.json");
    }

    fn run_dsl_runner_parser_test(dsl_runner: DivoomDslRunner, reference_file_path: &str) {
        let (_, request_body) = dsl_runner.build();

        let actual: serde_json::Value =
            serde_json::from_str(&request_body).expect("Parsing request body failed!");

        if env::var("DIVOOM_API_GENERATE_TEST_DATA").is_ok() {
            let formatted_request_body =
                serde_json::to_string_pretty(&actual).expect("Serialize commands failed!");
            fs::write(reference_file_path, formatted_request_body).unwrap_or_else(|_| {
                panic!(
                    "Generate test data file failed! Path = {}",
                    reference_file_path
                )
            });
            return;
        }

        let reference_commands_text =
            fs::read_to_string(reference_file_path).expect("Reading reference file failed!");
        let expected: serde_json::Value =
            serde_json::from_str(&reference_commands_text).expect("Parsing reference data failed!");
        assert_eq!(actual, expected);
    }
}
