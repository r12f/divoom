use crate::clients::common::DivoomRestAPIClient;
use crate::clients::pixoo::pixoo_command_store::*;
use crate::divoom_contracts::pixoo::animation::*;
use crate::divoom_contracts::pixoo::batch::*;
use crate::divoom_contracts::pixoo::channel::*;
use crate::divoom_contracts::pixoo::common::*;
use crate::divoom_contracts::pixoo::system::*;
use crate::divoom_contracts::pixoo::tool::*;
use crate::*;
use crate::{DivoomAPIError, DivoomAPIResult};
use serde::de::DeserializeOwned;
use std::rc::Rc;

/// Pixoo command builder for creating the JSON payload of Pixoo commands.
pub struct PixooCommandBuilder {
    command_store: Box<dyn PixooCommandStore>,
    client: Rc<DivoomRestAPIClient>,
}

/// Constructors, builder and executor
impl PixooCommandBuilder {
    pub(crate) fn start(client: Rc<DivoomRestAPIClient>) -> PixooCommandBuilder {
        PixooCommandBuilder {
            command_store: Box::new(PixooSingleCommandStore::new()),
            client,
        }
    }

    pub(crate) fn start_batch(client: Rc<DivoomRestAPIClient>) -> PixooCommandBuilder {
        PixooCommandBuilder {
            command_store: Box::new(PixooBatchedCommandStore::new()),
            client,
        }
    }

    pub(crate) fn build(self) -> (Rc<DivoomRestAPIClient>, usize, String) {
        let (command_count, request_body) = self.command_store.to_payload();
        (self.client, command_count, request_body)
    }

    pub(crate) async fn execute_with_parsed_response<TResp: DeserializeOwned>(
        self,
    ) -> DivoomAPIResult<TResp> {
        let (client, command_count, request_body) = self.build();
        if command_count == 0 {
            return Err(DivoomAPIError::ParameterError(
                "No command is built yet!".to_string(),
            ));
        }

        client
            .send_request_with_body::<TResp>("/post", request_body)
            .await
    }

    pub async fn execute_with_raw_response(self) -> DivoomAPIResult<String> {
        let (client, command_count, request_body) = self.build();
        if command_count == 0 {
            return Err(DivoomAPIError::ParameterError(
                "No command is built yet!".to_string(),
            ));
        }

        client
            .send_raw_request_with_body("/post", request_body)
            .await
    }

    pub async fn execute(self) -> DivoomAPIResult<()> {
        if self.command_store.mode() == PixooCommandStoreMode::Single {
            return Err(DivoomAPIError::ParameterError(
                "Command builder is not running in batch mode. Please use start_batch() instead."
                    .to_string(),
            ));
        }

        let response = self
            .execute_with_parsed_response::<DivoomPixooCommandBatchExecuteCommandsResponse>()
            .await?;
        if response.error_code() != 0 {
            return Err(DivoomAPIError::ServerError(
                DivoomServerErrorInfo::server_error(response.error_code()),
            ));
        }

        Ok(())
    }
}

macro_rules! impl_command_builder {
    ($api_name:ident, $api_doc_path:literal, $req_type:ty) => (
        #[doc = include_str!($api_doc_path)]
        pub fn $api_name(mut self) -> PixooCommandBuilder {
            let request = <$req_type>::new();
            let serialized_request = serde_json::to_string(&request).expect("Serializing pixoo command failed!");
            self.command_store.append(serialized_request);
            self
        }
    );

    ($api_name:ident, $api_doc_path:literal, $req_type:ty, $req_payload_type:ty, $($api_arg:ident: $api_arg_type:ty),*) => (
        #[doc = include_str!($api_doc_path)]
        pub fn $api_name(mut self, $($api_arg: $api_arg_type),*) -> PixooCommandBuilder {
            let payload = <$req_payload_type>::new($($api_arg),*);
            let request = <$req_type>::new(payload);
            let serialized_request = serde_json::to_string(&request).expect("Serializing pixoo command failed!");
            self.command_store.append(serialized_request);
            self
        }
    )
}

