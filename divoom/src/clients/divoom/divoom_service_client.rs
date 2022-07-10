use crate::clients::common::*;
use crate::divoom_contracts::divoom::*;
use crate::dto::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

/// Divoom backend service client
///
/// This is the client that talks to Divoom online service for fetching information like device info in the same LAN etc.
///
/// It is very simple to start using it, simply creates the client without argument, and then start calling the APIs:
///
/// ```rust
/// use divoom::*;
/// let divoom = DivoomServiceClient::new();
/// // let devices = divoom.get_same_lan_devices().await?;
/// // devices.iter().for_each(|x| println!("{:?}", x));
/// ```
pub struct DivoomServiceClient {
    client: DivoomRestAPIClient,
}

impl Default for DivoomServiceClient {
    fn default() -> Self {
        DivoomServiceClient::new()
    }
}

impl DivoomServiceClient {
    pub fn new() -> DivoomServiceClient {
        DivoomServiceClient::with_server_url_base("https://app.divoom-gz.com".into())
    }

    pub fn with_server_url_base(server_url_base: String) -> DivoomServiceClient {
        DivoomServiceClient {
            client: DivoomRestAPIClient {
                server_url_base,
                http_client: reqwest::Client::new(),
            },
        }
    }

    pub fn server_url_base(&self) -> &str {
        &self.client.server_url_base
    }

    #[doc = include_str!("../../divoom_contracts/divoom/api_return_same_lan_device.md")]
    pub async fn get_same_lan_devices(&self) -> DivoomAPIResult<Vec<DivoomDeviceInfo>> {
        self.send_request::<DivoomAPIResponseReturnSameLANDevice, Vec<DivoomDeviceInfo>>(
            DIVOOM_SERVICE_API_URL_RETURN_SAME_LAN_DEVICE,
        )
        .await
    }

    #[doc = include_str!("../../divoom_contracts/divoom/api_get_clock_type.md")]
    pub async fn get_clock_type(&self) -> DivoomAPIResult<Vec<String>> {
        self.send_request::<DivoomAPIResponseGetClockType, Vec<String>>(
            DIVOOM_SERVICE_API_URL_CHANNEL_GET_DIAL_TYPE,
        )
        .await
    }

    #[doc = include_str!("../../divoom_contracts/divoom/api_get_clock_list.md")]
    pub async fn get_clock_list(
        &self,
        dial_type: String,
        page_index: i32,
    ) -> DivoomAPIResult<DivoomClockInfoPage> {
        let request_body = DivoomAPIRequestGetClockList {
            dial_type,
            page: page_index,
        };
        self.send_read_request_with_body::<DivoomAPIRequestGetClockList, DivoomAPIResponseGetClockList, DivoomClockInfoPage>(
            DIVOOM_SERVICE_API_URL_GET_DIAL_LIST,
            request_body,
        )
        .await
    }

    #[doc = include_str!("../../divoom_contracts/divoom/api_get_dial_font_list.md")]
    pub async fn get_font_list(&self) -> DivoomAPIResult<Vec<DivoomFontInfo>> {
        self.send_request::<DivoomAPIResponseGetTimeDialFontList, Vec<DivoomFontInfo>>(
            DIVOOM_SERVICE_API_URL_GET_TIME_DIAL_FONT_LIST,
        )
        .await
    }

    async fn send_request<TResp: DeserializeOwned + DivoomServiceAPIResponseWithPayload<R>, R>(
        &self,
        url_path: &str,
    ) -> DivoomAPIResult<R> {
        let response: TResp = self.client.send_request(url_path).await?;
        self.check_divoom_api_result_code(response.result_details())?;
        let result = response.destructive_into();
        Ok(result)
    }

    async fn send_read_request_with_body<
        TReq: Serialize,
        TResp: DeserializeOwned + DivoomServiceAPIResponseWithPayload<R>,
        R,
    >(
        &self,
        url_path: &str,
        request_body: TReq,
    ) -> DivoomAPIResult<R> {
        let request_body_text = serde_json::to_string(&request_body)?;
        let response: TResp = self
            .client
            .send_request_with_body(url_path, request_body_text)
            .await?;
        self.check_divoom_api_result_code(response.result_details())?;
        let result = response.destructive_into();
        Ok(result)
    }

