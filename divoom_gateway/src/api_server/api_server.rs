use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use super::api_handler::*;

pub struct ApiServer {
    server_address: String,
    server_port: u16,
}

impl ApiServer {
    pub fn new(server_address: String, server_port: u16) -> ApiServer {
        ApiServer { server_address, server_port }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let api_service =
            OpenApiService::new(ApiHandler::new(), "Divoom Gateway", "1.0").server(format!("http://{}:{}/api", self.server_address, self.server_port));
        let ui = api_service.swagger_ui();
        let spec = api_service.spec_endpoint();
        let app = Route::new().nest("/api", api_service).nest("/", ui).nest("/openapi.json", spec);

        poem::Server::new(TcpListener::bind(format!("{}:{}", self.server_address, self.server_port)))
            .run(app)
            .await
    }
}