/// Channel API implementations
impl PixooCommandBuilder {
    impl_command_builder!(
        select_channel,
        "../../divoom_contracts/pixoo/channel/api_select_channel.md",
        DivoomPixooCommandChannelSelectChannelRequest,
        DivoomPixooCommandChannelSelectChannelRequestPayload,
        channel_type: DivoomChannelType
    );

    impl_command_builder!(
        get_current_channel,
        "../../divoom_contracts/pixoo/channel/api_get_current_channel.md",
        DivoomPixooCommandChannelGetCurrentChannelRequest
    );

    impl_command_builder!(
        select_clock,
        "../../divoom_contracts/pixoo/channel/api_select_clock.md",
        DivoomPixooCommandChannelSelectClockRequest,
        DivoomPixooCommandChannelSelectClockRequestPayload,
        clock_id: i32
    );

    impl_command_builder!(
        get_selected_clock_info,
        "../../divoom_contracts/pixoo/channel/api_get_selected_clock_info.md",
        DivoomPixooCommandChannelGetClockInfoRequest
    );

    impl_command_builder!(
        select_cloud_channel,
        "../../divoom_contracts/pixoo/channel/api_select_cloud_channel.md",
        DivoomPixooCommandChannelSelectCloudChannelRequest,
        DivoomPixooCommandChannelSelectCloudChannelRequestPayload,
        channel_type: DivoomCloudChannelType
    );

    impl_command_builder!(
        select_visualizer,
        "../../divoom_contracts/pixoo/channel/api_select_visualizer.md",
        DivoomPixooCommandChannelSelectVisualizerRequest,
        DivoomPixooCommandChannelSelectVisualizerRequestPayload,
        visializer_index: i32
    );

    impl_command_builder!(
        select_custom_page,
        "../../divoom_contracts/pixoo/channel/api_select_custom_page.md",
        DivoomPixooCommandChannelSelectCustomPageRequest,
        DivoomPixooCommandChannelSelectCustomPageRequestPayload,
        custom_page_index: i32
    );
}

/// System API implementations
impl PixooCommandBuilder {
    impl_command_builder!(
        get_device_settings,
        "../../divoom_contracts/pixoo/system/api_get_device_settings.md",
        DivoomPixooCommandSystemGetAllSettingsRequest
    );

    impl_command_builder!(
        get_device_time,
        "../../divoom_contracts/pixoo/system/api_get_device_time.md",
        DivoomPixooCommandSystemGetDeviceTimeRequest
    );

    impl_command_builder!(
        set_device_brightness,
        "../../divoom_contracts/pixoo/system/api_set_device_brightness.md",
        DivoomPixooCommandSystemSetBrightnessRequest,
        DivoomPixooCommandSystemSetBrightnessRequestPayload,
        brightness: i32
    );

    impl_command_builder!(
        set_device_time,
        "../../divoom_contracts/pixoo/system/api_set_device_time.md",
        DivoomPixooCommandSystemSetSystemTimeRequest,
        DivoomPixooCommandSystemSetSystemTimeRequestPayload,
        utc: u64
    );

    impl_command_builder!(
        set_device_high_light_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_high_light_mode.md",
        DivoomPixooCommandSystemSetHighLightModeRequest,
        DivoomPixooCommandSystemSetHighLightModeRequestPayload,
        mode: DivoomDeviceHighLightMode
    );

    impl_command_builder!(
        set_device_hour_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_hour_mode.md",
        DivoomPixooCommandSystemSetHourModeRequest,
        DivoomPixooCommandSystemSetHourModeRequestPayload,
        mode: DivoomDeviceHourMode
    );

    impl_command_builder!(
        set_device_mirror_mode,
        "../../divoom_contracts/pixoo/system/api_set_device_mirror_mode.md",
        DivoomPixooCommandSystemSetMirrorModeRequest,
        DivoomPixooCommandSystemSetMirrorModeRequestPayload,
        mode: DivoomDeviceMirrorMode
    );