    fn check_divoom_api_result_code(
        &self,
        result_code: &DivoomServiceAPIResultDetails,
    ) -> DivoomAPIResult<()> {
        if result_code.return_code == 0 {
            Ok(())
        } else {
            Err(DivoomAPIError::ServerError(DivoomServerErrorInfo {
                http_status_code: reqwest::StatusCode::OK.as_u16(),
                error_code: result_code.return_code,
                error_message: result_code.return_message.clone(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn divoom_service_should_have_default_server_url_base() {
        let divoom = DivoomServiceClient::new();
        assert_eq!(divoom.server_url_base(), "https://app.divoom-gz.com");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn divoom_service_should_get_same_lan_devices() {
        let _m = mockito::mock("POST", DIVOOM_SERVICE_API_URL_RETURN_SAME_LAN_DEVICE)
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=UTF-8")
            .with_header("Server", "nginx")
            .with_body("{\"ReturnCode\":0,\"ReturnMessage\":\"\",\"DeviceList\":[{\"DeviceName\":\"Pixoo\",\"DeviceId\":300000001,\"DevicePrivateIP\":\"192.168.0.2\"}]}")
            .create();

        let divoom = DivoomServiceClient::with_server_url_base(mockito::server_url());
        let devices = divoom.get_same_lan_devices().await.unwrap();
        assert_eq!(
            devices,
            vec![DivoomDeviceInfo {
                device_name: "Pixoo".into(),
                device_id: 300000001,
                device_private_ip: "192.168.0.2".into()
            }]
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn divoom_service_should_get_dial_type() {
        let _m = mockito::mock("POST", DIVOOM_SERVICE_API_URL_CHANNEL_GET_DIAL_TYPE)
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=UTF-8")
            .with_header("Server", "nginx")
            .with_body("{\"ReturnCode\":0,\"ReturnMessage\":\"\",\"DialTypeList\":[\"Social\",\"financial\",\"Game\",\"normal\",\"HOLIDAYS\",\"TOOLS\",\"Sport\",\"Custom\",\"self\"]}")
            .create();

        let divoom = DivoomServiceClient::with_server_url_base(mockito::server_url());
        let devices = divoom.get_clock_type().await.unwrap();
        assert_eq!(
            devices,
            vec![
                "Social",
                "financial",
                "Game",
                "normal",
                "HOLIDAYS",
                "TOOLS",
                "Sport",
                "Custom",
                "self"
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn divoom_service_should_get_dial_list() {
        let _m = mockito::mock("POST", DIVOOM_SERVICE_API_URL_GET_DIAL_LIST)
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=UTF-8")
            .with_header("Server", "nginx")
            .with_body("{ \"ReturnCode\": 0, \"ReturnMessage\": \"\", \"TotalNum\": 100, \"DialList\": [ { \"ClockId\": 10, \"Name\": \"Classic Digital Clock\" } ]}")
            .create();

        let divoom = DivoomServiceClient::with_server_url_base(mockito::server_url());
        let devices = divoom.get_clock_list("Social".into(), 1).await.unwrap();
        assert_eq!(
            devices,
            DivoomClockInfoPage {
                total_num: 100,
                dial_list: vec![DivoomClockInfo {
                    clock_id: 10,
                    name: "Classic Digital Clock".to_string()
                }]
            }
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn divoom_service_should_get_font_list() {
        let _m = mockito::mock("POST", DIVOOM_SERVICE_API_URL_GET_TIME_DIAL_FONT_LIST)
            .with_status(200)
            .with_header("Content-Type", "application/json; charset=UTF-8")
            .with_header("Server", "nginx")
            .with_body("{\"ReturnCode\":0,\"ReturnMessage\":\"\",\"FontList\":[{\"id\":2,\"name\":\"16*16 English letters, Arabic figures,punctuation\",\"width\":\"16\",\"high\":\"16\",\"charset\":\"\",\"type\":0}]}")
            .create();

        let divoom = DivoomServiceClient::with_server_url_base(mockito::server_url());
        let devices = divoom.get_font_list().await.unwrap();
        assert_eq!(
            devices,
            vec![DivoomFontInfo {
                id: 2,
                name: "16*16 English letters, Arabic figures,punctuation".to_string(),
                width: 16,
                height: 16,
                charset: "".to_string(),
                font_type: DivoomFontType::Scrollable
            }]
        );
    }
}
