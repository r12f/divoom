mod opt;

use crate::opt::*;
use divoom::*;
use serde::Serialize;
use std::time::Duration;
use structopt::StructOpt;
use tiny_skia::BlendMode;

#[tokio::main]
async fn main() -> DivoomAPIResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let opts = DivoomCliOptions::from_args();
    match opts.command {
        DivoomCliSubCommand::Discover => {
            let divoom = DivoomServiceClient::new();
            let devices = divoom.get_same_lan_devices().await?;
            serialize_to_console(devices, opts.common.output);
            Ok(())
        }

        DivoomCliSubCommand::Channel(channel_command) => {
            handle_channel_api(&opts.common, channel_command).await
        }

        DivoomCliSubCommand::System(system_command) => {
            handle_system_api(&opts.common, system_command).await
        }

        DivoomCliSubCommand::Tool(tool_command) => {
            handle_tool_api(&opts.common, tool_command).await
        }

        DivoomCliSubCommand::Animation(animation_command) => {
            handle_animation_api(&opts.common, animation_command).await
        }

        DivoomCliSubCommand::Batch(batch_command) => {
            handle_batch_api(&opts.common, batch_command).await
        }

        DivoomCliSubCommand::Raw { request } => {
            let pixoo = PixooClient::new(
                opts.common
                    .device_address
                    .as_ref()
                    .expect("Device Address is not set!"),
            );
            let response = pixoo.send_raw_request(request).await?;
            serialize_to_console(response, opts.common.output);
            Ok(())
        }
    }
}

async fn handle_channel_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    channel_command: DivoomCliChannelCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match channel_command {
        DivoomCliChannelCommand::Get => {
            let result = pixoo.get_current_channel().await?;
            serialize_to_console(result, common.output);
            Ok(())
        }

        DivoomCliChannelCommand::GetClock => {
            let result = pixoo.get_selected_clock_info().await?;
            serialize_to_console(result, common.output);
            Ok(())
        }

        DivoomCliChannelCommand::Set { channel_type } => pixoo.select_channel(channel_type).await,

        DivoomCliChannelCommand::SetClock { clock_id } => pixoo.select_clock(clock_id).await,

        DivoomCliChannelCommand::SetCloudChannel { channel_type } => {
            pixoo.select_cloud_channel(channel_type).await
        }

        DivoomCliChannelCommand::SetCustomPage { page_index } => {
            pixoo.select_custom_page(page_index).await
        }

        DivoomCliChannelCommand::SetVisualizer { visualizer_index } => {
            pixoo.select_visualizer(visualizer_index).await
        }
    }
}

async fn handle_system_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    system_command: DivoomCliSystemCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match system_command {
        DivoomCliSystemCommand::GetSettings => {
            let result = pixoo.get_device_settings().await?;
            serialize_to_console(result, common.output);
            Ok(())
        }

        DivoomCliSystemCommand::GetTime => {
            let result = pixoo.get_device_time().await?;
            serialize_to_console(result, common.output);
            Ok(())
        }

        DivoomCliSystemCommand::SetTime { utc } => pixoo.set_device_time(utc).await,

        DivoomCliSystemCommand::SetBrightness { brightness } => {
            pixoo.set_device_brightness(brightness).await
        }

        DivoomCliSystemCommand::SetHourMode { mode } => pixoo.set_device_hour_mode(mode).await,

        DivoomCliSystemCommand::SetHighLightMode { mode } => {
            pixoo.set_device_high_light_mode(mode).await
        }

        DivoomCliSystemCommand::SetMirrorMode { mode } => pixoo.set_device_mirror_mode(mode).await,

        DivoomCliSystemCommand::SetRotationAngle { mode } => {
            pixoo.set_device_rotation_angle(mode).await
        }

        DivoomCliSystemCommand::SetScreenPowerState { power_state } => {
            pixoo.set_device_screen_power_state(power_state).await
        }

        DivoomCliSystemCommand::SetTemperatureUnit { unit } => {
            pixoo.set_device_temperature_unit(unit).await
        }

        DivoomCliSystemCommand::SetTimeZone { time_zone } => {
            pixoo.set_device_time_zone(time_zone).await
        }

        DivoomCliSystemCommand::SetWeatherArea {
            longitude,
            latitude,
        } => pixoo.set_device_weather_area(longitude, latitude).await,

        DivoomCliSystemCommand::SetWhiteBalance { r, g, b } => {
            pixoo.set_device_white_balance(r, g, b).await
        }
    }
}

