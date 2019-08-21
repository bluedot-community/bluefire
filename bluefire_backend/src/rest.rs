// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! The REST-related functionality.

use std::collections::HashMap;
use std::convert::TryFrom;

use bluefire_twine::constants;

use crate::common::{Handler, Request, Response};
use crate::context::BlueFire;

// -------------------------------------------------------------------------------------------------

/// A trait binding request type, response type and path type.
///
/// This trait facilitates writing shorter generic code. This trait helps to avoid bugs where a
/// request is sent to wrong path, wrong type of response is sent in reply or the response is
/// deserialized in a wrong way.
///
/// `bluefire_protogen` generates implementations of this trait for whole API.
pub trait Method {
    /// Path type.
    // TODO: Rename to `Path`
    type PathParams;

    /// Request type.
    type Request: TryFrom<Request>;

    /// Response type.
    type Response: Into<Response>;
}

/// Currently in Rust using `?` operator for early exits is ergonomic only in functions returning
/// `Result`. In handle methods we want to always return the response type but early exits simplify
/// the code. Therefore we return a `Result` with the same type in `Ok` and `Err` variants.
pub type Reply<T> = Result<T, T>;

// -------------------------------------------------------------------------------------------------

/// Trait for simple REST handlers.
///
/// The `handler` method here is split into separate methods handling a different HTTP method each.
pub trait SimpleRestHandler: Handler {
    /// Builds a response for not allowed method. The default implementation builds a response with
    /// empty body.
    fn make_method_not_allowed_response(&self, _request: &Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::METHOD_NOT_ALLOWED)
            .body(String::new())
            .expect("Failed to build not allowed method response body content.")
    }

    /// "OPTIONS" method request handler. The default implementation builds a response allowing the
    /// access from any origin and using any method.
    fn options(&self, _context: &BlueFire, _request: &Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::OK)
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE")
            .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, constants::BLUEFIRE_TOKEN_HEADER)
            .body(String::new())
            .expect("Build OPTIONS response body")
    }

    /// "GET" method request handler. The default implementation returns "method not allowed".
    fn get(&self, _context: &BlueFire, request: &Request) -> Reply<Response> {
        Ok(self.make_method_not_allowed_response(request))
    }

    /// "POST" method request handler. The default implementation returns "method not allowed".
    fn post(&self, _context: &BlueFire, request: &Request) -> Reply<Response> {
        Ok(self.make_method_not_allowed_response(request))
    }

    /// "PUT" method request handler. The default implementation returns "method not allowed".
    fn put(&self, _context: &BlueFire, request: &Request) -> Reply<Response> {
        Ok(self.make_method_not_allowed_response(request))
    }

    /// "PATCH" method request handler. The default implementation returns "method not allowed".
    fn patch(&self, _context: &BlueFire, request: &Request) -> Reply<Response> {
        Ok(self.make_method_not_allowed_response(request))
    }

    /// "DELETE" method request handler. The default implementation returns "method not allowed".
    fn delete(&self, _context: &BlueFire, request: &Request) -> Reply<Response> {
        Ok(self.make_method_not_allowed_response(request))
    }
}

