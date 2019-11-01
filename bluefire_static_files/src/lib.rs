// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Static files provided by `bluefire`.
//!
//! This includes:
//!  - `css` files with default themes
//!  - `js` files for interacting popular JS libraries from WASM

#![warn(missing_docs)]

#[bluefire_static_files_macros::generate(namespace = "bluefire")]
struct BlueFireStaticFiles;
