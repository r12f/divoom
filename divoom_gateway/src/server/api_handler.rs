use poem_openapi::{param::Query, payload::PlainText, OpenApi};

pub struct ApiHandler {}

#[OpenApi]
impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {}
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
