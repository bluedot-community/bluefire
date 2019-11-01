// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `bluefire` static files macros.

#![warn(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

mod static_files;

/// Basing on `static.yaml` file
/// - adds member fields representing the files and generates a constructor initialising them with
///   path they can be accessed with
/// - generates an associated function returning `bluefire_backend::Route` containing provided files
///
/// # Parameters
///
/// * `namespace` - directory in the server from which the files will be served.
#[proc_macro_attribute]
pub fn generate(
    attributes: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    static_files::generate(attributes, input)
}
