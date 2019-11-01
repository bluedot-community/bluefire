// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_backend::translations`.

use bluefire_backend::{translations::*, *};

mod env {
    use super::*;

    pub struct Env {
        pub wielder: BlueFireWielder,
    }

    impl Env {
        pub fn new(supported_languages: Vec<String>) -> Env {
            let routing_builder = Box::new(router::RoutingBuilder::new());
            let translation_extension = TranslationExtension::new(supported_languages);
            let kindler = BlueFireKindler::start(routing_builder).extend(translation_extension);

            Env { wielder: kindler.kindle() }
        }
    }

    pub fn build_request(languages: &str) -> Request {
        let mut request = Request::new(Vec::new());
        let headers = request.headers_mut();
        let header_value = http::header::HeaderValue::from_str(languages).unwrap();
        headers.insert(http::header::ACCEPT_LANGUAGE, header_value);
        request
    }

    pub struct Texts {
        pub text1: &'static str,
        pub text2: &'static str,
    }

    impl TranslationProvider for Texts {
        fn provide(lang_code: &str) -> Option<Self> {
            match lang_code {
                "en" => Some(Self { text1: "text1_en", text2: "text2_en" }),
                "es" => Some(Self { text1: "text1_es", text2: "text2_es" }),
                _ => None,
            }
        }

        fn provide_default() -> Self {
            Self::provide("en").unwrap()
        }
    }
}

/// Checks if `get_accepted_languages` correctly extracts language codes from a request.
#[test]
fn test_get_accepted_languages() {
    let request = env::build_request("es,en,fr_FR");
    assert_eq!(translations::get_accepted_languages(&request), vec!["es", "en", "fr_FR"]);
}

/// Checks if when many supported languages are accepted the most preferred is used.
#[test]
fn test_provide_prefered_translation() {
    let env = env::Env::new(vec!["en".to_string(), "es".to_string()]);
    let request = env::build_request("fr,es,nl,en,ru");
    let texts = provide_translation::<env::Texts>(env.wielder.get_context(), &request);
    assert_eq!(texts.text1, "text1_es");
}

/// Checks if when no supported languages as accepted, the default, as defined by extension, is
/// used.
#[test]
fn test_provide_extension_default_translation() {
    let env = env::Env::new(vec!["ru".to_string(), "nl".to_string()]);
    let request = env::build_request("en,fr,es");
    let texts = provide_translation::<env::Texts>(env.wielder.get_context(), &request);
    assert_eq!(texts.text1, "text1_en");
}

/// Checks if when no supported languages as accepted and the extension-default is not available,
/// the default, as defined by the translation provider is used.
#[test]
fn test_provide_builder_default_translations() {
    let env = env::Env::new(vec!["ru".to_string(), "nl".to_string()]);
    let request = env::build_request("fr");
    let texts = provide_translation::<env::Texts>(env.wielder.get_context(), &request);
    assert_eq!(texts.text1, "text1_en");
}
