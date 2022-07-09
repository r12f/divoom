use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomServiceAPIResultDetails {
    pub return_code: i32,
    pub return_message: String,
}

pub trait DivoomServiceAPIResponse {
    fn result_details(&self) -> &DivoomServiceAPIResultDetails;

    fn error_code(&self) -> i32 {
        self.result_details().return_code
    }

    fn error_message(&self) -> &str {
        &self.result_details().return_message
    }
}

pub trait DivoomServiceAPIResponseWithPayload<T>: DivoomServiceAPIResponse {
    fn destructive_into(self) -> T;
}

macro_rules! define_divoom_service_api_response {
    ($response_type:ident, $response_payload_type:ty, $dto_type:ty) => {
        #[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $response_type {
            #[serde(flatten)]
            pub result: DivoomServiceAPIResultDetails,

            #[serde(flatten)]
            pub payload: $response_payload_type,
        }

        impl DivoomServiceAPIResponse for $response_type {
            fn result_details(&self) -> &DivoomServiceAPIResultDetails {
                &self.result
            }
        }

        impl DivoomServiceAPIResponseWithPayload<$dto_type> for $response_type {
            fn destructive_into(self) -> $dto_type {
                self.payload.destructive_into()
            }
        }
    };
}

pub(crate) use define_divoom_service_api_response;

pub const DIVOOM_SERVICE_API_URL_GET_TIME_DIAL_FONT_LIST: &str = "/Device/GetTimeDialFontList";
pub const DIVOOM_SERVICE_API_URL_GET_DIAL_LIST: &str = "/Channel/GetDialList";
pub const DIVOOM_SERVICE_API_URL_CHANNEL_GET_DIAL_TYPE: &str = "/Channel/GetDialType";
pub const DIVOOM_SERVICE_API_URL_RETURN_SAME_LAN_DEVICE: &str = "/Device/ReturnSameLANDevice";
