use dioxus::prelude::{Signal, WritableExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct AnimalModel {
    pub id: i64,
    pub species: String,
    pub description: String,
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

impl Into<AnimalAddUpdateModel> for AnimalModel {
    fn into(self) -> AnimalAddUpdateModel {
        AnimalAddUpdateModel {
            species: self.species,
            description: self.description,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AnimalAddUpdateModel {
    pub species: String,
    pub description: String,
}
