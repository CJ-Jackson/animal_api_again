use crate::validation::models::error_flag;
use crate::validation::types::description::{Description, DescriptionError};
use crate::validation::types::species::{Species, SpeciesError};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("Animal validation error")]
pub struct AnimalValidationError {
    pub species: Result<Species, SpeciesError>,
    pub description: Result<Description, DescriptionError>,
}

impl Default for AnimalValidationError {
    fn default() -> Self {
        Self {
            species: Ok(Species::default()),
            description: Ok(Description::default()),
        }
    }
}

pub struct AnimalValidated {
    pub species: Species,
    pub description: Description,
}

impl AnimalValidated {
    pub fn parse(
        species_ori: String,
        description_org: String,
    ) -> Result<Self, AnimalValidationError> {
        let mut flag = false;

        use error_flag as ef;
        let species = ef(&mut flag, Species::parse(species_ori.clone()));
        let description = ef(&mut flag, Description::parse(description_org.clone()));

        if flag {
            return Err(AnimalValidationError {
                species,
                description,
            });
        }

        Ok(Self {
            species: species.unwrap_or_default(),
            description: description.unwrap_or_default(),
        })
    }
}
