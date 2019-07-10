// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Functionality related to translations.

use crate::{BlueFire, Extension, Request};
pub use bluefire_translations::TranslationProvider;

// -------------------------------------------------------------------------------------------------

/// Given a request, returns a list of IDs of all languages supported by the client.
pub fn get_accepted_languages(request: &Request) -> Vec<String> {
    let mut result = Vec::new();
    let values = request.headers().get_all(http::header::ACCEPT_LANGUAGE);
    for value in values.iter() {
        if let Ok(langs_str) = value.to_str() {
            for lang_str in langs_str.split(",") {
                result.push(lang_str.trim().to_string());
            }
        }
    }
    result
}

// -------------------------------------------------------------------------------------------------

/// Extension providing translation configuration: default and supported languages.
#[derive(Clone, Debug)]
pub struct TranslationExtension {
    default_language: String,
    supported_languages: Vec<String>,
}

impl TranslationExtension {
    /// Constructs a new `TranslationExtension`.
    ///
    /// The passed list of supported languages must be non-empty, otherwise the constructor panics.
    /// The first language in the given list is the default one.
    pub fn new(supported_languages: Vec<String>) -> Self {
        if let Some(default_language) = supported_languages.first() {
            Self {
                default_language: default_language.clone(),
                supported_languages: supported_languages,
            }
        } else {
            panic!("At least one language must be supported");
        }
    }

    /// Returns an ID of the default language.
    pub fn get_default_language(&self) -> &String {
        &self.default_language
    }

    /// Returns a list of IDs of the all supported language.
    pub fn get_supported_languages(&self) -> &Vec<String> {
        &self.supported_languages
    }
}

impl Extension for TranslationExtension {
    fn get_name(&self) -> &str {
        "BlueFire:Translations"
    }

    fn check(&self) -> Result<(), ()> {
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn Extension> {
        Box::new(self.clone())
    }

    fn destroy(&self) {
        // noting to do
    }
}

// -------------------------------------------------------------------------------------------------

/// Given the request and its context determine the most appropriate language and load the
/// translations.
pub fn provide_translation<T>(context: &BlueFire, request: &Request) -> T
where
    T: TranslationProvider,
{
    fn langs_match(supported_lang: &String, accepted_lang: &String) -> bool {
        accepted_lang.starts_with(supported_lang)
    }

    let translations = context
        .extension::<TranslationExtension>()
        .expect("Expected translation extension not provided");
    let accepted_langs = get_accepted_languages(request);
    let supported_langs = translations.get_supported_languages();

    for accepted_lang in accepted_langs.iter() {
        for supported_lang in supported_langs.iter() {
            if langs_match(supported_lang, accepted_lang) {
                if let Some(translation) = T::provide(supported_lang) {
                    return translation;
                } else {
                    log_error!("Failed to provide translation for '{}' language", supported_lang);
                }
            }
        }
    }

    let default_lang = translations.get_default_language();
    if let Some(translation) = T::provide(default_lang) {
        return translation;
    } else {
        log_error!("Failed to provide translation for defaut language");
        T::provide_default()
    }
}
