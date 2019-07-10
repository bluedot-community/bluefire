// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! The REST-related functionality.

use bluefire_twine::constants::*;

use crate::common::{Handler, Request, Response};
use crate::context::BlueFire;

// -------------------------------------------------------------------------------------------------

/// Trait for simple REST handlers.
pub trait SimpleRestHandler: Handler {
    /// Builds a response for not allowed method. The default implementation builds a response with
    /// empty body.
    fn make_method_not_allowed_response(&self, _request: &Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::METHOD_NOT_ALLOWED)
            .body(String::new())
            .expect("Failed to build not allowed method response body content.")
    }

    /// Options method request handler. The default implementation builds a response allowing the
    /// access from any origin and using any method.
    fn options(&self, _context: &BlueFire, _request: &Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::OK)
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE")
            .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, BLUEFIRE_TOKEN_HEADER)
            .body(String::new())
            .expect("Build OPTIONS response body")
    }

    /// Get method request handler. The default implementation returns "method not allowed".
    fn get(&self, _context: &BlueFire, request: &Request) -> Response {
        self.make_method_not_allowed_response(request)
    }

    /// Post method request handler. The default implementation returns "method not allowed".
    fn post(&self, _context: &BlueFire, request: &Request) -> Response {
        self.make_method_not_allowed_response(request)
    }

    /// Put method request handler. The default implementation returns "method not allowed".
    fn put(&self, _context: &BlueFire, request: &Request) -> Response {
        self.make_method_not_allowed_response(request)
    }

    /// Patch method request handler. The default implementation returns "method not allowed".
    fn patch(&self, _context: &BlueFire, request: &Request) -> Response {
        self.make_method_not_allowed_response(request)
    }

    /// Delete method request handler. The default implementation returns "method not allowed".
    fn delete(&self, _context: &BlueFire, request: &Request) -> Response {
        self.make_method_not_allowed_response(request)
    }
}

impl<T> Handler for T
where
    T: SimpleRestHandler + Clone + 'static,
{
    fn handle(&self, context: &BlueFire, request: &Request) -> Response {
        match request.method() {
            &http::method::Method::OPTIONS => self.options(context, request),
            &http::method::Method::GET => self.get(context, request),
            &http::method::Method::POST => self.post(context, request),
            &http::method::Method::PUT => self.put(context, request),
            &http::method::Method::PATCH => self.patch(context, request),
            &http::method::Method::DELETE => self.delete(context, request),
            _ => self.make_method_not_allowed_response(request),
        }
    }

    fn duplicate(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}
