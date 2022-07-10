use divoom::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "divoom-cli",
    author = "r12f",
    about = "https://github.com/r12f/divoom"
)]
#[structopt(rename_all = "kebab-case")]
pub struct DivoomCliOptions {
    #[structopt(flatten)]
    pub common: DivoomCliDeviceCommandCommonOpts,

    #[structopt(subcommand)]
    pub command: DivoomCliSubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct DivoomCliDeviceCommandCommonOpts {
    #[structopt(help = "Device IP. Required when using device APIs, such as \"channel get\".")]
    pub device_ip: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum DivoomCliSubCommand {
    #[structopt(about = "Discover divoom devices by calling into divoom service API")]
    Discover,

    #[structopt(about = "Channel related APIs")]
    Channel(DivoomCliChannelCommand),

    #[structopt(about = "System/device related APIs")]
    System(DivoomCliSystemCommand),

    #[structopt(about = "APIs to launch some tools")]
    Tool(DivoomCliToolCommand),

    #[structopt(about = "Animation related APIs")]
    Animation(DivoomCliAnimationCommand),

    #[structopt(about = "Batch related APIs")]
    Batch(DivoomCliBatchCommand),
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum DivoomCliChannelCommand {
    #[structopt(about = "Get current selected channel type")]
    Get,

    #[structopt(about = "Get current selected clock type")]
    GetClock,

    #[structopt(about = "Set current channel")]
    Set {
        #[structopt(
            help = "Channel type. It can be clock, cloud-channel, visualizer and custom-page."
        )]
        channel_type: DivoomChannelType,
    },

    #[structopt(about = "Set current channel to clock")]
    SetClock {
        #[structopt(help = "Clock id.")]
        clock_id: i32,
    },

    #[structopt(about = "Set current channel to cloud channel")]
    SetCloudChannel {
        #[structopt(help = "Cloud channel type. It can be gallery, fav and artist.")]
        channel_type: DivoomCloudChannelType,
    },

    #[structopt(about = "Set current channel to custom page")]
    SetCustomPage {
        #[structopt(help = "Custom page index. Can be 0-2.")]
        page_index: i32,
    },

    #[structopt(about = "Set current channel to visualizer")]
    SetVisualizer {
        #[structopt(help = "Visualizer index. Starting from 0.")]
        visualizer_index: i32,
    },
}

#[derive(StructOpt, Debug)]
pub enum DivoomCliSystemCommand {}

#[derive(StructOpt, Debug)]
pub enum DivoomCliToolCommand {}

#[derive(StructOpt, Debug)]
pub enum DivoomCliAnimationCommand {
    #[structopt(about = "Play GIF from Internet")]
    Gif(DivoomCliGifAnimationCommand),

    #[structopt(about = "Create text animation")]
    Text(DivoomCliTextAnimationCommand),
}

#[derive(StructOpt, Debug)]
pub enum DivoomCliGifAnimationCommand {
    #[structopt(about = "Play gif file")]
    Play(DivoomCliPlayGifAnimationOpts),
}

#[derive(StructOpt, Debug)]
pub struct DivoomCliPlayGifAnimationOpts {
    #[structopt(long, help = "Specify a local file on *pixoo device*")]
    pub file: Option<String>,

    #[structopt(long, help = "Specify a local folder on *pixoo device*")]
    pub folder: Option<String>,

    #[structopt(long, help = "Specify a URL from Internet")]
    pub url: Option<String>,
}

#[derive(StructOpt, Debug)]
pub enum DivoomCliTextAnimationCommand {
    #[structopt(about = "Clear all text area")]
    Clear,

    #[structopt(about = "Send text animation")]
    Set(DivoomCliTextAnimationOpts),
}

#[derive(StructOpt, Debug)]
pub struct DivoomCliTextAnimationOpts {
    #[structopt(help = "Text id to create/update. Must be <= 20.")]
    pub text_id: i32,

    #[structopt(short, default_value = "0", help = "Start position x.")]
    pub x: i32,

    #[structopt(short, default_value = "0", help = "Start position y.")]
    pub y: i32,

    #[structopt(
        short = "d",
        default_value = "left",
        help = "Scroll direction, can be left, right."
    )]
    pub scroll_direction: DivoomTextAnimationScrollDirection,

    #[structopt(
        short,
        default_value = "0",
        help = "0-7: font id in app. Divoom only has 8 fonts."
    )]
    pub font_index: i32,

    #[structopt(
        short = "w",
        long = "width",
        default_value = "16",
        help = "Text size. Must be >= 16 and <= 64."
    )]
    pub text_width: i32,

    #[structopt(
        short = "s",
        long = "speed",
        default_value = "100",
        help = "Speed of each animation step (scroll) in milliseconds."
    )]
    pub speed_in_ms: i32,

    #[structopt(help = "Text data")]
    pub text_string: String,

    #[structopt(short, default_value = "255", help = "Font color, red.")]
    pub r: u8,

    #[structopt(short, default_value = "255", help = "Font color, green.")]
    pub g: u8,

    #[structopt(short, default_value = "255", help = "Font color, blue.")]
    pub b: u8,

    #[structopt(short = "a", default_value = "middle", help = "Text align.")]
    pub align: DivoomTextAnimationAlign,
}

impl From<DivoomCliTextAnimationOpts> for DivoomTextAnimation {
    fn from(opts: DivoomCliTextAnimationOpts) -> Self {
        DivoomTextAnimation {
            text_id: opts.text_id,
            x: opts.x,
            y: opts.y,
            scroll_direction: opts.scroll_direction,
            font_index: opts.font_index,
            text_width: opts.text_width,
            speed_in_ms: opts.speed_in_ms,
            text_string: opts.text_string,
            color: rgb::RGB8::new(opts.r, opts.g, opts.b),
            align: opts.align,
        }
    }
}

#[derive(StructOpt, Debug)]
pub enum DivoomCliBatchCommand {}
