// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides information about languages like their ISO codes and endonyms.

#![warn(missing_docs)]

/// Container for information about a language.
#[derive(Clone, Debug)]
pub struct Language {
    iso_639_1: &'static str,
    iso_639_3: &'static str,
    endonym: &'static str,
}

impl Language {
    /// Returns ISO 639-1 code for the laguage.
    pub fn get_iso_639_1_code(&self) -> &'static str {
        self.iso_639_1
    }

    /// Returns ISO 639-3 code for the laguage.
    pub fn get_iso_639_3_code(&self) -> &'static str {
        self.iso_639_3
    }

    /// Returns the laguages endonym (the name of the languge in this language).
    pub fn get_endonym(&self) -> &'static str {
        self.endonym
    }
}

include!(concat!(env!("OUT_DIR"), "/languages.rs"));
