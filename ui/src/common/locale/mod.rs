use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleValue, ValidateErrorStore};
use dioxus_i18n::fluent::FluentArgs;
use dioxus_i18n::prelude::{I18n, I18nConfig};
use dioxus_i18n::unic_langid::langid;
use std::sync::Arc;

pub fn build_locale_config() -> I18nConfig {
    let english = include_str!("_locale/english.ftl");
    let french = include_str!("_locale/french.ftl");

    I18nConfig::new(langid!("en-GB"))
        .with_locale((langid!("en-GB"), english))
        // .with_locale((langid!("en-US"), english))
        .with_locale((langid!("fr-FR"), french))
    // .with_fallback(langid!("en-GB"))
}

pub trait LocaleForData {
    fn get_translation(&self, locale: &I18n, original: String) -> String;
}

impl LocaleForData for LocaleData {
    fn get_translation(&self, locale: &I18n, original: String) -> String {
        if !self.args.is_empty() {
            let mut values = FluentArgs::default();
            for (key, value) in self.args.iter() {
                match value {
                    LocaleValue::String(string) => {
                        values.set::<String, String>(key.clone(), string.clone());
                    }
                    LocaleValue::Uint(unit) => {
                        values.set::<String, usize>(key.clone(), *unit);
                    }
                    LocaleValue::Int(int) => {
                        values.set::<String, isize>(key.clone(), *int);
                    }
                    LocaleValue::Float(float) => {
                        values.set::<String, f64>(key.clone(), *float);
                    }
                }
            }
            locale
                .try_translate_with_args(self.name.clone().as_str(), Some(&values))
                .unwrap_or(original)
        } else {
            locale
                .try_translate(self.name.clone().as_str())
                .unwrap_or(original)
        }
    }
}

pub trait LocaleForStore {
    fn as_translated_message(&self, locale: &I18n) -> Arc<[String]>;
}

impl LocaleForStore for ValidateErrorStore {
    fn as_translated_message(&self, locale: &I18n) -> Arc<[String]> {
        self.0
            .iter()
            .map(|e| e.1.get_locale_data().get_translation(locale, e.0.clone()))
            .collect()
    }
}
