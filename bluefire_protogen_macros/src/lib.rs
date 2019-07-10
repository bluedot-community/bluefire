// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Macros for generating code from `bluefire_protogen` protocol definitions.
//!
//! The goal of `bluefire_protogen` is to define the structure of an HTTP API in one place (YAML
//! file) and generate `Rust` code from it to mitigate later need to modify the code in many places
//! (potentially introducing bugs).

// TODO: Uncomment after https://github.com/rust-lang/rust/issues/42008 is fixed
// #![warn(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use std::path::PathBuf;

// -------------------------------------------------------------------------------------------------

struct Config {
    file_path: String,
}

// -------------------------------------------------------------------------------------------------

fn as_cargo_absolute_path(relative_path: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory not provided"));
    path.push(relative_path);
    path
}

// -------------------------------------------------------------------------------------------------

fn parse_params(stream: proc_macro::TokenStream) -> Config {
    let lit: syn::Lit = syn::parse(stream).expect("Failed to parse input into a literal");
    match lit {
        syn::Lit::Str(lit_str) => Config { file_path: lit_str.value() },
        _ => panic!("The argument should be a string"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Generates `bluefire_backend::Route` from API definition.
#[proc_macro]
pub fn routes(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let config = parse_params(stream);
    let path = as_cargo_absolute_path(&config.file_path);
    let input = std::fs::read_to_string(&path).expect(&format!("Read '{}' file", config.file_path));
    let routes = bluefire_protogen::spec::Routes::from_str(&input)
        .expect(&format!("Parse '{}' file", config.file_path));
    let generator = bluefire_protogen::rust_generator::RustGenerator::new();
    let result = generator.generate_routes(&routes);
    result.parse().expect("Parse into TokenStream")
}

// -------------------------------------------------------------------------------------------------

/// Generates structures representing HTTP paths from API definition.
///
/// The generated structures provide constructors for parametrized path.
#[proc_macro]
pub fn paths(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let config = parse_params(stream);
    let path = as_cargo_absolute_path(&config.file_path);
    let input = std::fs::read_to_string(&path).expect(&format!("Read '{}' file", config.file_path));
    let routes = bluefire_protogen::spec::Routes::from_str(&input)
        .expect(&format!("Parse '{}' file", config.file_path));
    let generator = bluefire_protogen::rust_generator::RustGenerator::new();
    let result = generator.generate_paths(&routes);
    result.parse().expect("Parse into TokenStream")
}