    impl_command_builder!(
        set_device_rotation_angle,
        "../../divoom_contracts/pixoo/system/api_set_device_rotation_angle.md",
        DivoomPixooCommandSystemSetRotationAngleRequest,
        DivoomPixooCommandSystemSetRotationAngleRequestPayload,
        mode: DivoomDeviceRotationAngle
    );

    impl_command_builder!(
        set_device_screen_power_state,
        "../../divoom_contracts/pixoo/system/api_set_device_screen_power_state.md",
        DivoomPixooCommandSystemSetScreenPowerStateRequest,
        DivoomPixooCommandSystemSetScreenPowerStateRequestPayload,
        power_state: DivoomDeviceScreenPowerState
    );

    impl_command_builder!(
        set_device_temperature_unit,
        "../../divoom_contracts/pixoo/system/api_set_device_temperature_unit.md",
        DivoomPixooCommandSystemSetTemperatureUnitRequest,
        DivoomPixooCommandSystemSetTemperatureUnitRequestPayload,
        unit: DivoomDeviceTemperatureUnit
    );

    impl_command_builder!(
        set_device_time_zone,
        "../../divoom_contracts/pixoo/system/api_set_device_time_zone.md",
        DivoomPixooCommandSystemSetTimeZoneRequest,
        DivoomPixooCommandSystemSetTimeZoneRequestPayload,
        time_zone: String
    );

    impl_command_builder!(
        set_device_weather_area,
        "../../divoom_contracts/pixoo/system/api_set_device_weather_area.md",
        DivoomPixooCommandSystemSetWeatherAreaRequest,
        DivoomPixooCommandSystemSetWeatherAreaRequestPayload,
        longitude: String,
        latitude: String
    );

    impl_command_builder!(
        set_device_white_balance,
        "../../divoom_contracts/pixoo/system/api_set_device_white_balance.md",
        DivoomPixooCommandSystemSetWhiteBalanceRequest,
        DivoomPixooCommandSystemSetWhiteBalanceRequestPayload,
        r: i32,
        g: i32,
        b: i32
    );
}

/// Tool API implementations
impl PixooCommandBuilder {
    impl_command_builder!(
        set_countdown_tool,
        "../../divoom_contracts/pixoo/tool/api_set_countdown_tool.md",
        DivoomPixooCommandToolSetCountdownRequest,
        DivoomPixooCommandToolSetCountdownRequestPayload,
        minute: i32,
        second: i32,
        action: DivoomToolCountdownAction
    );

    impl_command_builder!(
        set_noise_tool,
        "../../divoom_contracts/pixoo/tool/api_set_noise_tool.md",
        DivoomPixooCommandToolSetNoiseStatusRequest,
        DivoomPixooCommandToolSetNoiseStatusRequestPayload,
        action: DivoomToolNoiseAction
    );

    impl_command_builder!(
        set_scoreboard_tool,
        "../../divoom_contracts/pixoo/tool/api_set_scoreboard_tool.md",
        DivoomPixooCommandToolSetScoreboardRequest,
        DivoomPixooCommandToolSetScoreboardRequestPayload,
        blue_score: i32,
        red_score: i32
    );

    impl_command_builder!(
        set_stopwatch_tool,
        "../../divoom_contracts/pixoo/tool/api_set_stopwatch_tool.md",
        DivoomPixooCommandToolSetStopwatchRequest,
        DivoomPixooCommandToolSetStopwatchRequestPayload,
        action: DivoomToolStopwatchAction
    );
}

/// Animation API implementations
impl PixooCommandBuilder {
    impl_command_builder!(
        play_gif_file,
        "../../divoom_contracts/pixoo/animation/api_play_gif_file.md",
        DivoomPixooCommandAnimationPlayGifRequest,
        DivoomPixooCommandAnimationPlayGifRequestPayload,
        file_type: DivoomFileAnimationSourceType,
        file_name: String
    );

