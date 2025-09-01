use crate::validation::string_rules::{StringLengthRule, StringMandatoryRule};
use crate::validation::types::ValidationCheck;
use crate::validation::{StrValidationExtension, StringValidator};
use std::sync::Arc;
use thiserror::Error;

pub struct DescriptionRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for DescriptionRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: None,
            max_length: Some(40),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule)> for &DescriptionRules {
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

impl DescriptionRules {
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
    pub fn parse_custom(
        subject: String,
        rules: &DescriptionRules,
    ) -> Result<Self, DescriptionError> {
        let mut msgs: Vec<String> = vec![];
        let validator = subject.as_string_validator();

        rules.check(&mut msgs, &validator);
        ValidationCheck::validation_check(msgs)?;
        Ok(Self(subject))
    }

    pub fn parse(subject: String) -> Result<Self, DescriptionError> {
        Self::parse_custom(subject, &DescriptionRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
