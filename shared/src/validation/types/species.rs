use cjtoolkit_structured_validator::base::string_rules::{StringLengthRules, StringMandatoryRules};
use cjtoolkit_structured_validator::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use cjtoolkit_structured_validator::common::string_validator::{
    StrValidationExtension, StringValidator,
};
use cjtoolkit_structured_validator::common::validation_check::ValidationCheck;
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

impl Into<(StringMandatoryRules, StringLengthRules)> for &SpeciesRules {
    fn into(self) -> (StringMandatoryRules, StringLengthRules) {
        (
            StringMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRules {
                min_length: self.min_length,
                max_length: self.max_length,
            },
        )
    }
}

impl SpeciesRules {
    fn rules(&self) -> (StringMandatoryRules, StringLengthRules) {
        self.into()
    }

    fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        let (mandatory_rule, length_rule) = self.rules();
        mandatory_rule.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length_rule.check(msgs, subject);
    }
}

#[derive(Debug, Error, PartialEq, Default, Clone)]
#[error("Species Validation Error")]
pub struct SpeciesError(pub ValidateErrorStore);

impl ValidationCheck for SpeciesError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Species(String);

impl Species {
    pub fn parse_custom(subject: String, rules: SpeciesRules) -> Result<Self, SpeciesError> {
        let mut msgs = ValidateErrorCollector::new();
        let validator = subject.as_string_validator();
        rules.check(&mut msgs, &validator);
        ValidationCheck::validate_check(msgs)?;
        Ok(Species(subject))
    }

    pub fn parse(subject: String) -> Result<Self, SpeciesError> {
        Self::parse_custom(subject, SpeciesRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
