// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Back-end part of `BlueFire` web framework.

#![warn(missing_docs)]

#[macro_use]
mod common;
pub use self::common::{BlueFireError, Body, GlobalState, Handler, ParamsMap, Request, Response};

pub mod clock;
pub mod router;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "translations")]
pub mod translations;

mod context;
pub use self::context::{BlueFire, BlueFireKindler, BlueFireWielder};
pub use self::context::{Extension, Extensions, Middleware};

#[cfg(feature = "rest")]
pub mod rest;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "authentication")]
pub mod authentication;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "scheduler")]
pub mod scheduler;
