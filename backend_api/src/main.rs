use crate::animal::AnimalApi;
use crate::common::config::Config;
use crate::common::object::Message;
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::{EndpointExt, Route, Server};
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, OpenApiService, Tags};
use thiserror::Error;

pub mod animal;
pub mod common;

#[derive(Tags)]
pub enum ApiTag {
    /// Home
    Home,
    /// All about animals
    Animal,
}

struct HomeApi;

#[OpenApi(tag = "ApiTag::Home")]
impl HomeApi {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> Json<Message> {
        Json(Message {
            message: "Hello world".to_string(),
        })
    }
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Config error")]
    ConfigError,
    #[error("IO error")]
    IoError,
}

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context_lazy(|| MainError::ConfigError)?;

    let api_service = OpenApiService::new((HomeApi, AnimalApi), "Animal API", "1.0.0");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    let cors = Cors::new();
    let app = app.with(cors);

    match config.upgrade() {
        Some(config) => {
            println!("Listening on http://{}", config.poem.parse_address());
            Server::new(TcpListener::bind(config.poem.parse_address().as_str()))
                .run(app)
                .await
                .change_context_lazy(|| MainError::IoError)
        }
        None => Err(Report::new(MainError::ConfigError)),
    }
}
