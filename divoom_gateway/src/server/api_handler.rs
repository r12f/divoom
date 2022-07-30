use super::api_server_dto::*;
use divoom::*;
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, Tags};

pub struct ApiHandler {
    device_address: String,
}

#[derive(Tags)]
enum ApiTags {
    Channel,
}

#[OpenApi]
impl ApiHandler {
    pub fn new(device_address: String) -> ApiHandler {
        ApiHandler { device_address }
    }

    #[oai(path = "/channel", method = "get", tag = "ApiTags::Channel")]
    async fn channel_get(&self) -> GatewayResponse<String> {
        let pixoo = PixooClient::new(&self.device_address);
        let result = pixoo.get_current_channel().await.unwrap();
        GatewayResponse::Ok(Json(GatewayResponseDTO::ok(result.to_string())))
    }
}
