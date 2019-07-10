// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Functionality extending the `flow` module.

use wasm_bindgen::{prelude::*, JsCast};

use crate::flow::prelude::*;

// -------------------------------------------------------------------------------------------------

/// Interprets the passed JavaScript value as as response from a `fetch` request and casts it to
/// JSON. In case of failure the `flow` is interrupted.
pub fn extract_json_from_response(value: JsValue) -> FlowResult {
    if value.is_instance_of::<web_sys::Response>() {
        let resp: web_sys::Response = value.dyn_into().unwrap();
        match resp.json() {
            Ok(promise) => Ok(Some(promise)),
            Err(err) => {
                web_error!("bluefire: response does not contain JSON: {:?}", err);
                Err(())
            }
        }
    } else {
        web_error!("BlueFire: passed JSON value is not a response: {:?}", value);
        Err(())
    }
}
