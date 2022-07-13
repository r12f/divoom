#[doc = include_str!("./api_get_device_settings.md")]
use crate::divoom_contracts::pixoo::common::*;
use crate::DivoomPixooDeviceSettings;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request_without_payload!(
    "Channel/GetAllConf",
    DivoomPixooCommandSystemGetAllSettingsRequest
);

// Response
define_pixoo_command_response!(
    DivoomPixooCommandSystemGetAllSettingsResponse,
    DivoomPixooCommandSystemGetAllSettingsResponsePayload,
    DivoomPixooDeviceSettings
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandSystemGetAllSettingsResponsePayload {
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

impl DivoomPixooCommandSystemGetAllSettingsResponsePayload {
    pub fn destructive_into(self) -> DivoomPixooDeviceSettings {
        DivoomPixooDeviceSettings {
            brightness: self.brightness,
            rotation_flag: self.rotation_flag,
            clock_time: self.clock_time,
            gallery_time: self.gallery_time,
            single_gallery_time: self.single_galley_time,
            power_on_channel_id: self.power_on_channel_id,
            gallery_show_time_flag: self.gallery_show_time_flag,
            cur_clock_id: self.cur_clock_id,
            time24_flag: self.time24_flag.into(),
            temperature_mode: self.temperature_mode.into(),
            gyrate_angle: self.gyrate_angle.into(),
            mirror_flag: self.mirror_flag.into(),
            light_switch: self.light_switch,
        }
    }
}