impl<T> Handler for T
where
    T: SimpleRestHandler + Clone + 'static,
{
    fn handle(&self, context: &BlueFire, request: Request) -> Response {
        let result = match request.method() {
            &http::method::Method::OPTIONS => return self.options(context, &request),
            &http::method::Method::GET => self.get(context, &request),
            &http::method::Method::POST => self.post(context, &request),
            &http::method::Method::PUT => self.put(context, &request),
            &http::method::Method::PATCH => self.patch(context, &request),
            &http::method::Method::DELETE => self.delete(context, &request),
            _ => return self.make_method_not_allowed_response(&request),
        };
        match result {
            Ok(response) => response,
            Err(response) => response,
        }
    }

    fn duplicate(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}

// -------------------------------------------------------------------------------------------------

/// Default (dummy) request for "GET" method requests.
pub struct DefaultQueryRequest;

impl TryFrom<Request> for DefaultQueryRequest {
    type Error = serde::de::value::Error;

    fn try_from(_request: Request) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

/// Default (dummy) request for requests other than "GET".
pub struct DefaultJsonRequest;

impl TryFrom<Request> for DefaultJsonRequest {
    type Error = serde_json::Error;

    fn try_from(_request: Request) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

/// Default (empty) response.
pub struct DefaultResponse;

impl From<DefaultResponse> for Response {
    fn from(_: DefaultResponse) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::METHOD_NOT_ALLOWED)
            .body(String::new())
            .expect("Failed to build not allowed method response body content.")
    }
}

/// Default (empty) path.
pub struct DefaultPath;

impl std::convert::TryFrom<&HashMap<&'static str, String>> for DefaultPath {
    type Error = &'static str;
    fn try_from(_: &HashMap<&'static str, String>) -> Result<Self, &'static str> {
        Ok(DefaultPath)
    }
}

/// Implementations of `Method` trait for "GET" method requests.
pub struct DefaultQueryMethod;

impl Method for DefaultQueryMethod {
    type PathParams = DefaultPath;
    type Request = DefaultQueryRequest;
    type Response = DefaultResponse;
}

/// Implementations of `Method` trait for requests other than "GET".
pub struct DefaultJsonMethod;

impl Method for DefaultJsonMethod {
    type PathParams = DefaultPath;
    type Request = DefaultJsonRequest;
    type Response = DefaultResponse;
}

/// Trait for typed REST handlers.
///
/// The `handler` method here is split into separate methods handling a different HTTP method each
/// like in SimpleRestHandler, but moreover the it's also generic over request and response types.
pub trait TypedRestHandler: Handler {
    /// Request and response types for "GET" method.
    type GetMethod: Method;

    /// Request and response types for "POST" method.
    type PostMethod: Method;

    /// Request and response types for "PUT" method.
    type PutMethod: Method;

    /// Request and response types for "PATCH" method.
    type PatchMethod: Method;

    /// Request and response types for "DELETE" method.
    type DeleteMethod: Method;

    /// Builds a response for methods other than "OPTIONS", "GET", "POST", "PUT", "PATCH" and
    /// "DELETE". The default response has empty body and "method not allowed" status code.
    fn make_default_response(&self, _request: Request) -> Response {
        DefaultResponse.into()
    }

    /// "OPTION" method request handler.
    fn options(&self, _context: &BlueFire, _request: Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::OK)
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE")
            .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, constants::BLUEFIRE_TOKEN_HEADER)
            .body(String::new())
            .expect("Build OPTIONS response body")
    }

    /// "GET" method request handler.
    fn get(
        &self,
        _context: &BlueFire,
        _request: Result<
            <<Self as TypedRestHandler>::GetMethod as Method>::Request,
            serde::de::value::Error,
        >,
        _path: Result<
            <<Self as TypedRestHandler>::GetMethod as Method>::PathParams,
            &'static str,
        >
    ) -> Reply<<<Self as TypedRestHandler>::GetMethod as Method>::Response>;

    /// "POST" method request handler.
    fn post(
        &self,
        _context: &BlueFire,
        _request: Result<
            <<Self as TypedRestHandler>::PostMethod as Method>::Request,
            serde_json::Error,
        >,
        _path: Result<
            <<Self as TypedRestHandler>::PostMethod as Method>::PathParams,
            &'static str,
        >
    ) -> Reply<<<Self as TypedRestHandler>::PostMethod as Method>::Response>;

    /// "PUT" method request handler.
    fn put(
        &self,
        _context: &BlueFire,
        _request: Result<
            <<Self as TypedRestHandler>::PutMethod as Method>::Request,
            serde_json::Error,
        >,
        _path: Result<
            <<Self as TypedRestHandler>::PutMethod as Method>::PathParams,
            &'static str,
        >
    ) -> Reply<<<Self as TypedRestHandler>::PutMethod as Method>::Response>;

    /// "PATCH" method request handler.
    fn patch(
        &self,
        _context: &BlueFire,
        _request: Result<
            <<Self as TypedRestHandler>::PatchMethod as Method>::Request,
            serde_json::Error,
        >,
        _path: Result<
            <<Self as TypedRestHandler>::PatchMethod as Method>::PathParams,
            &'static str,
        >
    ) -> Reply<<<Self as TypedRestHandler>::PatchMethod as Method>::Response>;

    /// "DELETE" method request handler.
    fn delete(
        &self,
        _context: &BlueFire,
        _request: Result<
            <<Self as TypedRestHandler>::DeleteMethod as Method>::Request,
            serde_json::Error,
        >,
        _path: Result<
            <<Self as TypedRestHandler>::DeleteMethod as Method>::PathParams,
            &'static str,
        >,
    ) -> Reply<<<Self as TypedRestHandler>::DeleteMethod as Method>::Response>;
}

