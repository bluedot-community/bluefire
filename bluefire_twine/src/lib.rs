// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides definitions and functionality shared by `bluefire_backend` and
//! `bluefire_frontend`.

#![warn(missing_docs)]

pub mod constants;
pub mod id;
pub mod message;
pub mod validation;

pub use crate::id::Id;
pub use crate::message::Message;
pub use crate::validation::ValidationResult;
