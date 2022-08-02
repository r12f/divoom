use std::time::Duration;
use crate::{DivoomAPIError, PixooClient, DivoomAPIResult, PixooCommandBuilder};
use clap::Parser;
use crate::dto::*;
use crate::dsl::dsl_syntax::*;

#[cfg(feature = "animation-builder")]
use crate::{DivoomAnimationBuilder, DivoomAnimationResourceLoader};

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

    pub async fn parse(&mut self, command_input: &str) -> DivoomAPIResult<()> {
        let words = match shellwords::split(command_input) {
            Err(e) => return Err(DivoomAPIError::ParameterError(e.to_string())),
            Ok(v) => v,
        };

        let command_with_words = ["divoom".to_string()].into_iter().chain(words);
        let command: DivoomDeviceCommand = match DivoomDeviceCommand::try_parse_from(command_with_words) {
            Err(e) => return Err(DivoomAPIError::ParameterError(e.to_string())),
            Ok(v) => v,
        };

        match command {
            DivoomDeviceCommand::Channel(channel_command) => {
                self.parse_channel_commands(channel_command)
            }

            DivoomDeviceCommand::System(system_command) => {
                self.parse_system_commands(system_command)
            }

            DivoomDeviceCommand::Tool(tool_command) => {
                self.parse_tool_commands(tool_command)
            }

            DivoomDeviceCommand::Animation(animation_command) => {
                self.parse_animation_commands(animation_command).await
            }

            DivoomDeviceCommand::Batch(batch_command) => {
                self.parse_batch_commands(batch_command)
            }

            DivoomDeviceCommand::Raw { request } => {
                self.command_builder = Some(self.command_builder.take().unwrap().send_raw_request(request));
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub async fn execute(mut self) -> DivoomAPIResult<()> {
        self.command_builder.take().unwrap().execute().await
    }

    pub(crate) fn build(mut self) -> (usize, String) {
        let (_, command_count, payload) = self.command_builder.take().unwrap().build();
        (command_count, payload)
    }

    fn parse_channel_commands(
        &mut self,
        command: DivoomDeviceChannelCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceChannelCommand::Set { channel_type } => {
                self.command_builder = Some(self.command_builder.take().unwrap().select_channel(channel_type))
            },

            DivoomDeviceChannelCommand::SetClock { clock_id } => {
                self.command_builder = Some(self.command_builder.take().unwrap().select_clock(clock_id))
            },

            DivoomDeviceChannelCommand::SetCloudChannel { channel_type } => {
                self.command_builder = Some(self.command_builder.take().unwrap().select_cloud_channel(channel_type))
            }

            DivoomDeviceChannelCommand::SetCustomPage { page_index } => {
                self.command_builder = Some(self.command_builder.take().unwrap().select_custom_page(page_index))
            }

            DivoomDeviceChannelCommand::SetVisualizer { visualizer_index } => {
                self.command_builder = Some(self.command_builder.take().unwrap().select_visualizer(visualizer_index))
            }
        }

        Ok(())
    }

    fn parse_system_commands(
        &mut self,
        command: DivoomDeviceSystemCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceSystemCommand::SetTime { utc } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_time(utc))
            },

            DivoomDeviceSystemCommand::SetBrightness { brightness } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_brightness(brightness))
            }

            DivoomDeviceSystemCommand::SetHourMode { mode } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_hour_mode(mode))
            },

            DivoomDeviceSystemCommand::SetHighLightMode { mode } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_high_light_mode(mode))
            }

            DivoomDeviceSystemCommand::SetMirrorMode { mode } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_mirror_mode(mode))
            },

            DivoomDeviceSystemCommand::SetRotationAngle { mode } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_rotation_angle(mode))
            }

            DivoomDeviceSystemCommand::SetScreenPowerState { power_state } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_screen_power_state(power_state))
            }

            DivoomDeviceSystemCommand::SetTemperatureUnit { unit } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_temperature_unit(unit))
            }

            DivoomDeviceSystemCommand::SetTimeZone { time_zone } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_time_zone(time_zone))
            }

            DivoomDeviceSystemCommand::SetWeatherArea {
                longitude,
                latitude,
            } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_weather_area(longitude, latitude))
            },

            DivoomDeviceSystemCommand::SetWhiteBalance { r, g, b } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_device_white_balance(r, g, b))
            }
        }

        Ok(())
    }

    fn parse_tool_commands(
        &mut self,
        command: DivoomDeviceToolCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceToolCommand::Countdown {
                minute,
                second,
                action,
            } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_countdown_tool(minute, second, action))
            },

            DivoomDeviceToolCommand::Noise { action } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_noise_tool(action))
            },

            DivoomDeviceToolCommand::Scoreboard {
                blue_score,
                red_score,
            } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_scoreboard_tool(blue_score, red_score))
            },

            DivoomDeviceToolCommand::Stopwatch { action } => {
                self.command_builder = Some(self.command_builder.take().unwrap().set_stopwatch_tool(action))
            },
        }

        Ok(())
    }

    async fn parse_animation_commands(
        &mut self,
        command: DivoomDeviceAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceAnimationCommand::Gif(gif_animation_command) => {
                self.parse_gif_animation_commands(gif_animation_command)
            }

            DivoomDeviceAnimationCommand::Image(image_animation_command) => {
                self.parse_image_animation_commands(image_animation_command).await
            }

            DivoomDeviceAnimationCommand::Text(text_animation_command) => {
                self.parse_text_animation_commands(text_animation_command)
            }

            DivoomDeviceAnimationCommand::Buzzer {
                play_total_time,
                active_time_in_cycle,
                off_time_in_cycle,
            } => {
                self.command_builder = Some(self.command_builder.take().unwrap().play_buzzer(play_total_time, active_time_in_cycle, off_time_in_cycle));
                Ok(())
            }
        }
    }

    fn parse_gif_animation_commands(
        &mut self,
        command: DivoomDeviceGifAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match command {
            DivoomDeviceGifAnimationCommand::Play(gif) => {
                if let Some(local_gif_file) = gif.file {
                    self.command_builder = Some(self.command_builder.take().unwrap()
                        .play_gif_file(DivoomFileAnimationSourceType::LocalFile, local_gif_file));
                } else if let Some(local_gif_folder) = gif.folder {
                    self.command_builder = Some(self.command_builder.take().unwrap()
                        .play_gif_file(DivoomFileAnimationSourceType::LocalFolder, local_gif_folder));
                } else if let Some(gif_url) = gif.url {
                    self.command_builder = Some(self.command_builder.take().unwrap()
                        .play_gif_file(DivoomFileAnimationSourceType::Url, gif_url));
                } else {
                    return Err(DivoomAPIError::ParameterError(
                        "The source of GIF is not set!".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    async fn parse_image_animation_commands(
        &mut self,
        image_animation_command: DivoomDeviceImageAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match image_animation_command {
            DivoomDeviceImageAnimationCommand::ResetId => {
                self.command_builder = Some(self.command_builder.take().unwrap().reset_next_animation_id())
            },

            #[cfg(feature = "animation-builder")]
            DivoomDeviceImageAnimationCommand::SendGif {
                file_path,
                size: canvas_size,
                speed_in_ms,
                fit,
                rotation,
                opacity,
            } => {
                let animation_builder = DivoomAnimationBuilder::new(canvas_size, Duration::from_millis(speed_in_ms))?;
                let gif = DivoomAnimationResourceLoader::from_gif_file(&file_path)?;
                let animation = animation_builder
                    .draw_frames_fit(&gif, 0, fit, rotation, opacity, BlendMode::default())
                    .build();

                let animation_id = self.device_client.get_next_animation_id().await?;
                self.command_builder = Some(self.command_builder.take().unwrap().send_image_animation(animation_id, animation));
            }
        }

        Ok(())
    }

    fn parse_text_animation_commands(
        &mut self,
        text_animation_command: DivoomDeviceTextAnimationCommand,
    ) -> DivoomAPIResult<()> {
        match text_animation_command {
            DivoomDeviceTextAnimationCommand::Clear => {
                self.command_builder = Some(self.command_builder.take().unwrap().clear_all_text_area())
            },

            DivoomDeviceTextAnimationCommand::Set(text_animation) => {
                self.command_builder = Some(self.command_builder.take().unwrap().send_text_animation(text_animation.into()))
            }
        }

        Ok(())
    }

    fn parse_batch_commands(
        &mut self,
        batch_command: DivoomDeviceBatchCommand,
    ) -> DivoomAPIResult<()> {
        match batch_command {
            DivoomDeviceBatchCommand::RunUrl { command_url } => {
                self.command_builder = Some(self.command_builder.take().unwrap().execute_commands_from_url(command_url))
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::PixooClient;
    use super::*;
    use std::{env, fs};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_channel_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("channel set clock").await.unwrap();
        dsl_runner.parse("channel set cloud").await.unwrap();
        dsl_runner.parse("channel set visualizer").await.unwrap();
        dsl_runner.parse("channel set customPage").await.unwrap();
        dsl_runner.parse("channel set-clock 100").await.unwrap();
        dsl_runner.parse("channel set-cloud-channel gallery").await.unwrap();
        dsl_runner.parse("channel set-cloud-channel fav").await.unwrap();
        dsl_runner.parse("channel set-cloud-channel artist").await.unwrap();
        dsl_runner.parse("channel set-custom-page 2").await.unwrap();
        dsl_runner.parse("channel set-visualizer 5").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/channel_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_system_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("system set-brightness 80").await.unwrap();
        dsl_runner.parse("system set-time 10000").await.unwrap();
        dsl_runner.parse("system set-high-light-mode on").await.unwrap();
        dsl_runner.parse("system set-hour-mode 12h").await.unwrap();
        dsl_runner.parse("system set-mirror-mode on").await.unwrap();
        dsl_runner.parse("system set-rotation-angle 90").await.unwrap();
        dsl_runner.parse("system set-screen-power-state off").await.unwrap();
        dsl_runner.parse("system set-temperature-unit c").await.unwrap();
        dsl_runner.parse("system set-time-zone \"America/Los Angeles\"").await.unwrap();
        dsl_runner.parse("system set-weather-area \"-122.0\" 47.0").await.unwrap();
        dsl_runner.parse("system set-white-balance 100 150 200").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/system_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_tool_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("tool countdown start 10 30").await.unwrap();
        dsl_runner.parse("tool countdown stop").await.unwrap();
        dsl_runner.parse("tool noise start").await.unwrap();
        dsl_runner.parse("tool noise stop").await.unwrap();
        dsl_runner.parse("tool scoreboard 1 2").await.unwrap();
        dsl_runner.parse("tool stopwatch start").await.unwrap();
        dsl_runner.parse("tool stopwatch stop").await.unwrap();
        dsl_runner.parse("tool stopwatch reset").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/tool_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_animation_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("animation gif play --file d:\\1.gif").await.unwrap();
        dsl_runner.parse("animation gif play --folder d:\\1").await.unwrap();
        dsl_runner.parse("animation gif play --url http://example.com/1.gif").await.unwrap();
        dsl_runner.parse("animation image reset-id").await.unwrap();
        dsl_runner.parse("animation text clear").await.unwrap();
        dsl_runner.parse("animation text set 0 -x 0 -y 0 -d left -f 1 -w 32 \"test string\" -r 100 -g 150 -b 150 -a middle").await.unwrap();
        dsl_runner.parse("animation buzzer").await.unwrap();
        dsl_runner.parse("animation buzzer 500 -a 100 -o 200").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/animation_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_batch_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("batch run-url http://example.com/commands.txt").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/batch_commands.json",
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dsl_runner_should_parse_raw_commands() {
        let client = PixooClient::new("127.0.0.1").unwrap();

        let mut dsl_runner = DivoomDslRunner::new(&client);
        dsl_runner.parse("raw \"{ \\\"Command\\\": \\\"Tools/SetStopWatch\\\", \\\"Status\\\": 1 }\"").await.unwrap();

        run_dsl_runner_parser_test(
            dsl_runner,
            "test_data/dsl_runner_tests/raw_commands.json",
        );
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
