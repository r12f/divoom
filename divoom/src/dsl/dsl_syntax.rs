use crate::dto::*;
use clap::{Args, Parser, Subcommand};

#[cfg(feature = "animation-builder")]
use crate::animation::*;

#[derive(Parser, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum DivoomDeviceCommand {
    #[clap(subcommand, about = "Channel related APIs")]
    Channel(DivoomDeviceChannelCommand),

    #[clap(subcommand, about = "System/device related APIs")]
    System(DivoomDeviceSystemCommand),

    #[clap(subcommand, about = "APIs to launch some tools")]
    Tool(DivoomDeviceToolCommand),

    #[clap(subcommand, about = "Animation related APIs")]
    Animation(DivoomDeviceAnimationCommand),

    #[clap(subcommand, about = "Batch related APIs")]
    Batch(DivoomDeviceBatchCommand),

    #[clap(about = "Sending raw request")]
    Raw {
        #[clap(
            help = "Raw request body. Should be a valid JSON payload. Please refer to divoom's official API doc to check the format."
        )]
        request: String,
    },
}

#[derive(Subcommand, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum DivoomDeviceChannelCommand {
    #[clap(about = "Set current channel")]
    Set {
        #[clap(help = "Channel type. It can be clock, cloud, visualizer and customPage.")]
        channel_type: DivoomChannelType,
    },

    #[clap(about = "Set current channel to clock")]
    SetClock {
        #[clap(help = "Clock id.")]
        clock_id: i32,
    },

    #[clap(about = "Set current channel to cloud channel")]
    SetCloudChannel {
        #[clap(help = "Cloud channel type. It can be gallery, fav and artist.")]
        channel_type: DivoomCloudChannelType,
    },

    #[clap(about = "Set current channel to custom page")]
    SetCustomPage {
        #[clap(help = "Custom page index. Can be 0-2.")]
        page_index: i32,
    },

    #[clap(about = "Set current channel to visualizer")]
    SetVisualizer {
        #[clap(help = "Visualizer index. Starting from 0.")]
        visualizer_index: i32,
    },
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceSystemCommand {
    #[clap(about = "Set device brightness")]
    SetBrightness {
        #[clap(help = "Brightness (0-100)")]
        brightness: i32,
    },

    #[clap(about = "Set device time by UTC timestamp")]
    SetTime {
        #[clap(help = "Unix timestamp in UTC (in seconds)")]
        utc: u64,
    },

    #[clap(about = "Set device high light mode")]
    SetHighLightMode {
        #[clap(help = "High light mode. Can be on or off")]
        mode: DivoomDeviceHighLightMode,
    },

    #[clap(about = "Set device hour mode")]
    SetHourMode {
        #[clap(help = "Hour mode. Can be 12h or 24h")]
        mode: DivoomDeviceHourMode,
    },

    #[clap(about = "Set device mirror mode")]
    SetMirrorMode {
        #[clap(help = "Mirror mode. Can be on or off")]
        mode: DivoomDeviceMirrorMode,
    },

    #[clap(about = "Set device rotation angle")]
    SetRotationAngle {
        #[clap(help = "Screen rotation angle. Can be 0, 90, 180 and 270")]
        mode: DivoomDeviceRotationAngle,
    },

    #[clap(about = "Set device screen power state")]
    SetScreenPowerState {
        #[clap(help = "Screen power state, can be on or off")]
        power_state: DivoomDeviceScreenPowerState,
    },

    #[clap(about = "Set device temperature unit")]
    SetTemperatureUnit {
        #[clap(help = "Screen power state, can be c or f")]
        unit: DivoomDeviceTemperatureUnit,
    },

    #[clap(about = "Set device time zone")]
    SetTimeZone {
        #[clap(help = "Name of time zone")]
        time_zone: String,
    },

    #[clap(about = "Set device weather area")]
    SetWeatherArea {
        #[clap(allow_hyphen_values = true, help = "longitude")]
        longitude: String,

        #[clap(allow_hyphen_values = true, help = "latitude")]
        latitude: String,
    },

    #[clap(about = "Set device white balance")]
    SetWhiteBalance {
        #[clap(help = "Red, 0-255")]
        r: i32,

        #[clap(help = "Green, 0-255")]
        g: i32,

        #[clap(help = "Blue, 0-255")]
        b: i32,
    },
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceToolCommand {
    #[clap(about = "Countdown tool")]
    Countdown {
        #[clap(help = "Action, can be start, stop")]
        action: DivoomToolCountdownAction,

        #[clap(value_parser, default_value_t = 0, help = "Number of minutes, 0-59")]
        minute: i32,

        #[clap(value_parser, default_value_t = 0, help = "Number of seconds, 0-59")]
        second: i32,
    },

    #[clap(about = "Noise tool")]
    Noise {
        #[clap(help = "Action, can be start, stop")]
        action: DivoomToolNoiseAction,
    },

    #[clap(about = "Scoreboard tool")]
    Scoreboard {
        #[clap(help = "Score of blue team")]
        blue_score: i32,

        #[clap(help = "Score of red team")]
        red_score: i32,
    },

    #[clap(about = "Stopwatch tool")]
    Stopwatch {
        #[clap(help = "Action, can be start, stop, reset")]
        action: DivoomToolStopwatchAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceAnimationCommand {
    #[clap(subcommand, about = "Play GIF from Internet")]
    Gif(DivoomDeviceGifAnimationCommand),

    #[clap(subcommand, about = "Create image animation")]
    Image(DivoomDeviceImageAnimationCommand),

    #[clap(subcommand, about = "Create text animation")]
    Text(DivoomDeviceTextAnimationCommand),

    #[clap(about = "Play buzzer")]
    Buzzer {
        #[clap(default_value = "1000", help = "Total time to play in milliseconds")]
        play_total_time: i32,

        #[clap(
            short,
            default_value = "50",
            help = "Time to play in every buzz cycle in milliseconds"
        )]
        active_time_in_cycle: i32,

        #[clap(
            short,
            default_value = "100",
            help = "Time to off after every buzz in milliseconds"
        )]
        off_time_in_cycle: i32,
    },
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceGifAnimationCommand {
    #[clap(about = "Play gif file. Only supports 16x16, 32x32, 64x64 gifs")]
    Play(DivoomDevicePlayGifAnimationCommandArgs),
}

