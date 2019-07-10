// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions of constants.

/// The key name for session cookies.
pub const SESSION_COOKIE_KEY: &str = "SESSION_ID";

/// The prefix for session cookies.
pub const SESSION_COOKIE_PREFIX: &str = "SESSION_ID=";

/// The name for HTTP header used for transmitting the session token.
pub const BLUEFIRE_TOKEN_HEADER: &str = "X-BlueFire-Token";
