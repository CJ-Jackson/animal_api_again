use dioxus::prelude::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use shared::validation::models::animal::{AnimalValidated, AnimalValidationError};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct AnimalModel {
    pub id: i64,
    pub species: String,
    pub description: String,
}

impl AnimalModel {
    pub fn validate(&self) -> Result<AnimalValidated, AnimalValidationError> {
        AnimalValidated::parse(self.species.clone(), self.description.clone())
    }
}

impl From<(&AnimalValidationError, &AnimalModel)> for AnimalModel {
    fn from((error, model): (&AnimalValidationError, &AnimalModel)) -> Self {
        Self {
            id: 0,
            species: error
                .species
                .clone()
                .map(|s| s.as_str().to_string())
                .unwrap_or(model.species.clone()),
            description: error
                .description
                .clone()
                .map(|s| s.as_str().to_string())
                .unwrap_or(model.description.clone()),
        }
    }
}

pub trait AnimalModelSignal {
    fn species(&mut self, species: String);
    fn description(&mut self, description: String);
}

impl AnimalModelSignal for Signal<AnimalModel> {
    fn species(&mut self, species: String) {
        let mut signal = self.write();
        signal.species = species;
    }

    fn description(&mut self, description: String) {
        let mut signal = self.write();
        signal.description = description;
    }
}

#[derive(Debug, Serialize)]
pub struct AnimalAddUpdateModel {
    pub species: String,
    pub description: String,
}

impl From<AnimalValidated> for AnimalAddUpdateModel {
    fn from(value: AnimalValidated) -> Self {
        Self {
            species: value.species.as_str().to_string(),
            description: value.description.as_str().to_string(),
        }
    }
}
