use super::api_server_dto::*;
use divoom::*;
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, Tags};
use std::io::Cursor;
use std::time::Duration;
use tiny_skia::BlendMode;

pub struct ApiHandler {
    device_address: String,
}

#[derive(Tags)]
enum ApiTags {
    Channel,
    System,
    Tool,
    Animation,
    Batch,
}

// Cannot reduce code with macro here, because we are running into compile error below:
//
// ```
// error: cannot find attribute `oai` in this scope
// --> divoom_gateway\src\server\api_handler.rs:75:11
// |
// 75 |         #[oai(path = "/channel", method = "get", tag = "ApiTags::Channel")]
// |           ^^^
// ```
//
// Sample code below:
//
// ```
// macro_rules! impl_gateway_api_get_as_string_handler {
//     (
//         $(#[$docs:meta])* $api_name:ident
//     ) => (
//         $(#[$docs])*
//         async fn $api_name(&self) -> GatewayResponse<String> {
//             let pixoo = PixooClient::new(&self.device_address);
//
//             match pixoo.$api_name().await {
//                 Err(e) => e.into(),
//                 Ok(result) => {
//                     GatewayResponse::Ok(Json(GatewayResponseExtDto::ok_with_data(result.to_string())))
//                 }
//             }
//         }
//     );
// }
//
// impl_gateway_api_get_as_string_handler!(
//     #[oai(path = "/channel", method = "get", tag = "ApiTags::Channel")]
//     get_current_channel
// );
// ```

macro_rules! parse_gateway_api_arg {
    ($request:ident, $arg_name:ident, $arg_type:ident) => {
        match $request
            .$arg_name
            .parse::<$arg_type>()
            .map_err(|_| DivoomAPIError::ParameterError(stringify!($arg_name).into()))
        {
            Err(e) => return e.into(),
            Ok(parsed) => parsed,
        }
    };
}

macro_rules! invoke_pixoo_api_no_response {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name().await {
            Err(e) => return e.into(),
            Ok(_) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok())),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(_) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok())),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_string {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok_with_data(result.to_string()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok_with_data(result.to_string()))),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_object {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok_with_data(result.into()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).unwrap().$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok_with_data(result.into()))),
        }
    )
}

#[OpenApi]
impl ApiHandler {
    pub fn new(device_address: String) -> ApiHandler {
        ApiHandler { device_address }
    }

    #[oai(path = "/channel", method = "put", tag = "ApiTags::Channel")]
    async fn select_channel(
        &self,
        request: Json<DivoomGatewaySelectChannelRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_channel = parse_gateway_api_arg!(request, channel, DivoomChannelType);
        return invoke_pixoo_api_no_response!(self, select_channel, parsed_channel);
    }

    #[oai(path = "/channel", method = "get", tag = "ApiTags::Channel")]
    async fn get_current_channel(&self) -> DivoomGatewayResponse<String> {
        return invoke_pixoo_api_respond_string!(self, get_current_channel);
    }

    #[oai(path = "/channel/clock", method = "put", tag = "ApiTags::Channel")]
    async fn select_clock(
        &self,
        request: Json<DivoomGatewaySelectClockRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySelectClockRequest { id } = request.0;
        return invoke_pixoo_api_no_response!(self, select_clock, id);
    }

    #[oai(path = "/channel/clock", method = "get", tag = "ApiTags::Channel")]
    async fn get_selected_clock_info(
        &self,
    ) -> DivoomGatewayResponse<DivoomGatewayGetSelectedClockInfoResponse> {
        return invoke_pixoo_api_respond_object!(self, get_selected_clock_info);
    }

    #[oai(path = "/channel/cloud", method = "put", tag = "ApiTags::Channel")]
    async fn select_cloud_channel(
        &self,
        request: Json<DivoomGatewaySelectCloudChannelRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_channel_type = parse_gateway_api_arg!(request, channel, DivoomCloudChannelType);
        return invoke_pixoo_api_no_response!(self, select_cloud_channel, parsed_channel_type);
    }

