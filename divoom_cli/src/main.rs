mod opt;

use crate::opt::*;
use divoom::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> DivoomAPIResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let opts = DivoomCliOptions::from_args();
    match opts.command {
        DivoomCliSubCommand::Discover => {
            let divoom = DivoomServiceClient::new();
            let devices = divoom.get_same_lan_devices().await?;
            if devices.len() == 0 {
                println!("No device is found!");
                return Ok(());
            }

            println!("{} devices are found:", devices.len());
            for device in devices {
                println!(
                    "- Id = {}, Name = {}, IP = {}",
                    device.device_id, device.device_name, device.device_private_ip
                );
            }

            println!();
        }

        DivoomCliSubCommand::Channel(channel_command) => {
            let _ = handle_channel_api(&opts.common, channel_command).await?;
        }

        DivoomCliSubCommand::Animation(animation_command) => {
            let _ = handle_animation_api(&opts.common, animation_command).await?;
        }

        _ => {}
    }

    Ok(())
}

async fn handle_channel_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    channel_command: DivoomCliChannelCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(&common.device_ip.as_ref().expect("Device IP is not set!"));

    match channel_command {
        DivoomCliChannelCommand::Get => {
            let result = pixoo.get_current_channel().await?;
            println!("{:?}", result);
        }

        DivoomCliChannelCommand::GetClock => {
            let result = pixoo.get_selected_clock_info().await?;
            println!("{:?}", result);
        }

        _ => (),
    }

    Ok(())
}

async fn handle_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    animation_command: DivoomCliAnimationCommand,
) -> DivoomAPIResult<()> {
    match animation_command {
        DivoomCliAnimationCommand::Gif(gif_animation_command) => {
            let _ = handle_gif_animation_api(common, gif_animation_command).await?;
        }

        DivoomCliAnimationCommand::Text(text_animation_command) => {
            let _ = handle_text_animation_api(common, text_animation_command).await?;
        }
    }

    Ok(())
}

async fn handle_gif_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    gif_animation_command: DivoomCliGifAnimationCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(&common.device_ip.as_ref().expect("Device IP is not set!"));

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
        }
    }

    Ok(())
}

async fn handle_text_animation_api(
    common: &DivoomCliDeviceCommandCommonOpts,
    text_animation_command: DivoomCliTextAnimationCommand,
) -> DivoomAPIResult<()> {
    let pixoo = PixooClient::new(&common.device_ip.as_ref().expect("Device IP is not set!"));

    match text_animation_command {
        DivoomCliTextAnimationCommand::Clear => {
            pixoo.clear_all_text_area().await?;
        }

        DivoomCliTextAnimationCommand::Set(text_animation) => {
            pixoo.send_text_animation(text_animation.into()).await?;
        }
    }

    Ok(())
}
