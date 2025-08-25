use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Animal {
    pub id: i64,
    pub species: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct AnimalAddUpdate {
    pub species: String,
    pub description: String,
}
