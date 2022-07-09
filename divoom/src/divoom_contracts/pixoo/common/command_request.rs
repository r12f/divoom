pub trait DivoomPixooCommandRequest {
    fn command(&self) -> &str;
}

macro_rules! define_pixoo_command_request_without_payload {
    ($command:literal, $request_type:ident) => {
        #[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $request_type {
            pub command: String,
        }

        impl $request_type {
            // Somehow rust is throwing all these functions, which are definitely used, are not used.
            // My guess is the usage is defined in macro and it cannot correctly detect it, hence force marking it as allowed.
            #[allow(dead_code)]
            pub fn new() -> $request_type {
                $request_type {
                    command: $command.into(),
                }
            }
        }

        impl DivoomPixooCommandRequest for $request_type {
            fn command(&self) -> &str {
                &self.command
            }
        }
    };
}

pub(crate) use define_pixoo_command_request_without_payload;

macro_rules! define_pixoo_command_request {
    ($command:literal, $request_type:ident, $request_payload_type:ty) => {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $request_type {
            pub command: String,

            #[serde(flatten)]
            pub payload: $request_payload_type,
        }

        impl $request_type {
            // Somehow rust is throwing all these functions, which are definitely used, are not used.
            // My guess is the usage is defined in macro and it cannot correctly detect it, hence force marking it as allowed.
            #[allow(dead_code)]
            pub fn new(payload: $request_payload_type) -> $request_type {
                $request_type {
                    command: $command.into(),
                    payload,
                }
            }
        }

        impl DivoomPixooCommandRequest for $request_type {
            fn command(&self) -> &str {
                &self.command
            }
        }
    };
}

pub(crate) use define_pixoo_command_request;
