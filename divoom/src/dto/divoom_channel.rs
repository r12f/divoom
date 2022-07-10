use super::divoom_dto_common::*;
use std::fmt;
use std::str::FromStr;

/// Divoom device channel types.
///
/// It can be Clock, CloudChannel, Visualizer, CustomPage. And in case we didn't have things covered, we provided `Raw(i32)` to allow us to set it
/// to any values.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomChannelType {
    Clock,
    CloudChannel,
    Visualizer,
    CustomPage,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomChannelType, Clock: "clock", CloudChannel: "cloud", Visualizer: "visualizer", CustomPage: "custom");

/// Clock info list that returned from Divoom service (not device).
/// The result will be paginated, hence we have to `total_num` field here to help query with pagination.
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomClockInfoPage {
    pub total_num: i32,
    pub dial_list: Vec<DivoomClockInfo>,
}

/// Clock info that returned from Divoom service (not device).
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomClockInfo {
    pub clock_id: i32,
    pub name: String,
}

/// Clock info that returned from Divoom device, such as Pixoo-64 (not service).
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomSelectedClockInfo {
    pub clock_id: i32,
    pub brightness: i32,
}

/// The sub channel type of cloud channel.
/// Same as the top level channel, we provided `Raw(i32)` to help us setting to any value in case we didn't have it covered here.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomCloudChannelType {
    Gallery,
    Fav,
    Artist,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomCloudChannelType, Gallery: "gallery", Fav: "fav", Artist: "artist");
