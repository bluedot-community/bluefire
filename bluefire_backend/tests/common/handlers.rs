// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of a handler for tests.

use http;

use bluefire_backend::{BlueFire, Handler, Request, Response};

#[derive(Clone, Debug)]
pub struct TestHandler {
    id: String,
}

impl TestHandler {
    pub fn new(id: &str) -> Box<dyn Handler> {
        Box::new(TestHandler { id: id.to_string() })
    }
}

impl Handler for TestHandler {
    fn handle(&self, _context: &BlueFire, _request: Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::OK)
            .body(self.id.clone().into())
            .expect("Failed to build empty response body")
    }

    fn duplicate(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}
