use crate::validation::StrValidationExtension;
use crate::validation::types::ValidationCheck;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Species Validation Error")]
pub struct SpeciesError(pub Arc<[String]>);

impl ValidationCheck for SpeciesError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(SpeciesError(strings.into()))
        }
    }
}

impl Clone for SpeciesError {
    fn clone(&self) -> Self {
        SpeciesError(Arc::clone(&self.0))
    }
}

#[derive(Default, Clone, Debug)]
pub struct Species(String);

impl Species {
    pub fn parse(subject: String) -> Result<Self, SpeciesError> {
        let mut msgs: Vec<String> = vec![];
        let validator = subject.as_string_validator();

        let mut check_count = true;

        validator.is_empty().then(|| {
            check_count = false;
            msgs.push("Cannot be empty".to_string())
        });
        check_count.then(|| {
            let count = validator.count_graphemes();
            (count > 20).then(|| msgs.push("Must be at most 20 characters".to_string()));
        });

        ValidationCheck::validation_check(msgs)?;
        Ok(Species(subject))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
