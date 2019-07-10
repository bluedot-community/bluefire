// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Communication with the server.

use bluefire_twine::constants::*;

/// Fetched a remote resource.
pub fn fetch(host: &str, message: &bluefire_twine::message::Message) -> js_sys::Promise {
    let mut request_init = web_sys::RequestInit::new();
    request_init.method(message.method());
    if (message.method() != "GET") && (message.method() != "HEAD") {
        request_init.body(Some(&wasm_bindgen::JsValue::from_str(message.body())));
    }

    if let Some(session_id) = crate::authentication::get_session_cookie() {
        let headers = web_sys::Headers::new().expect("Initialize headers");
        headers.append(BLUEFIRE_TOKEN_HEADER, &session_id).expect("Append header");
        request_init.headers(&headers);
    }

    let path = if message.query().is_empty() {
        String::from(host) + message.path()
    } else {
        String::from(host) + message.path() + "?" + message.query()
    };

    let request =
        web_sys::Request::new_with_str_and_init(&path, &request_init).expect("Initialize request");

    web_sys::window().unwrap().fetch_with_request(&request)
}

/// Prelude for `fetch` module.
pub mod prelude {
    pub use super::fetch;
}
