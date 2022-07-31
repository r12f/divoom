use super::api_server_dto::*;
use divoom::*;
use poem::web::Data;
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
//                     GatewayResponse::Ok(Json(GatewayResponseExDTO::ok_with_data(result.to_string())))
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
            Ok(_) => GatewayResponse::Ok(Json(GatewayResponseExDTO::ok())),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_string {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExDTO::ok_with_data(result.to_string()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExDTO::ok_with_data(result.to_string()))),
        }
    )
}

macro_rules! invoke_pixoo_api_respond_object {
    ($self:ident, $api_name:ident) => (
        match PixooClient::new(&$self.device_address).$api_name().await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExDTO::ok_with_data(result.into()))),
        }
    );

    ($self:ident, $api_name:ident, $($api_arg:ident),*) => (
        match PixooClient::new(&$self.device_address).$api_name($($api_arg),*).await {
            Err(e) => return e.into(),
            Ok(result) => GatewayResponse::Ok(Json(GatewayResponseExDTO::ok_with_data(result.into()))),
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
    async fn get_selected_clock_info(&self) -> GatewayResponse<DivoomSelectedClockInfoExDTO> {
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
}

/*
impl_pixoo_client_api!(
        select_clock,
        "../../divoom_contracts/pixoo/channel/api_select_clock.md",
        DivoomPixooCommandChannelSelectClockResponse,
        (),
        clock_id: i32
    );

impl_pixoo_client_api!(
        get_selected_clock_info,
        "../../divoom_contracts/pixoo/channel/api_get_selected_clock_info.md",
        DivoomPixooCommandChannelGetClockInfoResponse,
        DivoomSelectedClockInfo
    );
impl_pixoo_client_api!(
        select_cloud_channel,
        "../../divoom_contracts/pixoo/channel/api_select_cloud_channel.md",
        DivoomPixooCommandChannelSelectCloudChannelResponse,
        (),
        channel_type: DivoomCloudChannelType
    );
impl_pixoo_client_api!(
        select_visualizer,
        "../../divoom_contracts/pixoo/channel/api_select_visualizer.md",
        DivoomPixooCommandChannelSelectVisualizerResponse,
        (),
        visializer_index: i32
    );
impl_pixoo_client_api!(
        select_custom_page,
        "../../divoom_contracts/pixoo/channel/api_select_custom_page.md",
        DivoomPixooCommandChannelSelectCustomPageResponse,
        (),
        custom_page_index: i32
    );
}

/// # System API implementations
impl PixooClient {
    impl_pixoo_client_api!(
        get_device_settings,
        "../../divoom_contracts/pixoo/system/api_get_device_settings.md",
        DivoomPixooCommandSystemGetAllSettingsResponse,
        DivoomPixooDeviceSettings
    );

    impl_pixoo_client_api!(
        get_device_time,
        "../../divoom_contracts/pixoo/system/api_get_device_time.md",
        DivoomPixooCommandSystemGetDeviceTimeResponse,
        u64
    );

    impl_pixoo_client_api!(
        set_device_brightness,
        "../../divoom_contracts/pixoo/system/api_set_device_brightness.md",
        DivoomPixooCommandSystemSetBrightnessResponse,
        (),
        brightness: i32
    );

    impl_pixoo_client_api!(
        set_device_time,
        "../../divoom_contracts/pixoo/system/api_set_device_time.md",
        DivoomPixooCommandSystemSetSystemTimeResponse,
        (),
        utc: u64
    );

    impl_pixoo_client_api!(
        set_device_high_light_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_high_light_mode.md",
        DivoomPixooCommandSystemSetHighLightModeResponse,
        (),
        mode: DivoomDeviceHighLightMode
    );

    impl_pixoo_client_api!(
        set_device_hour_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_hour_mode.md",
        DivoomPixooCommandSystemSetHourModeResponse,
        (),
        mode: DivoomDeviceHourMode
    );

    impl_pixoo_client_api!(
        set_device_mirror_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_mirror_mode.md",
        DivoomPixooCommandSystemSetMirrorModeResponse,
        (),
        mode: DivoomDeviceMirrorMode
    );

    impl_pixoo_client_api!(
        set_device_rotation_angle,
        "../../divoom_contracts/pixoo/system/api_set_device_rotation_angle.md",
        DivoomPixooCommandSystemSetRotationAngleResponse,
        (),
        mode: DivoomDeviceRotationAngle
    );

    impl_pixoo_client_api!(
        set_device_screen_power_state,
        "../../divoom_contracts/pixoo/system/api_set_device_screen_power_state.md",
        DivoomPixooCommandSystemSetScreenPowerStateResponse,
        (),
        power_state: DivoomDeviceScreenPowerState
    );

    impl_pixoo_client_api!(
        set_device_temperature_unit,
        "../../divoom_contracts/pixoo/system/api_set_device_temperature_unit.md",
        DivoomPixooCommandSystemSetTemperatureUnitResponse,
        (),
        unit: DivoomDeviceTemperatureUnit
    );

    impl_pixoo_client_api!(
        set_device_time_zone,
        "../../divoom_contracts/pixoo/system/api_set_device_time_zone.md",
        DivoomPixooCommandSystemSetTimeZoneResponse,
        (),
        time_zone: String
    );

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
