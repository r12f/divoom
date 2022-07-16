use super::divoom_dto_common::*;
use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// Divoom device information returned from Divoom backend service
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DivoomDeviceInfo {
    pub device_name: String,
    pub device_id: u64,
    pub device_private_ip: String,
}

/// All settings of a pixoo device.
/// Supported settings could be very different device per device, hence we scope this to pixoo device only.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DivoomPixooDeviceSettings {
    pub brightness: i32,
    pub rotation_flag: i32,
    pub clock_time: i32,
    pub gallery_time: i32,
    pub single_gallery_time: i32,
    pub power_on_channel_id: i32,
    pub gallery_show_time_flag: i32,
    pub cur_clock_id: i32,
    pub time24_flag: DivoomDeviceHourMode,
    pub temperature_mode: DivoomDeviceTemperatureUnit,
    pub gyrate_angle: DivoomDeviceRotationAngle,
    pub mirror_flag: DivoomDeviceMirrorMode,
    pub light_switch: i32,
}

/// High light mode
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceHighLightMode {
    Off,
    On,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceHighLightMode, Off: "off", On: "on");

/// Hour mode, 12-hours o 24-hours.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceHourMode {
    Hour12,
    Hour24,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceHourMode, Hour12: "12h", Hour24: "24h");

/// Mirror mode.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceMirrorMode {
    Off,
    On,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceMirrorMode, Off: "off", On: "on");

/// Temperature unit. Used in weather report.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceTemperatureUnit {
    Celsius,
    Fahrenheit,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceTemperatureUnit, Celsius: "c", Fahrenheit: "f");

/// Device screen rotation angle.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceRotationAngle {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceRotationAngle, None: "0", Rotate90: "90", Rotate180: "180", Rotate270: "270");

/// Device screen power state.
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum DivoomDeviceScreenPowerState {
    Off,
    On,
    Raw(i32),
}

impl_divoom_dto_enum_traits!(DivoomDeviceScreenPowerState, Off: "off", On: "on");
