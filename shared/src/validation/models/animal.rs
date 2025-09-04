use crate::validation::models::error_flag;
use crate::validation::types::species::{Species, SpeciesError};
use cjtoolkit_structured_validator::types::description::{Description, DescriptionError};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
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

#[derive(Default, Clone)]
pub struct AnimalValidated {
    pub species: Species,
    pub description: Description,
}

impl AnimalValidated {
    pub fn parse(species: String, description: String) -> Result<Self, AnimalValidationError> {
        let mut flag = false;

        use error_flag as ef;
        let species = ef(&mut flag, Species::parse(species.clone()));
        let description = ef(
            &mut flag,
            Description::parse(Some(description.clone().as_str())),
        );

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
