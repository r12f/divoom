use crate::clients::common::*;
use crate::clients::pixoo::pixoo_command_builder::PixooCommandBuilder;
use crate::divoom_contracts::pixoo::animation::*;
use crate::divoom_contracts::pixoo::batch::*;
use crate::divoom_contracts::pixoo::channel::*;
use crate::divoom_contracts::pixoo::common::*;
use crate::divoom_contracts::pixoo::system::*;
use crate::divoom_contracts::pixoo::tool::*;
use crate::dto::*;
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "animation-builder")]
use tiny_skia::BlendMode;

#[cfg(feature = "animation-builder")]
use crate::animation::*;

/// Pixoo device client
///
/// Once we have the IP address of the device, we can create the client and start to execute command directly:
///
/// ```rust
/// use divoom::*;
/// let pixoo = PixooClient::new("192.168.0.123");
/// // let result = pixoo.get_current_channel().await?;
/// // println!("{:?}", result);
/// ```
pub struct PixooClient {
    client: Arc<DivoomRestAPIClient>,
}

macro_rules! impl_pixoo_client_api {
    (
        $api_name:ident, $api_doc_path:literal, $resp_type:ty, $resp_return_type:ty
    ) => (
        #[doc = include_str!($api_doc_path)]
        pub async fn $api_name(&self) -> DivoomAPIResult<$resp_return_type> {
            let response: $resp_type = PixooCommandBuilder::start(self.client.clone())
                .$api_name()
                .execute_with_parsed_response::<$resp_type>()
                .await?;

            let error_code = response.error_code();
            if error_code != 0 {
                return Err(DivoomAPIError::ServerError(DivoomServerErrorInfo::server_error(error_code)));
            }

            Ok(response.destructive_into())
        }
    );

    (
        $api_name:ident, $api_doc_path:literal, $resp_type:ty, $resp_return_type:ty, $($api_arg:ident: $api_arg_type:ty),*
    ) => (
        #[doc = include_str!($api_doc_path)]
        pub async fn $api_name(&self, $($api_arg: $api_arg_type),*) -> DivoomAPIResult<$resp_return_type> {
            let response: $resp_type = PixooCommandBuilder::start(self.client.clone())
                .$api_name($($api_arg),*)
                .execute_with_parsed_response::<$resp_type>()
                .await?;

            let error_code = response.error_code();
            if error_code != 0 {
                return Err(DivoomAPIError::ServerError(DivoomServerErrorInfo::server_error(error_code)));
            }

            Ok(response.destructive_into())
        }
    )
}

/// Ctor
impl PixooClient {
    /// Create new PixooClient
    pub fn new(device_address: &str) -> PixooClient {
        PixooClient::with_options(device_address, None)
    }

    /// Create new PixooClient with options
    pub fn with_options(device_address: &str, timeout: Option<Duration>) -> PixooClient {
        PixooClient {
            client: Arc::new(DivoomRestAPIClient::new(
                format!("http://{}", device_address),
                timeout,
            )),
        }
    }
}

/// # Chanel API implementations
impl PixooClient {
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

    /// ## Batch mode
    /// This function returns the command builder, which allows us to build multiple commands and execute them at once.
    pub fn start_batch(&self) -> PixooCommandBuilder {
        PixooCommandBuilder::start_batch(self.client.clone())
    }
}

/// # Raw API implementation
impl PixooClient {
    pub async fn send_raw_request(&self, request: String) -> DivoomAPIResult<String> {
        let response: String = PixooCommandBuilder::start(self.client.clone())
            .send_raw_request(request)
            .execute_with_raw_response()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn pixoo_client_batch_mode_should_work() {
        let _m = mockito::mock("POST", "/post")
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=UTF-8")
            .with_header("Server", "nginx")
            .with_body("{\"error_code\": 0}")
            .create();

        let pixoo = PixooClient::new(&mockito::server_address().to_string());
        pixoo
            .start_batch()
            .set_device_rotation_angle(DivoomDeviceRotationAngle::Rotate90)
            .set_device_mirror_mode(DivoomDeviceMirrorMode::On)
            .set_device_brightness(30)
            .execute()
            .await
            .expect("Request should succeed.");
    }
}