    impl_command_builder!(
        get_next_animation_id,
        "../../divoom_contracts/pixoo/animation/api_get_next_animation_id.md",
        DivoomPixooCommandAnimationGetNextAnimationIdRequest
    );

    impl_command_builder!(
        reset_next_animation_id,
        "../../divoom_contracts/pixoo/animation/api_reset_next_animation_id.md",
        DivoomPixooCommandAnimationResetNextAnimationIdRequest
    );

    #[doc = include_str!("../../divoom_contracts/pixoo/animation/api_send_image_animation_frame.md")]
    pub fn send_image_animation(
        mut self,
        id: i32,
        animation: DivoomImageAnimation,
    ) -> PixooCommandBuilder {
        let payloads =
            DivoomPixooCommandAnimationSendImageAnimationFrameRequestPayload::create_frames(
                id, animation,
            );
        payloads.into_iter().for_each(|payload| {
            let request = DivoomPixooCommandAnimationSendImageAnimationFrameRequest::new(payload);
            let serialized_request =
                serde_json::to_string(&request).expect("Serializing pixoo command failed!");
            self.command_store.append(serialized_request);
        });
        self
    }

    impl_command_builder!(
        send_text_animation,
        "../../divoom_contracts/pixoo/animation/api_send_text_animation.md",
        DivoomPixooCommandAnimationSendTextAnimationRequest,
        DivoomPixooCommandAnimationSendTextAnimationRequestPayload,
        animation: DivoomTextAnimation
    );

    impl_command_builder!(
        clear_all_text_area,
        "../../divoom_contracts/pixoo/animation/api_clear_all_text_area.md",
        DivoomPixooCommandAnimationClearAllTextAreaRequest
    );

    impl_command_builder!(
        play_buzzer,
        "../../divoom_contracts/pixoo/animation/api_play_buzzer.md",
        DivoomPixooCommandAnimationPlayBuzzerRequest,
        DivoomPixooCommandAnimationPlayBuzzerRequestPayload,
        play_total_time: i32,
        active_time_in_cycle: i32,
        off_time_in_cycle: i32
    );
}

/// Batch API implementations
impl PixooCommandBuilder {
    impl_command_builder!(
        execute_commands_from_url,
        "../../divoom_contracts/pixoo/batch/api_execute_commands_from_url.md",
        DivoomPixooCommandBatchExecuteCommandsFromUrlRequest,
        DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload,
        command_url: String
    );
}

