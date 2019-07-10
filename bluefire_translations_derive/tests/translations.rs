// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_translations_derive`.

use bluefire_translations::TranslationProvider;
use bluefire_translations_derive::Translations;

#[derive(Translations)]
#[translations(path = "tests/translations", default_language = "es")]
struct Messages {
    msg1: &'static str,
    msg2: &'static str,
}

#[test]
fn test_translations() {
    let messages_en = Messages::provide("en").unwrap();
    assert_eq!(messages_en.msg1, "Message 1");
    assert_eq!(messages_en.msg2, "Message 2");

    let messages_es = Messages::provide("es").unwrap();
    assert_eq!(messages_es.msg1, "Mensaje 1");
    assert_eq!(messages_es.msg2, "Mensaje 2");
}

#[test]
fn test_default_language() {
    let messages_es = Messages::provide_default();
    assert_eq!(messages_es.msg1, "Mensaje 1");
    assert_eq!(messages_es.msg2, "Mensaje 2");
}

#[test]
#[should_panic]
fn test_panic_for_not_existing_lang_code() {
    let _ = Messages::provide("fr").unwrap();
}
