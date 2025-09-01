use crate::validation::string_rules::{StringLengthRule, StringMandatoryRule};
use crate::validation::types::ValidationCheck;
use crate::validation::{StrValidationExtension, StringValidator};
use std::sync::Arc;
use thiserror::Error;

pub struct SpeciesRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for SpeciesRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: None,
            max_length: Some(20),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule)> for &SpeciesRules {
    fn into(self) -> (StringMandatoryRule, StringLengthRule) {
        (
            StringMandatoryRule {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRule {
                min_length: self.min_length,
                max_length: self.max_length,
            },
        )
    }
}

impl SpeciesRules {
    fn rules(&self) -> (StringMandatoryRule, StringLengthRule) {
        self.into()
    }

    fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        let (mandatory_rule, length_rule) = self.rules();
        mandatory_rule.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length_rule.check(msgs, subject);
    }
}

#[derive(Debug, Error, PartialEq, Default)]
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

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Species(String);

impl Species {
    pub fn parse_custom(subject: String, rules: &SpeciesRules) -> Result<Self, SpeciesError> {
        let mut msgs: Vec<String> = vec![];
        let validator = subject.as_string_validator();
        rules.check(&mut msgs, &validator);
        ValidationCheck::validation_check(msgs)?;
        Ok(Species(subject))
    }

    pub fn parse(subject: String) -> Result<Self, SpeciesError> {
        Self::parse_custom(subject, &SpeciesRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
