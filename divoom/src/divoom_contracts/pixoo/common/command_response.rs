pub trait DivoomPixooCommandResponse {
    fn error_code(&self) -> i32;
}

pub trait DivoomPixooCommandResponseWithPayload<T>: DivoomPixooCommandResponse {
    fn destructive_into(self) -> T;
}

macro_rules! define_pixoo_command_response_without_payload {
    ($response_type:ident) => {
        #[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $response_type {
            #[serde(rename = "error_code")]
            pub error_code: i32,
        }

        impl DivoomPixooCommandResponse for $response_type {
            fn error_code(&self) -> i32 {
                self.error_code
            }
        }

        impl DivoomPixooCommandResponseWithPayload<()> for $response_type {
            fn destructive_into(self) -> () {}
        }
    };
}

pub(crate) use define_pixoo_command_response_without_payload;

macro_rules! define_pixoo_command_response {
    ($response_type:ident, $response_payload_type:ty, $dto_type:ty) => {
        #[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $response_type {
            #[serde(rename = "error_code")]
            pub error_code: i32,

            #[serde(flatten)]
            pub payload: $response_payload_type,
        }

        impl DivoomPixooCommandResponse for $response_type {
            fn error_code(&self) -> i32 {
                self.error_code
            }
        }

        impl DivoomPixooCommandResponseWithPayload<$dto_type> for $response_type {
            fn destructive_into(self) -> $dto_type {
                self.payload.destructive_into()
            }
        }
    };
}

pub(crate) use define_pixoo_command_response;
