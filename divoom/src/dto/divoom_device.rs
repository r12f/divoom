/// Divoom device information returned from Divoom backend service
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomDeviceInfo {
    pub device_name: String,
    pub device_id: u64,
    pub device_private_ip: String,
}

/// All settings of a pixoo device.
/// Supported settings could be very different device per device, hence we scope this to pixoo device only.
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomPixooDeviceSettings {
    pub brightness: i32,
    pub rotation_flag: i32,
    pub clock_time: i32,
    pub gallery_time: i32,
    pub single_galley_time: i32,
    pub power_on_channel_id: i32,
    pub gallery_show_time_flag: i32,
    pub cur_clock_id: i32,
    pub time24_flag: i32,
    pub temperature_mode: i32,
    pub gyrate_angle: i32,
    pub mirror_flag: i32,
    pub light_switch: i32,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceHighLightMode {
    Off,
    On,
    Raw(i32),
}

/// Hour mode, 12-hours o 24-hours.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceHourMode {
    Hour12,
    Hour24,
    Raw(i32),
}

/// Mirror mode.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceMirrorMode {
    Disable,
    Enable,
    Raw(i32),
}

/// Temperature unit. Used in weather report.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceTemperatureUnit {
    Celsius,
    Fahrenheit,
    Raw(i32),
}

/// Device screen rotation angle.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceRotationAngle {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Raw(i32),
}

/// Device screen power state.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceScreenPowerState {
    Off,
    On,
    Raw(i32),
}