async fn handle_tool_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    tool_command: DivoomCliToolCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match tool_command {
        DivoomCliToolCommand::Countdown {
            minute,
            second,
            action,
        } => pixoo.set_countdown_tool(minute, second, action).await,

        DivoomCliToolCommand::Noise { action } => pixoo.set_noise_tool(action).await,

        DivoomCliToolCommand::Scoreboard {
            blue_score,
            red_score,
        } => pixoo.set_scoreboard_tool(blue_score, red_score).await,

        DivoomCliToolCommand::Stopwatch { action } => pixoo.set_stopwatch_tool(action).await,
    }
}

async fn handle_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    animation_command: DivoomCliAnimationCommand,
) -> DivoomAPIResult<()> {
    match animation_command {
        DivoomCliAnimationCommand::Gif(gif_animation_command) => {
            handle_gif_animation_api(common, gif_animation_command).await
        }

        DivoomCliAnimationCommand::Image(image_animation_command) => {
            handle_image_animation_api(common, image_animation_command).await
        }

        DivoomCliAnimationCommand::Text(text_animation_command) => {
            handle_text_animation_api(common, text_animation_command).await
        }

        DivoomCliAnimationCommand::Buzzer {
            play_total_time,
            active_time_in_cycle,
            off_time_in_cycle,
        } => {
            let pixoo = PixooClient::new(
                common
                    .device_address
                    .as_ref()
                    .expect("Device Address is not set!"),
            );
            pixoo
                .play_buzzer(play_total_time, active_time_in_cycle, off_time_in_cycle)
                .await
        }
    }
}

async fn handle_gif_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    gif_animation_command: DivoomCliGifAnimationCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match gif_animation_command {
        DivoomCliGifAnimationCommand::Play(gif) => {
            if let Some(local_gif_file) = gif.file {
                pixoo
                    .play_gif_file(DivoomFileAnimationSourceType::LocalFile, local_gif_file)
                    .await?;
            } else if let Some(local_gif_folder) = gif.folder {
                pixoo
                    .play_gif_file(DivoomFileAnimationSourceType::LocalFolder, local_gif_folder)
                    .await?;
            } else if let Some(gif_url) = gif.url {
                pixoo
                    .play_gif_file(DivoomFileAnimationSourceType::Url, gif_url)
                    .await?;
            } else {
                return Err(DivoomAPIError::ParameterError(
                    "The source of GIF is not set!".into(),
                ));
            }

            Ok(())
        }
    }
}

async fn handle_image_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    image_animation_command: DivoomCliImageAnimationCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match image_animation_command {
        DivoomCliImageAnimationCommand::GetNextId => {
            let result = pixoo.get_next_animation_id().await?;
            serialize_to_console(result, common.output);
            Ok(())
        }

        DivoomCliImageAnimationCommand::ResetId => pixoo.reset_next_animation_id().await,

        DivoomCliImageAnimationCommand::SendGif {
            file_path,
            size,
            speed_in_ms,
            fit,
            rotation,
            opacity,
        } => {
            pixoo
                .send_gif_as_animation(
                    size,
                    Duration::from_millis(speed_in_ms),
                    &file_path,
                    fit,
                    rotation,
                    opacity,
                    BlendMode::default(),
                )
                .await
        }
    }
}

async fn handle_text_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    text_animation_command: DivoomCliTextAnimationCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match text_animation_command {
        DivoomCliTextAnimationCommand::Clear => pixoo.clear_all_text_area().await,

        DivoomCliTextAnimationCommand::Set(text_animation) => {
            pixoo.send_text_animation(text_animation.into()).await
        }
    }
}

async fn handle_batch_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    batch_command: DivoomCliBatchCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(
        common
            .device_address
            .as_ref()
            .expect("Device Address is not set!"),
    );

    match batch_command {
        DivoomCliBatchCommand::RunUrl { command_url } => {
            pixoo.execute_commands_from_url(command_url).await
        }
    }
}

fn serialize_to_console<Data: Serialize>(v: Data, format: DivoomCliOutputFormat) {
    let output = match format {
        DivoomCliOutputFormat::Yaml => {
            serde_yaml::to_string(&v).unwrap_or_else(|_| panic!("Serializing data failed!"))
        }
        DivoomCliOutputFormat::Json => {
            serde_json::to_string(&v).unwrap_or_else(|_| panic!("Serializing data failed!"))
        }
    };

    println!("{}", output);
}
