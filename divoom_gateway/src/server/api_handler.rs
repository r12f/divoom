use super::api_server_dto::*;
use divoom::*;
use poem::{http::StatusCode, Error, Result, Route};
use poem_openapi::error::ParseRequestPayloadError;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON};
use poem_openapi::{param::Query, payload::PlainText, ApiResponse, Object, OpenApi, Tags};

pub struct ApiHandler {}

#[derive(Tags)]
enum ApiTags {
    Channel,
}

#[OpenApi]
impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {}
    }

    #[oai(
        path = "/devices/:device_address/channel",
        method = "get",
        tag = "ApiTags::Channel"
    )]
    async fn channel_get(&self, device_address: Path<String>) -> GatewayResponse<String> {
        let pixoo = PixooClient::new(&device_address.0);
        let result = pixoo.get_current_channel().await.unwrap();
        GatewayResponse::Ok(Json(GatewayResponseDTO::ok(result.to_string())))
    }
}