/// Raw API implementations
impl PixooCommandBuilder {
    pub fn send_raw_request(mut self, request: String) -> PixooCommandBuilder {
        self.command_store.append(request);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::{env, fs};

    #[test]
    fn pixoo_command_builder_should_work_with_channel_commands_in_batch_mode() {
        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .select_channel(DivoomChannelType::Visualizer)
            .get_current_channel()
            .select_clock(1)
            .get_selected_clock_info()
            .select_cloud_channel(DivoomCloudChannelType::Fav)
            .select_visualizer(2)
            .select_custom_page(3);

        run_pixoo_command_builder_test(
            builder,
            7,
            "test_data/pixoo_command_builder_tests/channel_commands.json",
        );
    }

    #[test]
    fn pixoo_command_builder_should_work_with_system_commands_in_batch_mode() {
        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .get_device_settings()
            .get_device_time()
            .set_device_brightness(100)
            .set_device_time(1000000)
            .set_device_high_light_mode(DivoomDeviceHighLightMode::On)
            .set_device_hour_mode(DivoomDeviceHourMode::Hour24)
            .set_device_mirror_mode(DivoomDeviceMirrorMode::On)
            .set_device_rotation_angle(DivoomDeviceRotationAngle::Rotate270)
            .set_device_screen_power_state(DivoomDeviceScreenPowerState::On)
            .set_device_temperature_unit(DivoomDeviceTemperatureUnit::Fahrenheit)
            .set_device_time_zone("UTC".to_string())
            .set_device_weather_area("10.01".to_string(), "20.02".to_string())
            .set_device_white_balance(1, 2, 3);

        run_pixoo_command_builder_test(
            builder,
            13,
            "test_data/pixoo_command_builder_tests/system_commands.json",
        );
    }

    #[test]
    fn pixoo_command_builder_should_work_with_tool_commands_in_batch_mode() {
        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .set_countdown_tool(10, 20, DivoomToolCountdownAction::Start)
            .set_noise_tool(DivoomToolNoiseAction::Start)
            .set_scoreboard_tool(5, 15)
            .set_stopwatch_tool(DivoomToolStopwatchAction::Reset);

        run_pixoo_command_builder_test(
            builder,
            4,
            "test_data/pixoo_command_builder_tests/tool_commands.json",
        );
    }

    #[test]
    fn pixoo_command_builder_should_work_with_animation_commands_in_batch_mode() {
        let text_animation = DivoomTextAnimation {
            text_id: 1,
            x: 10,
            y: 20,
            scroll_direction: DivoomTextAnimationScrollDirection::Right,
            font_index: 3,
            text_width: 24,
            speed_in_ms: 100,
            text_string: "The quick brown fox jumps over the lazy dog".to_string(),
            color: rgb::RGB8 {
                r: 50,
                g: 100,
                b: 150,
            },
            align: DivoomTextAnimationAlign::Right,
        };

        let mut image_animation = DivoomImageAnimation {
            size: 64,
            frame_count: 5,
            speed_in_ms: 100,
            frames: BTreeMap::new(),
        };

        image_animation
            .frames
            .insert(0, vec![1, 2, 3]);
        image_animation
            .frames
            .insert(3, vec![4, 5, 6]);

        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .play_gif_file(
                DivoomFileAnimationSourceType::Url,
                "https://example.com/some_gif.gif".to_string(),
            )
            .get_next_animation_id()
            .reset_next_animation_id()
            .send_image_animation(1, image_animation)
            .clear_all_text_area()
            .send_text_animation(text_animation)
            .play_buzzer(100, 10, 10);

        run_pixoo_command_builder_test(
            builder,
            8,
            "test_data/pixoo_command_builder_tests/animation_commands.json",
        );
    }

    #[test]
    fn pixoo_command_builder_should_work_with_batch_commands_in_batch_mode() {
        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .execute_commands_from_url("http://example.com/commands.txt".to_string());

        run_pixoo_command_builder_test(
            builder,
            1,
            "test_data/pixoo_command_builder_tests/batch_commands.json",
        );
    }

    #[test]
    fn pixoo_command_builder_should_work_with_raw_commands_in_batch_mode() {
        let client = Rc::new(DivoomRestAPIClient::new(
            "http://192.168.0.123".to_string(),
            None,
        ));
        let builder = PixooCommandBuilder::start_batch(client)
            .send_raw_request("{ \"Command\": \"Device/SetHighLightMode\", \"Mode\": 0 }".into());

        run_pixoo_command_builder_test(
            builder,
            1,
            "test_data/pixoo_command_builder_tests/raw_commands.json",
        );
    }

    fn run_pixoo_command_builder_test(
        builder: PixooCommandBuilder,
        expected_command_count: usize,
        reference_file_path: &str,
    ) {
        let (_, command_count, request_body) = builder.build();
        assert_eq!(command_count, expected_command_count);

        let actual: serde_json::Value =
            serde_json::from_str(&request_body).expect("Parsing request body failed!");

        if env::var("DIVOOM_API_GENERATE_TEST_DATA").is_ok() {
            let formatted_request_body =
                serde_json::to_string_pretty(&actual).expect("Serialize commands failed!");
            fs::write(reference_file_path, formatted_request_body).unwrap_or_else(|_| {
                panic!(
                    "Generate test data file failed! Path = {}",
                    reference_file_path
                )
            });
            return;
        }

        let reference_commands_text =
            fs::read_to_string(reference_file_path).expect("Reading reference file failed!");
        let expected: serde_json::Value =
            serde_json::from_str(&reference_commands_text).expect("Parsing reference data failed!");
        assert_eq!(actual, expected);
    }
}
