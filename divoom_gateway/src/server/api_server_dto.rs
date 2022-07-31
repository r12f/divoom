use divoom::DivoomAPIError;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use poem_openapi::{ApiResponse, Object};

#[derive(Object)]
pub struct GatewayResponseDTO<T: ParseFromJSON + ToJSON + Send + Sync> {
    error: String,
    server_status_code: i32,
    server_error_code: i32,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GatewayResponseDTO<T> {
    pub fn ok() -> Self {
        Self {
            error: "OK".to_string(),
            server_status_code: 200,
            server_error_code: 0,
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            error: "OK".to_string(),
            server_status_code: 200,
            server_error_code: 0,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            error: message,
            server_status_code: 0,
            server_error_code: 0,
            data: None,
        }
    }

    pub fn server_error(status_code: i32, server_error: i32, message: String) -> Self {
        Self {
            error: message,
            server_status_code: status_code,
            server_error_code: server_error,
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

impl<T: ParseFromJSON + ToJSON + Send + Sync> From<DivoomAPIError> for GatewayResponse<T> {
    fn from(err: DivoomAPIError) -> Self {
        match err {
            DivoomAPIError::ParameterError(e) => GatewayResponse::BadRequest(Json(
                GatewayResponseDTO::error(format!("Invalid parameter: {}", e)),
            )),
            DivoomAPIError::ResourceLoadError { source: e } => {
                GatewayResponse::BadRequest(Json(GatewayResponseDTO::error(e.to_string())))
            }
            DivoomAPIError::ResourceDecodeError(e) => {
                GatewayResponse::BadRequest(Json(GatewayResponseDTO::error(e.to_string())))
            }
            DivoomAPIError::RequestError { source: e } => {
                GatewayResponse::ServiceUnavailable(Json(GatewayResponseDTO::error(e.to_string())))
            }
            DivoomAPIError::ResponseDeserializationError { source: e } => {
                GatewayResponse::InternalServerError(Json(GatewayResponseDTO::error(e.to_string())))
            }
            DivoomAPIError::ServerError(e) => {
                GatewayResponse::BadRequest(Json(GatewayResponseDTO::server_error(
                    e.http_status_code as i32,
                    e.error_code,
                    e.error_message,
                )))
            }
        }
    }
}
