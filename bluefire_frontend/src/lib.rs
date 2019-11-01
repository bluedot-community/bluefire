// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Front-end part of `BlueFire` web framework.

#![warn(missing_docs)]
#![feature(proc_macro_hygiene)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

#[cfg(feature = "web")]
pub use web_sys;

#[cfg(feature = "web")]
#[macro_use]
pub mod web;

#[cfg(feature = "fetch")]
pub mod fetch;

#[cfg(feature = "flow")]
pub mod flow;

#[cfg(feature = "flowex")]
pub mod flowex;

#[cfg(feature = "elements")]
pub mod elements;

#[cfg(feature = "cookies")]
pub mod cookies;

#[cfg(feature = "authentication")]
pub mod authentication;
