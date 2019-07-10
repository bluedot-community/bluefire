// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate contains tools useful for providing and handling translations.

#![warn(missing_docs)]

/// Trait for `struct`s providing translations.
///
/// Do not implement this trait manually. `bluefire_translations_derive` provides macros to generate
/// implementations automatically from translation files.
///
/// The structures implementing this trait are expected to provide public members of type `&'static
/// str` or `String` containing translations. Construction of the implementations is done calling
/// one of traits methods.
pub trait TranslationProvider: Sized {
    /// Constructs a new translations provider for a given language code.
    fn provide(lang_code: &str) -> Option<Self>;

    /// Constructs a new translations provider for a default language.
    fn provide_default() -> Self;
}