/// Implements `Handler` trait for use with `TypedRestHandler` trait.
#[macro_export]
macro_rules! impl_handler_via_typed_handler {
    ($type:ty) => {
        impl bluefire_backend::Handler for $type {
            fn handle(
                &self,
                context: &bluefire_backend::BlueFire,
                request: bluefire_backend::Request,
            ) -> bluefire_backend::Response {
                let params = context.params();
                match request.method() {
                    &http::method::Method::OPTIONS => self.options(context, request),
                    &http::method::Method::GET => {
                        match self.get(context, request.try_into(), params.try_into()) {
                            Ok(response) => response.into(),
                            Err(response) => response.into(),
                        }
                    }
                    &http::method::Method::POST => {
                        match self.post(context, request.try_into(), params.try_into()) {
                            Ok(response) => response.into(),
                            Err(response) => response.into(),
                        }
                    }
                    &http::method::Method::PUT => {
                        match self.put(context, request.try_into(), params.try_into()) {
                            Ok(response) => response.into(),
                            Err(response) => response.into(),
                        }
                    }
                    &http::method::Method::PATCH => {
                        match self.patch(context, request.try_into(), params.try_into()) {
                            Ok(response) => response.into(),
                            Err(response) => response.into(),
                        }
                    }
                    &http::method::Method::DELETE => {
                        match self.delete(context, request.try_into(), params.try_into()) {
                            Ok(response) => response.into(),
                            Err(response) => response.into(),
                        }
                    }
                    _ => self.make_default_response(request),
                }
            }

            fn duplicate(&self) -> Box<dyn bluefire_backend::Handler> {
                Box::new(self.clone())
            }
        }
    };
}

/// Default implementation for "GET" method. Returns an empty message with "method not allowed"
/// code.
#[macro_export]
macro_rules! default_get_method {
    () => {
        type GetMethod = bluefire_backend::rest::DefaultQueryMethod;
        fn get(
            &self,
            _: &BlueFire,
            _: Result<bluefire_backend::rest::DefaultQueryRequest, serde::de::value::Error>,
            _: Result<bluefire_backend::rest::DefaultPath, &'static str>,
        ) -> bluefire_backend::rest::Reply<bluefire_backend::rest::DefaultResponse> {
            Ok(bluefire_backend::rest::DefaultResponse)
        }
    }
}

/// Default implementation for "POST" method. Returns an empty message with "method not allowed"
/// code.
#[macro_export]
macro_rules! default_post_method {
    () => {
        type PostMethod = bluefire_backend::rest::DefaultJsonMethod;
        fn post(
            &self,
            _: &BlueFire,
            _: Result<bluefire_backend::rest::DefaultJsonRequest, serde_json::Error>,
            _: Result<bluefire_backend::rest::DefaultPath, &'static str>,
        ) -> bluefire_backend::rest::Reply<bluefire_backend::rest::DefaultResponse> {
            Ok(bluefire_backend::rest::DefaultResponse)
        }
    }
}

/// Default implementation for "PUT" method. Returns an empty message with "method not allowed"
/// code.
#[macro_export]
macro_rules! default_put_method {
    () => {
        type PutMethod = bluefire_backend::rest::DefaultJsonMethod;
        fn put(
            &self,
            _: &BlueFire,
            _: Result<bluefire_backend::rest::DefaultJsonRequest, serde_json::Error>,
            _: Result<bluefire_backend::rest::DefaultPath, &'static str>,
        ) -> bluefire_backend::rest::Reply<bluefire_backend::rest::DefaultResponse> {
            Ok(bluefire_backend::rest::DefaultResponse)
        }
    }
}

/// Default implementation for "PATCH" method. Returns an empty message with "method not allowed"
/// code.
#[macro_export]
macro_rules! default_patch_method {
    () => {
        type PatchMethod = bluefire_backend::rest::DefaultJsonMethod;
        fn patch(
            &self,
            _: &BlueFire,
            _: Result<bluefire_backend::rest::DefaultJsonRequest, serde_json::Error>,
            _: Result<bluefire_backend::rest::DefaultPath, &'static str>,
        ) -> bluefire_backend::rest::Reply<bluefire_backend::rest::DefaultResponse> {
            Ok(bluefire_backend::rest::DefaultResponse)
        }
    }
}

/// Default implementation for "DELETE" method. Returns an empty message with "method not allowed"
/// code.
#[macro_export]
macro_rules! default_delete_method {
    () => {
        type DeleteMethod = bluefire_backend::rest::DefaultJsonMethod;
        fn delete(
            &self,
            _: &BlueFire,
            _: Result<bluefire_backend::rest::DefaultJsonRequest, serde_json::Error>,
            _: Result<bluefire_backend::rest::DefaultPath, &'static str>,
        ) -> bluefire_backend::rest::Reply<bluefire_backend::rest::DefaultResponse> {
            Ok(bluefire_backend::rest::DefaultResponse)
        }
    }
}
