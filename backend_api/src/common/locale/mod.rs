use poem::error::I18NError;
use poem::i18n::{I18NArgs, I18NResources, Locale};
use shared::validation::validate_locale::{LocaleData, LocaleValue, ValidateErrorStore};
use std::sync::Arc;

pub fn build_resources() -> Result<I18NResources, I18NError> {
    let english = include_str!("_locale/english.ftl");
    let french = include_str!("_locale/french.ftl");

    I18NResources::builder()
        .add_ftl("en-GB", english)
        .add_ftl("en-US", english)
        .add_ftl("fr-FR", french)
        .build()
}

pub trait LocaleForData {
    fn get_translation(&self, locale: &Locale, original: String) -> String;
}

impl LocaleForData for LocaleData {
    fn get_translation(&self, locale: &Locale, original: String) -> String {
        if self.args.is_empty() {
            let mut values = I18NArgs::default();
            for (key, value) in self.args.iter() {
                match value {
                    LocaleValue::String(string) => {
                        values = values.set::<String, String>(key.clone(), string.clone());
                    }
                    LocaleValue::Uint(unit) => {
                        values = values.set::<String, usize>(key.clone(), *unit);
                    }
                    LocaleValue::Int(int) => {
                        values = values.set::<String, isize>(key.clone(), *int);
                    }
                    LocaleValue::Float(float) => {
                        values = values.set::<String, f64>(key.clone(), *float);
                    }
                }
            }
            locale
                .text_with_args(self.name.clone(), values)
                .unwrap_or(original)
        } else {
            locale.text(self.name.clone()).unwrap_or(original)
        }
    }
}

pub trait LocaleForStore {
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]>;
}

impl LocaleForStore for ValidateErrorStore {
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]> {
        self.0
            .iter()
            .map(|e| e.1.get_locale_data().get_translation(locale, e.0.clone()))
            .collect()
    }
}
