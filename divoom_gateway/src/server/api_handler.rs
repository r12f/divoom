use super::api_server_dto::*;
use divoom::*;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{OpenApi, Tags};

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
    ($arg_name:ident, $arg_type:ident) => (
        match $arg_name.0.parse::<$arg_type>().map_err(|_| DivoomAPIError::ParameterError(stringify!($arg_name).into())) {
            Err(e) => return e.into(),
            Ok(parsed) => parsed,
        }
    )
}

macro_rules! invoke_pixoo_api_no_response {
    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(_) => GatewayResponse::Ok(Json(GatewayResponseExtDto::ok())),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_string {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExtDto::ok_with_data(result.to_string()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExtDto::ok_with_data(result.to_string()))),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_object {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExtDto::ok_with_data(result.into()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExtDto::ok_with_data(result.into()))),
        }
    )
}

#[OpenApi]
impl ApiHandler {
    pub fn new(device_address: String) -> ApiHandler {
        ApiHandler { device_address }
    }

    #[oai(path = "/channel", method = "put", tag = "ApiTags::Channel")]
    async fn select_channel(&self, channel: PlainText<String>) -> GatewayResponse<String> {
        let parsed_channel = parse_gateway_api_arg!(channel, DivoomChannelType);
        return invoke_pixoo_api_no_response!(self, select_channel, parsed_channel);
    }

    #[oai(path = "/channel", method = "get", tag = "ApiTags::Channel")]
    async fn get_current_channel(&self) -> GatewayResponse<String> {
        return invoke_pixoo_api_respond_string!(self, get_current_channel);
    }

    #[oai(path = "/channel/clock", method = "put", tag = "ApiTags::Channel")]
    async fn select_clock(&self, clock_id: PlainText<String>) -> GatewayResponse<String> {
        let parsed_clock_id = parse_gateway_api_arg!(clock_id, i32);
        return invoke_pixoo_api_no_response!(self, select_clock, parsed_clock_id);
    }

    #[oai(path = "/channel/clock", method = "get", tag = "ApiTags::Channel")]
    async fn get_selected_clock_info(&self) -> GatewayResponse<DivoomSelectedClockInfoExtDto> {
        return invoke_pixoo_api_respond_object!(self, get_selected_clock_info);
    }

    #[oai(path = "/channel/cloud", method = "put", tag = "ApiTags::Channel")]
    async fn select_cloud_channel(&self, channel_type: PlainText<String>) -> GatewayResponse<String> {
        let parsed_channel_type = parse_gateway_api_arg!(channel_type, DivoomCloudChannelType);
        return invoke_pixoo_api_no_response!(self, select_cloud_channel, parsed_channel_type);
    }

    #[oai(path = "/channel/visualizer", method = "put", tag = "ApiTags::Channel")]
    async fn select_visualizer(&self, visualizer: PlainText<String>) -> GatewayResponse<String> {
        let parsed_visualizer = parse_gateway_api_arg!(visualizer, i32);
        return invoke_pixoo_api_no_response!(self, select_visualizer, parsed_visualizer);
    }

    #[oai(path = "/channel/custom", method = "put", tag = "ApiTags::Channel")]
    async fn select_custom_page(&self, custom_page: PlainText<String>) -> GatewayResponse<String> {
        let parsed_custom_page = parse_gateway_api_arg!(custom_page, i32);
        return invoke_pixoo_api_no_response!(self, select_custom_page, parsed_custom_page);
    }

    #[oai(path = "/system/device-settings", method = "get", tag = "ApiTags::System")]
    async fn get_device_settings(&self) -> GatewayResponse<DivoomPixooDeviceSettingsExtDto> {
        return invoke_pixoo_api_respond_object!(self, get_device_settings);
    }

    #[oai(path = "/system/device-time", method = "get", tag = "ApiTags::System")]
    async fn get_device_time(&self) -> GatewayResponse<u64> {
        return invoke_pixoo_api_respond_object!(self, get_device_time);
    }

