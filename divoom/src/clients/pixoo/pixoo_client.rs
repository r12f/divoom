use crate::clients::common::*;
use crate::clients::pixoo::pixoo_command_builder::PixooCommandBuilder;
use crate::divoom_contracts::pixoo::animation::*;
use crate::divoom_contracts::pixoo::batch::*;
use crate::divoom_contracts::pixoo::channel::*;
use crate::divoom_contracts::pixoo::common::*;
use crate::divoom_contracts::pixoo::system::*;
use crate::divoom_contracts::pixoo::tool::*;
use crate::dto::*;

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
    client: DivoomRestAPIClient,
}

macro_rules! impl_pixoo_client_api {
    (
        $api_name:ident, $api_doc_path:literal, $resp_type:ty, $resp_return_type:ty
    ) => (
        #[doc = include_str!($api_doc_path)]
        pub async fn $api_name(&self) -> DivoomAPIResult<$resp_return_type> {
            let response: $resp_type = PixooCommandBuilder::start()
                .$api_name()
                .execute_raw::<$resp_type>(&self.client)
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
            let response: $resp_type = PixooCommandBuilder::start()
                .$api_name($($api_arg),*)
                .execute_raw::<$resp_type>(&self.client)
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
    pub fn new(device_ip: &str) -> PixooClient {
        PixooClient {
            client: DivoomRestAPIClient {
                server_url_base: format!("http://{}", device_ip),
                http_client: reqwest::Client::new(),
            },
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

    #[doc = include_str!("../../divoom_contracts/pixoo/animation/api_send_image_animation_frame.md")]
    pub async fn send_image_animation(
        &self,
        animation: DivoomImageAnimation,
    ) -> DivoomAPIResult<()> {
        let response: DivoomPixooCommandBatchExecuteCommandsResponse =
            PixooCommandBuilder::start_batch()
                .send_image_animation(animation)
                .execute_raw::<DivoomPixooCommandBatchExecuteCommandsResponse>(&self.client)
                .await?;

        let error_code = response.error_code();
        if error_code != 0 {
            return Err(DivoomAPIError::ServerError(
                DivoomServerErrorInfo::server_error(error_code),
            ));
        }

        Ok(response.destructive_into())
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
}
