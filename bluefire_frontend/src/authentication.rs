// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Authentication-related utilities.

use bluefire_twine::constants::*;

/// Return a session cookie if defined.
pub fn get_session_cookie() -> Option<String> {
    crate::cookies::get_cookie(SESSION_COOKIE_KEY)
}

/// Builds the body of a session cookie.
pub fn build_session_cookie(session_id: String) -> crate::cookies::Cookie {
    crate::cookies::Cookie::new(SESSION_COOKIE_KEY.to_string(), session_id)
}

/// Creates a session cookie.
pub fn set_session_cookie(session_id: String) {
    build_session_cookie(session_id)
        .with_lifetime(crate::cookies::Lifetime::MaxAgeSeconds(7 * 24 * 60 * 60))
        .set();
}

/// Remove the session cookie.
pub fn remove_session_cookie() {
    crate::cookies::Cookie::new(SESSION_COOKIE_KEY.to_string(), String::default())
        .with_lifetime(crate::cookies::Lifetime::MaxAgeSeconds(0))
        .set();
}
