use std::str::FromStr;
use divoom::*;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use poem_openapi::{ApiResponse, Object, Multipart};
use poem_openapi::types::multipart::Upload;
use serde::{Serialize, Deserialize};

#[derive(Object)]
pub struct DivoomGatewayResponsePayload<T: ParseFromJSON + ToJSON + Send + Sync> {
    error: String,
    server_status_code: i32,
    server_error_code: i32,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> DivoomGatewayResponsePayload<T> {
    pub fn ok() -> Self {
        Self {
            error: "OK".to_string(),
            server_status_code: 200,
            server_error_code: 0,
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            error: "OK".to_string(),
            server_status_code: 200,
            server_error_code: 0,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            error: message,
            server_status_code: 0,
            server_error_code: 0,
            data: None,
        }
    }

    pub fn server_error(status_code: i32, server_error: i32, message: String) -> Self {
        Self {
            error: message,
            server_status_code: status_code,
            server_error_code: server_error,
            data: None,
        }
    }
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "gateway_bad_request_handler")]
pub enum DivoomGatewayResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<DivoomGatewayResponsePayload<T>>),

    #[oai(status = 400)]
    BadRequest(Json<DivoomGatewayResponsePayload<T>>),

    #[oai(status = 404)]
    NotFound(Json<DivoomGatewayResponsePayload<T>>),

    #[oai(status = 500)]
    InternalServerError(Json<DivoomGatewayResponsePayload<T>>),

    #[oai(status = 503)]
    ServiceUnavailable(Json<DivoomGatewayResponsePayload<T>>),
}

pub fn gateway_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> DivoomGatewayResponse<T> {
    DivoomGatewayResponse::BadRequest(Json(DivoomGatewayResponsePayload::error(err.to_string())))
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> From<DivoomAPIError> for DivoomGatewayResponse<T> {
    fn from(err: DivoomAPIError) -> Self {
        match err {
            DivoomAPIError::ParameterError(e) => DivoomGatewayResponse::BadRequest(Json(
                DivoomGatewayResponsePayload::error(format!("Invalid parameter: {}", e)),
            )),
            DivoomAPIError::ResourceLoadError { source: e } => {
                DivoomGatewayResponse::BadRequest(Json(DivoomGatewayResponsePayload::error(e.to_string())))
            }
            DivoomAPIError::ResourceDecodeError(e) => {
                DivoomGatewayResponse::BadRequest(Json(DivoomGatewayResponsePayload::error(e.to_string())))
            }
            DivoomAPIError::RequestError { source: e } => {
                DivoomGatewayResponse::ServiceUnavailable(Json(DivoomGatewayResponsePayload::error(e.to_string())))
            }
            DivoomAPIError::ResponseDeserializationError { source: e } => {
                DivoomGatewayResponse::InternalServerError(Json(DivoomGatewayResponsePayload::error(e.to_string())))
            }
            DivoomAPIError::ServerError(e) => {
                DivoomGatewayResponse::BadRequest(Json(DivoomGatewayResponsePayload::server_error(
                    e.http_status_code as i32,
                    e.error_code,
                    e.error_message,
                )))
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewayGetSelectedClockInfoResponse {
    pub clock_id: i32,
    pub brightness: i32,
}

impl From<DivoomSelectedClockInfo> for DivoomGatewayGetSelectedClockInfoResponse {
    fn from(v: DivoomSelectedClockInfo) -> Self {
        DivoomGatewayGetSelectedClockInfoResponse {
            clock_id: v.clock_id,
            brightness: v.brightness,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewayGetDeviceSettingsResponse {
    pub brightness: i32,
    pub rotation_flag: i32,
    pub clock_time: i32,
    pub gallery_time: i32,
    pub single_gallery_time: i32,
    pub power_on_channel_id: i32,
    pub gallery_show_time_flag: i32,
    pub cur_clock_id: i32,
    pub time24_flag: String,
    pub temperature_mode: String,
    pub gyrate_angle: String,
    pub mirror_flag: String,
    pub light_switch: i32,
}

impl From<DivoomPixooDeviceSettings> for DivoomGatewayGetDeviceSettingsResponse {
    fn from(v: DivoomPixooDeviceSettings) -> Self {
        DivoomGatewayGetDeviceSettingsResponse {
            brightness: v.brightness,
            rotation_flag: v.rotation_flag,
            clock_time: v.clock_time,
            gallery_time: v.gallery_time,
            single_gallery_time: v.single_gallery_time,
            power_on_channel_id: v.power_on_channel_id,
            gallery_show_time_flag: v.gallery_show_time_flag,
            cur_clock_id: v.cur_clock_id,
            time24_flag: v.time24_flag.to_string(),
            temperature_mode: v.temperature_mode.to_string(),
            gyrate_angle: v.gyrate_angle.to_string(),
            mirror_flag: v.mirror_flag.to_string(),
            light_switch: v.light_switch,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySelectChannelRequest {
    pub channel: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySelectClockRequest {
    pub id: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySelectCloudChannelRequest {
    pub channel: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySelectVisualizerRequest {
    pub id: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySelectCustomPageRequest {
    pub id: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceBrightnessRequest {
    pub brightness: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceTimeRequest {
    pub time: u64,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceHighLightModeRequest {
    pub mode: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceHourModeRequest {
    pub mode: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceMirrorModeRequest {
    pub mode: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceRotationAngleRequest {
    pub mode: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceScreenPowerStateRequest {
    pub state: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceTemperatureUnitRequest {
    pub unit: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceTimeZoneRequest {
    pub time_zone: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceWeatherAreaRequest {
    pub longitude: String,
    pub latitude: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetDeviceWhiteBalanceRequest {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetCountdownToolRequest {
    pub minute: i32,
    pub second: i32,
    pub action: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetNoiseToolRequest {
    pub action: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetScoreboardToolRequest {
    pub blue_score: i32,
    pub red_score: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySetStopwatchToolRequest {
    pub action: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewayExecuteCommandsFromUrlRequest {
    pub url: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewayPlayGifRequest {
    pub file_type: String,
    pub file_name: String,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewayPlayBuzzerRequest {
    pub play_total_time: i32,
    pub active_time_in_cycle: i32,
    pub off_time_in_cycle: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
pub struct DivoomGatewaySendTextAnimationRequest {
    pub text_id: i32,
    pub x: i32,
    pub y: i32,
    pub scroll_direction: String,
    pub font_index: i32,
    pub text_width: i32,
    pub speed_in_ms: i32,
    pub text_string: String,
    pub color: String,
    pub align: String,
}

impl DivoomGatewaySendTextAnimationRequest {
    pub fn into(self) -> Result<DivoomTextAnimation, String> {
        Ok(DivoomTextAnimation {
            text_id: self.text_id,
            x: self.x,
            y: self.y,
            scroll_direction: DivoomTextAnimationScrollDirection::from_str(&self.scroll_direction)?,
            font_index: self.font_index,
            text_width: self.text_width,
            speed_in_ms: self.speed_in_ms,
            text_string: self.text_string,
            color: Default::default(),
            align: DivoomTextAnimationAlign::from_str(&self.align)?,
        })
    }
}

#[derive(Debug, Multipart)]
pub struct DivoomGatewaySendGifAsAnimationRequest {
    pub canvas_size: u32,
    pub speed_in_ms: u64,
    pub file: Upload,
}
