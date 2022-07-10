use std::str::FromStr;

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

/// High light mode
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceHighLightMode {
    Off,
    On,
    Raw(i32),
}

impl FromStr for DivoomDeviceHighLightMode {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "off" => Ok(DivoomDeviceHighLightMode::Off),
            "on" => Ok(DivoomDeviceHighLightMode::On),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceHighLightMode: {}", x))?;
                Ok(DivoomDeviceHighLightMode::Raw(parsed))
            }
        }
    }
}

/// Hour mode, 12-hours o 24-hours.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceHourMode {
    Hour12,
    Hour24,
    Raw(i32),
}

impl FromStr for DivoomDeviceHourMode {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "12h" => Ok(DivoomDeviceHourMode::Hour12),
            "24h" => Ok(DivoomDeviceHourMode::Hour24),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceHourMode: {}", x))?;
                Ok(DivoomDeviceHourMode::Raw(parsed))
            }
        }
    }
}

/// Mirror mode.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceMirrorMode {
    Disable,
    Enable,
    Raw(i32),
}

impl FromStr for DivoomDeviceMirrorMode {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "off" => Ok(DivoomDeviceMirrorMode::Disable),
            "on" => Ok(DivoomDeviceMirrorMode::Enable),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceMirrorMode: {}", x))?;
                Ok(DivoomDeviceMirrorMode::Raw(parsed))
            }
        }
    }
}

/// Temperature unit. Used in weather report.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceTemperatureUnit {
    Celsius,
    Fahrenheit,
    Raw(i32),
}

impl FromStr for DivoomDeviceTemperatureUnit {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "c" => Ok(DivoomDeviceTemperatureUnit::Celsius),
            "f" => Ok(DivoomDeviceTemperatureUnit::Fahrenheit),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceTemperatureUnit: {}", x))?;
                Ok(DivoomDeviceTemperatureUnit::Raw(parsed))
            }
        }
    }
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

impl FromStr for DivoomDeviceRotationAngle {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "0" => Ok(DivoomDeviceRotationAngle::None),
            "90" => Ok(DivoomDeviceRotationAngle::Rotate90),
            "180" => Ok(DivoomDeviceRotationAngle::Rotate180),
            "270" => Ok(DivoomDeviceRotationAngle::Rotate270),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceRotationAngle: {}", x))?;
                Ok(DivoomDeviceRotationAngle::Raw(parsed))
            }
        }
    }
}

/// Device screen power state.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum DivoomDeviceScreenPowerState {
    Off,
    On,
    Raw(i32),
}

impl FromStr for DivoomDeviceScreenPowerState {
    type Err = String;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "off" => Ok(DivoomDeviceScreenPowerState::Off),
            "on" => Ok(DivoomDeviceScreenPowerState::On),
            _ => {
                let parsed = v
                    .parse::<i32>()
                    .map_err(|x| format!("Invalid value for DivoomDeviceScreenPowerState: {}", x))?;
                Ok(DivoomDeviceScreenPowerState::Raw(parsed))
            }
        }
    }
}
