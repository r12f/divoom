use super::api_handler::*;
use divoom::DivoomAnimationTemplateManager;
use poem::{listener::TcpListener, Route, handler};
use poem_openapi::OpenApiService;
use std::sync::Arc;

pub struct ApiServer {
    server_address: String,
    server_port: u16,
    device_address: String,
    animation_template_manager: Arc<DivoomAnimationTemplateManager>,
}

impl ApiServer {
    pub fn new(
        server_address: String,
        server_port: u16,
        device_address: String,
        animation_template_manager: Arc<DivoomAnimationTemplateManager>,
    ) -> ApiServer {
        ApiServer {
            server_address,
            server_port,
            device_address,
            animation_template_manager,
        }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let api_service = OpenApiService::new(
            ApiHandler::new(
                self.device_address.clone(),
                self.animation_template_manager.clone(),
            ),
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
            .nest("/", ui)
            .at("/probe", poem::get(probe));

        let server_endpoint = format!("{}:{}", self.server_address, self.server_port);
        let server_listener = TcpListener::bind(server_endpoint);
        poem::Server::new(server_listener).run(app).await
    }
}

#[handler]
fn probe() -> String {
    "ok".to_string()
}
