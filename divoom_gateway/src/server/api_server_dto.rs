use divoom::DivoomChannelType;
use poem::{http::StatusCode, Error, Result, Route};
use poem_openapi::error::ParseRequestPayloadError;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use poem_openapi::{param::Query, payload::PlainText, ApiResponse, Object, OpenApi, Tags};
use serde_json::Value;

#[derive(Object)]
pub struct GatewayResponseDTO<T: ParseFromJSON + ToJSON + Send + Sync> {
    error: String,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GatewayResponseDTO<T> {
    pub fn ok(data: T) -> Self {
        Self {
            error: "OK".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            error: message,
            data: None,
        }
    }
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "gateway_bad_request_handler")]
pub enum GatewayResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<GatewayResponseDTO<T>>),

    #[oai(status = 400)]
    BadRequest(Json<GatewayResponseDTO<T>>),

    #[oai(status = 404)]
    NotFound(Json<GatewayResponseDTO<T>>),

    #[oai(status = 500)]
    InternalServerError(Json<GatewayResponseDTO<T>>),

    #[oai(status = 503)]
    ServiceUnavailable(Json<GatewayResponseDTO<T>>),
}

pub fn gateway_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> GatewayResponse<T> {
    GatewayResponse::BadRequest(Json(GatewayResponseDTO::error(err.to_string())))
}

/*
impl_pixoo_client_api!(
        select_channel,
        "../../divoom_contracts/pixoo/channel/api_select_channel.md",
        DivoomPixooCommandChannelSelectChannelResponse,
        (),
        channel_type: DivoomChannelType
    );

impl_pixoo_client_api!(
        get_current_channel,
        "../../divoom_contracts/pixoo/channel/api_get_current_channel.md",
        DivoomPixooCommandChannelGetCurrentChannelResponse,
        DivoomChannelType
    );

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
