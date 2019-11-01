// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions related to static files used by both `bluefire_build` and
//! `bluefire_static_files_macros` crates.
//!
//! This crate is an implementation detail and should not be used directly. Use the aforementioned
//! crates instead.

#![warn(missing_docs)]

use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

/// Type and additional options for the source.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Type {
    /// Javascript file.
    #[serde(rename = "js")]
    Js {
        /// Name of the generated field.
        field_name: String,
    },

    /// SCCS file.
    #[serde(rename = "scss")]
    Scss {
        /// Name of the generated field.
        field_name: String,
    },

    /// WASM file.
    #[serde(rename = "wasm")]
    Wasm {
        /// Path to the target directory where the WASM file can be found.
        target_path: String,

        /// Name of the generated field for the WASM file.
        field_name_wasm: String,

        /// Name of the generated field for the binding Javascript file.
        field_name_js: String,
    },
}

/// Source of the data.
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    /// Basename of the input file. The extension is deduced from the type and path the file is
    /// provided by `Spec`.
    pub input_base_name: String,

    /// Basename of the file as it will be accessible from the server. The extension is deduced
    /// from the type and path the file is
    pub output_base_name: String,

    /// Type of the file plus additional options.
    #[serde(flatten)]
    pub variant: Type,
}

/// Specification of the static file description file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    /// Path to all the source files.
    pub source_dir: String,

    /// Source files to be taken into consideration.
    pub sources: Vec<Source>,
}

impl Spec {
    /// Read the spec from the given file.
    pub fn read(path: PathBuf) -> Self {
        let input = {
            match std::fs::read_to_string(&path) {
                Ok(input) => input,
                Err(err) => panic!("Failed to read file {:?}: {}", path, err),
            }
        };

        let spec = {
            match serde_yaml::from_str::<Self>(&input) {
                Ok(spec) => spec,
                Err(err) => panic!("Failed to parse file {:?}: {}", path, err),
            }
        };

        spec
    }
}

/// Prelude for this crate.
pub mod prelude {
    pub use super::{Source, Spec, Type};
}