    #[oai(path = "/system/brightness", method = "put", tag = "ApiTags::System")]
    async fn set_device_brightness(&self, brightness: PlainText<String>) -> GatewayResponse<String> {
        let parsed_brightness = parse_gateway_api_arg!(brightness, i32);
        return invoke_pixoo_api_no_response!(self, set_device_brightness, parsed_brightness);
    }

    #[oai(path = "/system/device-time", method = "put", tag = "ApiTags::System")]
    async fn set_device_time(&self, device_time: PlainText<String>) -> GatewayResponse<String> {
        let parsed_device_time = parse_gateway_api_arg!(device_time, u64);
        return invoke_pixoo_api_no_response!(self, set_device_time, parsed_device_time);
    }

    #[oai(path = "/system/high-light-mode", method = "put", tag = "ApiTags::System")]
    async fn set_device_high_light_mode(&self, high_light_mode: PlainText<String>) -> GatewayResponse<String> {
        let parsed_high_light_mode = parse_gateway_api_arg!(high_light_mode, DivoomDeviceHighLightMode);
        return invoke_pixoo_api_no_response!(self, set_device_high_light_mode, parsed_high_light_mode);
    }

    #[oai(path = "/system/hour-mode", method = "put", tag = "ApiTags::System")]
    async fn set_device_hour_mode(&self, hour_mode: PlainText<String>) -> GatewayResponse<String> {
        let parsed_hour_mode = parse_gateway_api_arg!(hour_mode, DivoomDeviceHourMode);
        return invoke_pixoo_api_no_response!(self, set_device_hour_mode, parsed_hour_mode);
    }

    #[oai(path = "/system/mirror-mode", method = "put", tag = "ApiTags::System")]
    async fn set_device_mirror_mode(&self, mirror_mode: PlainText<String>) -> GatewayResponse<String> {
        let parsed_mirror_mode = parse_gateway_api_arg!(mirror_mode, DivoomDeviceMirrorMode);
        return invoke_pixoo_api_no_response!(self, set_device_mirror_mode, parsed_mirror_mode);
    }

    #[oai(path = "/system/rotation-angle", method = "put", tag = "ApiTags::System")]
    async fn set_device_rotation_angle(&self, rotation_angle: PlainText<String>) -> GatewayResponse<String> {
        let parsed_rotation_angle = parse_gateway_api_arg!(rotation_angle, DivoomDeviceRotationAngle);
        return invoke_pixoo_api_no_response!(self, set_device_rotation_angle, parsed_rotation_angle);
    }

    #[oai(path = "/system/screen-power-state", method = "put", tag = "ApiTags::System")]
    async fn set_device_screen_power_state(&self, screen_power_state: PlainText<String>) -> GatewayResponse<String> {
        let parsed_screen_power_state = parse_gateway_api_arg!(screen_power_state, DivoomDeviceScreenPowerState);
        return invoke_pixoo_api_no_response!(self, set_device_screen_power_state, parsed_screen_power_state);
    }

    #[oai(path = "/system/temperature-unit", method = "put", tag = "ApiTags::System")]
    async fn set_device_temperature_unit(&self, temperature_unit: PlainText<String>) -> GatewayResponse<String> {
        let parsed_temperature_unit = parse_gateway_api_arg!(temperature_unit, DivoomDeviceTemperatureUnit);
        return invoke_pixoo_api_no_response!(self, set_device_temperature_unit, parsed_temperature_unit);
    }

    #[oai(path = "/system/time-zone", method = "put", tag = "ApiTags::System")]
    async fn set_device_time_zone(&self, time_zone: PlainText<String>) -> GatewayResponse<String> {
        let parsed_time_zone = time_zone.0;
        return invoke_pixoo_api_no_response!(self, set_device_time_zone, parsed_time_zone);
    }

