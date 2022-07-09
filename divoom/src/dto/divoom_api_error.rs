use thiserror::Error;

/// This represents the error that returned from Divoom online service or Divoom devices.
#[derive(Debug, PartialOrd, PartialEq)]
pub struct DivoomServerErrorInfo {
    /// HTTP status code returned from Divoom service or device.
    pub http_status_code: u16,

    /// Internal error code returned from Divoom service or device.
    /// Usually 0 means success and non-zero means failures.
    pub error_code: i32,

    /// Internal error message returned from Divoom service or device.
    /// Empty when error code is 0 and *maybe* not empty when error code is not 0.
    pub error_message: String,
}

impl DivoomServerErrorInfo {
    /// Create error info from error http status code.
    pub fn http_error(http_status_code: u16) -> DivoomServerErrorInfo {
        DivoomServerErrorInfo {
            http_status_code,
            error_code: 0,
            error_message: "".to_string(),
        }
    }

    /// Create error info from Divoom internal error code without message.
    pub fn server_error(error_code: i32) -> DivoomServerErrorInfo {
        DivoomServerErrorInfo {
            http_status_code: 0,
            error_code,
            error_message: "".to_string(),
        }
    }

    /// Create error info from Divoom internal error code with error message.
    pub fn server_error_with_message(
        error_code: i32,
        error_message: String,
    ) -> DivoomServerErrorInfo {
        DivoomServerErrorInfo {
            http_status_code: 0,
            error_code,
            error_message,
        }
    }
}

/// Divoom API error.
/// Since the Divoom service and device APIs are http servers, it can fail due to many reasons. Hence we have a few categories of errors here.
#[derive(Debug, Error)]
pub enum DivoomAPIError {
    #[error("Invalid parameter.")]
    ParameterError(String),

    #[error("Failed to send request")]
    RequestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Failed to deserialize the response")]
    ResponseDeserializationError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Service or device responded failure")]
    ServerError(DivoomServerErrorInfo),
}

/// Result that wraps the error.
pub type DivoomAPIResult<T> = std::result::Result<T, DivoomAPIError>;