    #[oai(path = "/channel/visualizer", method = "put", tag = "ApiTags::Channel")]
    async fn select_visualizer(
        &self,
        request: Json<DivoomGatewaySelectVisualizerRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySelectVisualizerRequest { id } = request.0;
        return invoke_pixoo_api_no_response!(self, select_visualizer, id);
    }

    #[oai(path = "/channel/custom", method = "put", tag = "ApiTags::Channel")]
    async fn select_custom_page(
        &self,
        request: Json<DivoomGatewaySelectCustomPageRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySelectCustomPageRequest { id } = request.0;
        return invoke_pixoo_api_no_response!(self, select_custom_page, id);
    }

    #[oai(
        path = "/system/device-settings",
        method = "get",
        tag = "ApiTags::System"
    )]
    async fn get_device_settings(
        &self,
    ) -> DivoomGatewayResponse<DivoomGatewayGetDeviceSettingsResponse> {
        return invoke_pixoo_api_respond_object!(self, get_device_settings);
    }

    #[oai(path = "/system/device-time", method = "get", tag = "ApiTags::System")]
    async fn get_device_time(&self) -> DivoomGatewayResponse<u64> {
        return invoke_pixoo_api_respond_object!(self, get_device_time);
    }

    #[oai(path = "/system/brightness", method = "put", tag = "ApiTags::System")]
    async fn set_device_brightness(
        &self,
        request: Json<DivoomGatewaySetDeviceBrightnessRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySetDeviceBrightnessRequest { brightness } = request.0;
        return invoke_pixoo_api_no_response!(self, set_device_brightness, brightness);
    }

    #[oai(path = "/system/device-time", method = "put", tag = "ApiTags::System")]
    async fn set_device_time(
        &self,
        request: Json<DivoomGatewaySetDeviceTimeRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_device_time = request.time;
        return invoke_pixoo_api_no_response!(self, set_device_time, parsed_device_time);
    }

    #[oai(
        path = "/system/high-light-mode",
        method = "put",
        tag = "ApiTags::System"
    )]
    async fn set_device_high_light_mode(
        &self,
        request: Json<DivoomGatewaySetDeviceHighLightModeRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_high_light_mode =
            parse_gateway_api_arg!(request, mode, DivoomDeviceHighLightMode);
        return invoke_pixoo_api_no_response!(
            self,
            set_device_high_light_mode,
            parsed_high_light_mode
        );
    }

    #[oai(path = "/system/hour-mode", method = "put", tag = "ApiTags::System")]
    async fn set_device_hour_mode(
        &self,
        request: Json<DivoomGatewaySetDeviceHourModeRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_hour_mode = parse_gateway_api_arg!(request, mode, DivoomDeviceHourMode);
        return invoke_pixoo_api_no_response!(self, set_device_hour_mode, parsed_hour_mode);
    }

    #[oai(path = "/system/mirror-mode", method = "put", tag = "ApiTags::System")]
    async fn set_device_mirror_mode(
        &self,
        request: Json<DivoomGatewaySetDeviceMirrorModeRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_mirror_mode = parse_gateway_api_arg!(request, mode, DivoomDeviceMirrorMode);
        return invoke_pixoo_api_no_response!(self, set_device_mirror_mode, parsed_mirror_mode);
    }

    #[oai(
        path = "/system/rotation-angle",
        method = "put",
        tag = "ApiTags::System"
    )]
    async fn set_device_rotation_angle(
        &self,
        request: Json<DivoomGatewaySetDeviceRotationAngleRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_rotation_angle =
            parse_gateway_api_arg!(request, mode, DivoomDeviceRotationAngle);
        return invoke_pixoo_api_no_response!(
            self,
            set_device_rotation_angle,
            parsed_rotation_angle
        );
    }

    #[oai(
        path = "/system/screen-power-state",
        method = "put",
        tag = "ApiTags::System"
    )]
    async fn set_device_screen_power_state(
        &self,
        request: Json<DivoomGatewaySetDeviceScreenPowerStateRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_screen_power_state =
            parse_gateway_api_arg!(request, state, DivoomDeviceScreenPowerState);
        return invoke_pixoo_api_no_response!(
            self,
            set_device_screen_power_state,
            parsed_screen_power_state
        );
    }

    #[oai(
        path = "/system/temperature-unit",
        method = "put",
        tag = "ApiTags::System"
    )]
    async fn set_device_temperature_unit(
        &self,
        request: Json<DivoomGatewaySetDeviceTemperatureUnitRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_temperature_unit =
            parse_gateway_api_arg!(request, unit, DivoomDeviceTemperatureUnit);
        return invoke_pixoo_api_no_response!(
            self,
            set_device_temperature_unit,
            parsed_temperature_unit
        );
    }

    #[oai(path = "/system/time-zone", method = "put", tag = "ApiTags::System")]
    async fn set_device_time_zone(
        &self,
        request: Json<DivoomGatewaySetDeviceTimeZoneRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySetDeviceTimeZoneRequest {
            time_zone: parsed_time_zone,
        } = request.0;
        return invoke_pixoo_api_no_response!(self, set_device_time_zone, parsed_time_zone);
    }

    #[oai(path = "/system/weather_area", method = "put", tag = "ApiTags::System")]
    async fn set_device_weather_area(
        &self,
        request: Json<DivoomGatewaySetDeviceWeatherAreaRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySetDeviceWeatherAreaRequest {
            longitude: parsed_longitude,
            latitude: parsed_latitude,
        } = request.0;
        return invoke_pixoo_api_no_response!(
            self,
            set_device_weather_area,
            parsed_longitude,
            parsed_latitude
        );
    }

    #[oai(
        path = "/system/white-balance",
        method = "put",
        tag = "ApiTags::System"
    )]
    async fn set_device_white_balance(
        &self,
        request: Json<DivoomGatewaySetDeviceWhiteBalanceRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySetDeviceWhiteBalanceRequest { r, g, b } = request.0;
        return invoke_pixoo_api_no_response!(self, set_device_white_balance, r, g, b);
    }

    #[oai(path = "/tool/countdown", method = "post", tag = "ApiTags::Tool")]
    async fn set_countdown_tool(
        &self,
        request: Json<DivoomGatewaySetCountdownToolRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_action = parse_gateway_api_arg!(request, action, DivoomToolCountdownAction);
        let DivoomGatewaySetCountdownToolRequest { minute, second, .. } = request.0;
        return invoke_pixoo_api_no_response!(
            self,
            set_countdown_tool,
            minute,
            second,
            parsed_action
        );
    }

    #[oai(path = "/tool/noise", method = "post", tag = "ApiTags::Tool")]
    async fn set_noise_tool(
        &self,
        request: Json<DivoomGatewaySetNoiseToolRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_action = parse_gateway_api_arg!(request, action, DivoomToolNoiseAction);
        return invoke_pixoo_api_no_response!(self, set_noise_tool, parsed_action);
    }

    #[oai(path = "/tool/scoreboard", method = "post", tag = "ApiTags::Tool")]
    async fn set_scoreboard_tool(
        &self,
        request: Json<DivoomGatewaySetScoreboardToolRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewaySetScoreboardToolRequest {
            blue_score,
            red_score,
            ..
        } = request.0;
        return invoke_pixoo_api_no_response!(self, set_scoreboard_tool, blue_score, red_score);
    }

    #[oai(path = "/tool/stopwatch", method = "put", tag = "ApiTags::Tool")]
    async fn set_stopwatch_tool(
        &self,
        request: Json<DivoomGatewaySetStopwatchToolRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_action = parse_gateway_api_arg!(request, action, DivoomToolStopwatchAction);
        return invoke_pixoo_api_no_response!(self, set_stopwatch_tool, parsed_action);
    }

    #[oai(
        path = "/animation/play-gif",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn play_gif_file(
        &self,
        request: Json<DivoomGatewayPlayGifRequest>,
    ) -> DivoomGatewayResponse<String> {
        let parsed_file_type =
            parse_gateway_api_arg!(request, file_type, DivoomFileAnimationSourceType);
        let DivoomGatewayPlayGifRequest {
            file_type: _,
            file_name,
        } = request.0;
        return invoke_pixoo_api_no_response!(self, play_gif_file, parsed_file_type, file_name);
    }

    #[oai(
        path = "/animation/send-gif",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn send_gif_as_animation(
        &self,
        request: DivoomGatewaySendGifAsAnimationRequest,
    ) -> DivoomGatewayResponse<String> {
        let gif_data = match request.file.into_vec().await {
            Err(_) => return DivoomAPIError::ParameterError("file".to_string()).into(),
            Ok(v) => v,
        };

        let animation_builder = match DivoomAnimationBuilder::new(
            request.canvas_size,
            Duration::from_millis(request.speed_in_ms),
        ) {
            Err(e) => {
                return DivoomGatewayResponse::BadRequest(Json(
                    DivoomGatewayResponsePayload::error(format!("{:?}", e)),
                ))
            }
            Ok(v) => v,
        };

        let gif = match DivoomAnimationResourceLoader::from_gif(Cursor::new(gif_data)) {
            Err(e) => {
                return DivoomGatewayResponse::BadRequest(Json(
                    DivoomGatewayResponsePayload::error(format!("Invalid uploaded file: {:?}", e)),
                ))
            }
            Ok(v) => v,
        };

        let animation = animation_builder
            .draw_frames_fit(
                &gif,
                0,
                DivoomDrawFitMode::Center,
                0.0,
                1.0,
                BlendMode::default(),
            )
            .build();

        let pixoo = PixooClient::new(&self.device_address).unwrap();
        match pixoo.send_image_animation(animation).await {
            Err(e) => e.into(),
            Ok(_) => DivoomGatewayResponse::Ok(Json(DivoomGatewayResponsePayload::ok())),
        }
    }

    #[oai(
        path = "/animation/next-id",
        method = "get",
        tag = "ApiTags::Animation"
    )]
    async fn get_next_animation_id(&self) -> DivoomGatewayResponse<i32> {
        return invoke_pixoo_api_respond_object!(self, get_next_animation_id);
    }

    #[oai(
        path = "/animation/reset-id",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn reset_next_animation_id(&self) -> DivoomGatewayResponse<String> {
        return invoke_pixoo_api_no_response!(self, reset_next_animation_id);
    }

    #[oai(
        path = "/animation/clear-all-text",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn clear_all_text_area(&self) -> DivoomGatewayResponse<String> {
        return invoke_pixoo_api_no_response!(self, clear_all_text_area);
    }

    #[oai(
        path = "/animation/send-text",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn send_text_animation(
        &self,
        request: Json<DivoomGatewaySendTextAnimationRequest>,
    ) -> DivoomGatewayResponse<String> {
        let animation: DivoomTextAnimation = match request.0.into() {
            Err(e) => return DivoomAPIError::ParameterError(e).into(),
            Ok(v) => v,
        };

        return invoke_pixoo_api_no_response!(self, send_text_animation, animation);
    }

    #[oai(
        path = "/animation/play-buzzer",
        method = "post",
        tag = "ApiTags::Animation"
    )]
    async fn play_buzzer(
        &self,
        request: Json<DivoomGatewayPlayBuzzerRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewayPlayBuzzerRequest {
            play_total_time,
            active_time_in_cycle,
            off_time_in_cycle,
        } = request.0;
        return invoke_pixoo_api_no_response!(
            self,
            play_buzzer,
            play_total_time,
            active_time_in_cycle,
            off_time_in_cycle
        );
    }

    #[oai(
        path = "/batch/execute-commands-from-url",
        method = "post",
        tag = "ApiTags::Batch"
    )]
    async fn execute_commands_from_url(
        &self,
        request: Json<DivoomGatewayExecuteCommandsFromUrlRequest>,
    ) -> DivoomGatewayResponse<String> {
        let DivoomGatewayExecuteCommandsFromUrlRequest { url } = request.0;
        return invoke_pixoo_api_no_response!(self, execute_commands_from_url, url);
    }
}
