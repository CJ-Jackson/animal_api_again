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

#[derive(Debug, Object)]
pub struct AnimalErrorObject {
    pub species: Vec<String>,
    pub description: Vec<String>,
}

impl From<AnimalValidationError> for AnimalErrorObject {
    fn from(value: AnimalValidationError) -> Self {
        Self {
            species: value
                .species
                .err()
                .map(|v| v.0.iter().map(|s| s.clone()).collect())
                .unwrap_or_default(),
            description: value
                .description
                .err()
                .map(|v| v.0.iter().map(|s| s.clone()).collect())
                .unwrap_or_default(),
        }
    }
}
