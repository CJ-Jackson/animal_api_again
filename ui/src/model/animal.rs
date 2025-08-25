use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct AnimalModel {
    pub id: i64,
    pub species: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct AnimalAddUpdateModel {
    pub species: String,
    pub description: String,
}