    #[oai(path = "/system/weather_area", method = "put", tag = "ApiTags::System")]
    async fn set_device_weather_area(&self, longitude: Json<String>, latitude: Json<String>) -> GatewayResponse<String> {
        let parsed_longitude = longitude.0;
        let parsed_latitude = latitude.0;
        return invoke_pixoo_api_no_response!(self, set_device_weather_area, parsed_longitude, parsed_latitude);
    }
}

/*
/// # System API implementations
impl PixooClient {
    impl_pixoo_client_api!(
        set_device_weather_area,
        "../../divoom_contracts/pixoo/system/api_set_device_weather_area.md",
        DivoomPixooCommandSystemSetWeatherAreaResponse,
        (),
        longitude: String,
        latitude: String
    );

    impl_pixoo_client_api!(
        set_device_white_balance,
        "../../divoom_contracts/pixoo/system/api_set_device_white_balance.md",
        DivoomPixooCommandSystemSetWhiteBalanceResponse,
        (),
        r: i32,
        g: i32,
        b: i32
    );
}

/// # Tool API implementations
impl PixooClient {
    impl_pixoo_client_api!(
        set_countdown_tool,
        "../../divoom_contracts/pixoo/tool/api_set_countdown_tool.md",
        DivoomPixooCommandToolSetCountdownResponse,
        (),
        minute: i32,
        second: i32,
        action: DivoomToolCountdownAction
    );

    impl_pixoo_client_api!(
        set_noise_tool,
        "../../divoom_contracts/pixoo/tool/api_set_noise_tool.md",
        DivoomPixooCommandToolSetNoiseStatusResponse,
        (),
        action: DivoomToolNoiseAction
    );

    impl_pixoo_client_api!(
        set_scoreboard_tool,
        "../../divoom_contracts/pixoo/tool/api_set_scoreboard_tool.md",
        DivoomPixooCommandToolSetScoreboardResponse,
        (),
        blue_score: i32,
        red_score: i32
    );

    impl_pixoo_client_api!(
        set_stopwatch_tool,
        "../../divoom_contracts/pixoo/tool/api_set_stopwatch_tool.md",
        DivoomPixooCommandToolSetStopwatchResponse,
        (),
        action: DivoomToolStopwatchAction
    );
}

/// # Animation API implementations
impl PixooClient {
    impl_pixoo_client_api!(
        play_gif_file,
        "../../divoom_contracts/pixoo/animation/api_play_gif_file.md",
        DivoomPixooCommandAnimationPlayGifResponse,
        (),
        file_type: DivoomFileAnimationSourceType,
        file_name: String
    );

    impl_pixoo_client_api!(
        get_next_animation_id,
        "../../divoom_contracts/pixoo/animation/api_get_next_animation_id.md",
        DivoomPixooCommandAnimationGetNextAnimationIdResponse,
        i32
    );

    impl_pixoo_client_api!(
        reset_next_animation_id,
        "../../divoom_contracts/pixoo/animation/api_reset_next_animation_id.md",
        DivoomPixooCommandAnimationResetNextAnimationIdResponse,
        ()
    );

    /// Send GIF to the device to play as an animation.
    ///
    /// This API is different from `play_gif_file`, which is provided by divoom device directly. This API will try to leverage the animation API,
    /// create a new animation, load the gif files and draw all the frames into the animation, and send the to device to play.
    ///
    /// The API `play_gif_file` doesn't seems to be very stable when the package is published, hence `send_gif_as_animation` is more preferred
    /// as of now.
    #[cfg(feature = "animation-builder")]
    pub async fn send_gif_as_animation(
        &self,
        canvas_size: u32,
        speed: Duration,
        file_path: &str,
    ) -> DivoomAPIResult<()> {
        let animation_builder = DivoomAnimationBuilder::new(canvas_size, speed)?;
        let gif = DivoomAnimationResourceLoader::from_gif_file(file_path)?;
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
        self.send_image_animation(animation).await
    }

    /// Send GIF to the device to play as an animation.
    ///
    /// This API is different from `play_gif_file`, which is provided by divoom device directly. This API will try to leverage the animation API,
    /// create a new animation, load the gif files and draw all the frames into the animation, and send the to device to play.
    ///
    /// The API `play_gif_file` doesn't seems to be very stable when the package is published, hence `send_gif_as_animation` is more preferred
    /// as of now.
    #[cfg(feature = "animation-builder")]
    pub async fn send_gif_as_animation_with_options(
        &self,
        canvas_size: u32,
        speed: Duration,
        file_path: &str,
        fit: DivoomDrawFitMode,
        rotation: f32,
        opacity: f32,
        blend: BlendMode,
    ) -> DivoomAPIResult<()> {
        let animation_builder = DivoomAnimationBuilder::new(canvas_size, speed)?;
        let gif = DivoomAnimationResourceLoader::from_gif_file(file_path)?;
        let animation = animation_builder
            .draw_frames_fit(&gif, 0, fit, rotation, opacity, blend)
            .build();
        self.send_image_animation(animation).await
    }

    #[doc = include_str!("../../divoom_contracts/pixoo/animation/api_send_image_animation_frame.md")]
    pub async fn send_image_animation(
        &self,
        animation: DivoomImageAnimation,
    ) -> DivoomAPIResult<()> {
        self.send_image_animation_with_id(DIVOOM_IMAGE_ANIMATION_ID_AUTO, animation)
            .await
    }

    #[doc = include_str!("../../divoom_contracts/pixoo/animation/api_send_image_animation_frame.md")]
    pub async fn send_image_animation_with_id(
        &self,
        id: i32,
        animation: DivoomImageAnimation,
    ) -> DivoomAPIResult<()> {
        let animation_id = if id == DIVOOM_IMAGE_ANIMATION_ID_AUTO {
            self.get_next_animation_id().await?
        } else {
            id
        };

        let response: DivoomPixooCommandBatchExecuteCommandsResponse =
            PixooCommandBuilder::start_batch(self.client.clone())
                .send_image_animation(animation_id, animation)
                .execute_with_parsed_response::<DivoomPixooCommandBatchExecuteCommandsResponse>()
                .await?;

        let error_code = response.error_code();
        if error_code != 0 {
            return Err(DivoomAPIError::ServerError(
                DivoomServerErrorInfo::server_error(error_code),
            ));
        }

        response.destructive_into();
        Ok(())
    }

    impl_pixoo_client_api!(
        send_text_animation,
        "../../divoom_contracts/pixoo/animation/api_send_text_animation.md",
        DivoomPixooCommandAnimationSendTextAnimationResponse,
        (),
        animation: DivoomTextAnimation
    );

    impl_pixoo_client_api!(
        clear_all_text_area,
        "../../divoom_contracts/pixoo/animation/api_clear_all_text_area.md",
        DivoomPixooCommandAnimationClearAllTextAreaResponse,
        ()
    );

    impl_pixoo_client_api!(
        play_buzzer,
        "../../divoom_contracts/pixoo/animation/api_play_buzzer.md",
        DivoomPixooCommandAnimationPlayBuzzerResponse,
        (),
        play_total_time: i32,
        active_time_in_cycle: i32,
        off_time_in_cycle: i32
    );
}

/// # Batch API implementations
impl PixooClient {
    impl_pixoo_client_api!(
        execute_commands_from_url,
        "../../divoom_contracts/pixoo/batch/api_execute_commands_from_url.md",
        DivoomPixooCommandBatchExecuteCommandsFromUrlResponse,
        (),
        command_url: String
    );


    impl_pixoo_client_api!(
        select_channel,
        "../../divoom_contracts/pixoo/channel/api_select_channel.md",
        DivoomPixooCommandChannelSelectChannelResponse,
        (),
        channel_type: DivoomChannelType
    );

*/
