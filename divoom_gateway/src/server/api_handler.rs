use divoom::*;
use poem_openapi::{param::Query, payload::PlainText, OpenApi, Tags};
use poem_openapi::param::Path;

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

    #[oai(path = "/devices/:device_address/channel", method = "get", tag = "ApiTags::Channel")]
    async fn channel_get(&self, device_address: Path<String>) -> PlainText<String> {
        let pixoo = PixooClient::new(&device_address.0);
        let result = pixoo.get_current_channel().await.unwrap();
        let response = serde_json::to_string(&result).unwrap();
        PlainText(response)
    }
}
