use divoom::*;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use poem_openapi::{ApiResponse, Object};
use serde::{Serialize, Deserialize};

#[derive(Object)]
pub struct GatewayResponseExtDto<T: ParseFromJSON + ToJSON + Send + Sync> {
    error: String,
    server_status_code: i32,
    server_error_code: i32,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GatewayResponseExtDto<T> {
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
pub enum GatewayResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<GatewayResponseExtDto<T>>),

    #[oai(status = 400)]
    BadRequest(Json<GatewayResponseExtDto<T>>),

    #[oai(status = 404)]
    NotFound(Json<GatewayResponseExtDto<T>>),

    #[oai(status = 500)]
    InternalServerError(Json<GatewayResponseExtDto<T>>),

    #[oai(status = 503)]
    ServiceUnavailable(Json<GatewayResponseExtDto<T>>),
}

pub fn gateway_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> GatewayResponse<T> {
    GatewayResponse::BadRequest(Json(GatewayResponseExtDto::error(err.to_string())))
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> From<DivoomAPIError> for GatewayResponse<T> {
    fn from(err: DivoomAPIError) -> Self {
        match err {
            DivoomAPIError::ParameterError(e) => GatewayResponse::BadRequest(Json(
                GatewayResponseExtDto::error(format!("Invalid parameter: {}", e)),
            )),
            DivoomAPIError::ResourceLoadError { source: e } => {
                GatewayResponse::BadRequest(Json(GatewayResponseExtDto::error(e.to_string())))
            }
            DivoomAPIError::ResourceDecodeError(e) => {
                GatewayResponse::BadRequest(Json(GatewayResponseExtDto::error(e.to_string())))
            }
            DivoomAPIError::RequestError { source: e } => {
                GatewayResponse::ServiceUnavailable(Json(GatewayResponseExtDto::error(e.to_string())))
            }
            DivoomAPIError::ResponseDeserializationError { source: e } => {
                GatewayResponse::InternalServerError(Json(GatewayResponseExtDto::error(e.to_string())))
            }
            DivoomAPIError::ServerError(e) => {
                GatewayResponse::BadRequest(Json(GatewayResponseExtDto::server_error(
                    e.http_status_code as i32,
                    e.error_code,
                    e.error_message,
                )))
            }
        }
    }
}

/// Clock info that returned from Divoom device, such as Pixoo-64 (not service).
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomSelectedClockInfoExtDto {
    pub clock_id: i32,
    pub brightness: i32,
}

impl From<DivoomSelectedClockInfo> for DivoomSelectedClockInfoExtDto {
    fn from(v: DivoomSelectedClockInfo) -> Self {
        DivoomSelectedClockInfoExtDto {
            clock_id: v.clock_id,
            brightness: v.brightness,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, Object)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomPixooDeviceSettingsExtDto {
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

impl From<DivoomPixooDeviceSettings> for DivoomPixooDeviceSettingsExtDto {
    fn from(v: DivoomPixooDeviceSettings) -> Self {
        DivoomPixooDeviceSettingsExtDto {
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
