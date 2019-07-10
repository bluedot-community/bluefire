// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Code generation of HTTP API from protocol definition files.
//!
//! The goal of `bluefire_protogen` is to define the structure of an HTTP API in one place (YAML
//! file) and generate `Rust` code from it to mitigate later need to modify the code in many places
//! (potentially introducing bugs).
//!
//! This crate provides a generator that can be used in `build.rs` scripts. See also
//! `bluefire_protogen_macros` for related macros.

// TODO: Macros do not use binaries, so split the binaries from this crate.

#![warn(missing_docs)]

pub mod buffer;
pub mod rust_generator;
pub mod spec;
pub mod utils;
