use crate::validation::validate_locale::ValidateErrorCollector;

pub mod description;
pub mod species;
pub trait ValidationCheck: Sized {
    fn validation_check(strings: ValidateErrorCollector) -> Result<(), Self>;
}
