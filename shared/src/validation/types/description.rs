use crate::validation::StrValidationExtension;
use crate::validation::types::ValidationCheck;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Default)]
#[error("Description Validation Error")]
pub struct DescriptionError(pub Arc<[String]>);

impl ValidationCheck for DescriptionError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(Self(strings.into()))
        }
    }
}

impl Clone for DescriptionError {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Description(String);

impl Description {
    pub fn parse(subject: String) -> Result<Self, DescriptionError> {
        let mut msgs: Vec<String> = vec![];
        let validator = subject.as_string_validator();

        let mut check_count = true;

        validator.is_empty().then(|| {
            check_count = false;
            msgs.push("Cannot be empty".to_string())
        });
        check_count.then(|| {
            let count = validator.count_graphemes();
            (count > 40).then(|| msgs.push("Must be at most 40 characters".to_string()));
        });

        ValidationCheck::validation_check(msgs)?;
        Ok(Self(subject))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