#[derive(Args, Debug)]
pub struct DivoomDevicePlayGifAnimationCommandArgs {
    #[clap(
        long,
        help = "Specify a local file on *pixoo device*. Only supports 16x16, 32x32, 64x64 gifs"
    )]
    pub file: Option<String>,

    #[clap(
        long,
        help = "Specify a local folder on *pixoo device*. Only supports 16x16, 32x32, 64x64 gifs"
    )]
    pub folder: Option<String>,

    #[clap(
        long,
        help = "Specify a URL from Internet. Only supports 16x16, 32x32, 64x64 gifs"
    )]
    pub url: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceImageAnimationCommand {
    #[clap(about = "Reset next animation id")]
    ResetId,

    #[cfg(feature = "animation-builder")]
    #[clap(
        about = "Send gif as animation. This is different from \"gif play\" command, which is provided directly by Divoom device. This command will create a regular animation and load the gif file and draw the frames into it in order to play it."
    )]
    RenderGif {
        #[clap(help = "Gif file path")]
        file_path: String,

        #[clap(
            default_value = "64",
            help = "Animation size in pixels. Only 16 and 32 and 64 are allowed."
        )]
        size: u32,

        #[clap(
            short,
            long = "speed",
            default_value = "100",
            help = "Animation play speed in milliseconds"
        )]
        speed_in_ms: u64,

        #[clap(
            short,
            long = "fit",
            default_value = "center",
            help = "Animation fit mode. Can be center, stretch, fitX and fitY"
        )]
        fit: DivoomDrawFitMode,

        #[clap(
            short,
            long = "rotate",
            default_value = "0.0",
            help = "Animation rotate angle"
        )]
        rotation: f32,

        #[clap(
            short,
            long = "opacity",
            default_value = "1.0",
            help = "Animation opacity"
        )]
        opacity: f32,
    },
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceTextAnimationCommand {
    #[clap(about = "Clear all text area")]
    Clear,

    #[clap(about = "Send text animation.")]
    Set(DivoomDeviceTextAnimationCommandArgs),
}

#[derive(Args, Debug)]
pub struct DivoomDeviceTextAnimationCommandArgs {
    #[clap(help = "Text id to create/update. Must be <= 20.")]
    pub text_id: i32,

    #[clap(short, default_value = "0", help = "Start position x.")]
    pub x: i32,

    #[clap(short, default_value = "0", help = "Start position y.")]
    pub y: i32,

    #[clap(
        short = 'd',
        default_value = "left",
        help = "Scroll direction, can be left, right."
    )]
    pub scroll_direction: DivoomTextAnimationScrollDirection,

    #[clap(
        short,
        default_value = "0",
        help = "0-7: font id in app. Divoom only has 8 fonts."
    )]
    pub font_index: i32,

    #[clap(
        short = 'w',
        long = "width",
        default_value = "16",
        help = "Text size. Must be >= 16 and <= 64."
    )]
    pub text_width: i32,

    #[clap(
        short = 's',
        long = "speed",
        default_value = "100",
        help = "Speed of each animation step (scroll) in milliseconds."
    )]
    pub speed_in_ms: i32,

    #[clap(help = "Text data")]
    pub text_string: String,

    #[clap(short, default_value = "255", help = "Font color, red.")]
    pub r: u8,

    #[clap(short, default_value = "255", help = "Font color, green.")]
    pub g: u8,

    #[clap(short, default_value = "255", help = "Font color, blue.")]
    pub b: u8,

    #[clap(
        short = 'a',
        default_value = "middle",
        help = "Text align. Can be left, middle, right."
    )]
    pub align: DivoomTextAnimationAlign,
}

impl DivoomDeviceTextAnimationCommandArgs {
    pub fn as_animation(&self) -> DivoomTextAnimation {
        DivoomTextAnimation {
            text_id: self.text_id,
            x: self.x,
            y: self.y,
            scroll_direction: self.scroll_direction,
            font_index: self.font_index,
            text_width: self.text_width,
            speed_in_ms: self.speed_in_ms,
            text_string: self.text_string.to_string(),
            color: rgb::RGB8::new(self.r, self.g, self.b),
            align: self.align,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum DivoomDeviceBatchCommand {
    #[clap(about = "Run commands from a URL")]
    RunUrl {
        #[clap(help = "URL to the command list file")]
        command_url: String,
    },
}
