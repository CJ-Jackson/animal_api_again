pub mod animal;

use reqwest::Client;
use std::sync::OnceLock;
use thiserror::Error;

fn get_url() -> String {
    "http://127.0.0.1:8000".to_string()
}

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
    let client = CLIENT.get_or_init(|| Client::new());
    client.clone()
}

#[derive(Debug, Error)]
#[error("Api Client Error")]
pub struct ApiClentError;
