pub mod description;
pub mod species;

pub trait ValidationCheck: Sized {
    fn validation_check(strings: Vec<String>) -> Result<(), Self>;
}
