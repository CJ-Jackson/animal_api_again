use poem_openapi::Object;
use shared::validation::models::animal::{AnimalValidated, AnimalValidationError};

#[derive(Debug, Object, Clone)]
pub struct AnimalObject {
    pub id: i64,
    pub species: String,
    pub description: String,
}

#[derive(Debug, Object)]
pub struct AnimalAddUpdateObject {
    pub species: String,
    pub description: String,
}

impl AnimalAddUpdateObject {
    pub fn to_validate(&self) -> Result<AnimalValidated, AnimalValidationError> {
        AnimalValidated::parse(self.species.clone(), self.description.clone())
    }
}
