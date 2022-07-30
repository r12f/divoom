use super::api_handler::*;
use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;

pub struct ApiServer {
    server_address: String,
    server_port: u16,
    device_address: String,
}

impl ApiServer {
    pub fn new(server_address: String, server_port: u16, device_address: String) -> ApiServer {
        ApiServer {
            server_address,
            server_port,
            device_address,
        }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let api_service = OpenApiService::new(
            ApiHandler::new(self.device_address.clone()),
            "Divoom Gateway",
            "1.0",
        )
        .server(format!(
            "http://{}:{}/api",
            self.server_address, self.server_port
        ));

        let ui = api_service.swagger_ui();
        let spec = api_service.spec_endpoint();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/openapi.json", spec)
            .nest("/", ui);

        let server_endpoint = format!("{}:{}", self.server_address, self.server_port);
        let server_listener = TcpListener::bind(server_endpoint);
        poem::Server::new(server_listener).run(app).await
    }
}
