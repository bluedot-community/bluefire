// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Functionality related to routing.
//!
//! TODO: More description needed.

use std::collections::HashMap;

use crate::common::*;
use crate::context::BlueFire;

// -------------------------------------------------------------------------------------------------

mod utils {
    use super::Request;

    pub fn extract_host_and_path(request: &Request) -> (Option<String>, String) {
        let uri = request.uri();
        let path = uri.path().to_string();
        let host_name = {
            if let Some(host_name) = uri.host() {
                Some(host_name.to_string())
            } else if let Some(host_name) = request.headers().get("Host") {
                host_name.to_str().ok().map(|name| name.to_string())
            } else {
                None
            }
        };

        (host_name, path)
    }
}

// -------------------------------------------------------------------------------------------------

const NOT_FOUND_BODY: &'static str = "<html>\
                                      <head><title>Not Found</title></head>\
                                      <body><h1>Not Found</h1></body>\
                                      </html>";

/// Fall-back handler for routes for which a handler was not defined.
#[derive(Clone, Debug)]
pub struct NotFoundHandler;

impl NotFoundHandler {
    /// Constructs a new `NotFoundHandler`.
    pub fn new() -> Box<dyn Handler> {
        Box::new(NotFoundHandler)
    }
}

impl Handler for NotFoundHandler {
    fn handle(&self, _context: &BlueFire, _request: Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::NOT_FOUND)
            .body(NOT_FOUND_BODY.into())
            .expect("Build response")
    }

    fn duplicate(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Scheme {
    Http,
    Https,
}

/// Represents scheme and host part in HTTP URI.
#[derive(Debug)]
pub struct Host {
    host_name: Option<&'static str>,
    not_found_handler: Box<dyn Handler>,
    scheme: Scheme,
}

impl Host {
    /// Constructs a new HTTP `Host`.
    pub fn http(host: &'static str) -> Host {
        Host {
            host_name: Some(host),
            not_found_handler: NotFoundHandler::new(),
            scheme: Scheme::Http,
        }
    }

    /// Constructs a new HTTPS `Host`.
    pub fn https(host: &'static str) -> Host {
        Host {
            host_name: Some(host),
            not_found_handler: NotFoundHandler::new(),
            scheme: Scheme::Https,
        }
    }

    /// Constructs a new `Host` without any specified name. The routing algorithm such host will
    /// match any host from request.
    pub fn new_nameless() -> Host {
        Host { host_name: None, not_found_handler: NotFoundHandler::new(), scheme: Scheme::Http }
    }

    /// Sets a default handler for routes for which handler was not defined.
    pub fn with_not_found_handler(mut self, handler: Box<dyn Handler>) -> Host {
        self.not_found_handler = handler;
        self
    }

    /// Returns the default handler.
    pub fn get_not_found_handler(&self) -> &Box<dyn Handler> {
        &self.not_found_handler
    }

    /// Returns the name of the host.
    pub fn get_host_name(&self) -> Option<String> {
        if let Some(host_name) = self.host_name {
            match self.scheme {
                Scheme::Http => Some(String::from("http://") + &host_name),
                Scheme::Https => Some(String::from("https://") + &host_name),
            }
        } else {
            None
        }
    }
}

impl Clone for Host {
    fn clone(&self) -> Self {
        Host {
            host_name: self.host_name,
            not_found_handler: self.not_found_handler.duplicate(),
            scheme: self.scheme,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Exact { name: &'static str },
    Param { name: &'static str },
    Index,
}

// -------------------------------------------------------------------------------------------------

/// Represents a whole path to some HTTP resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    host_name: Option<String>,
    segments: Vec<Segment>,
}

impl Path {
    /// Constructs a new `Path`.
    pub fn new(host_name: Option<String>) -> Path {
        Path { host_name: host_name, segments: Vec::new() }
    }

    /// Given a mapping from parameter name to parameter value returns a string representation of
    /// this path (not including the host name).
    pub fn as_path(&self, params: &HashMap<&'static str, String>) -> String {
        let mut result = String::new();
        for segment in self.segments.iter() {
            match segment {
                Segment::Exact { name } => {
                    result.push('/');
                    result.push_str(&name);
                }
                Segment::Param { name } => {
                    result.push('/');
                    if let Some(value) = params.get(name) {
                        result.push_str(value);
                    } else {
                        log_warn!("Parameter '{}' not found in path parameters", name);
                    }
                }
                Segment::Index => {}
            }
        }
        result
    }

    /// Returns a string representation of this path (not including the host name) assuming no
    /// parameters are needed to correctly reconstruct the path.
    pub fn as_path_no_params(&self) -> String {
        self.as_path(&HashMap::new())
    }

    /// Given a mapping from parameter name to parameter value returns a string representation of
    /// this path (including the host name).
    pub fn as_uri(&self, params: &HashMap<&'static str, String>) -> String {
        if let Some(host_name) = &self.host_name {
            host_name.clone() + &self.as_path(params)
        } else {
            self.as_path(params)
        }
    }

    /// Returns a string representation of this path (including the host name) assuming no
    /// parameters are needed to correctly reconstruct the path.
    pub fn as_uri_no_params(&self) -> String {
        self.as_uri(&HashMap::new())
    }
}

impl Path {
    fn push(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    fn pop(&mut self) {
        self.segments.pop();
    }
}

// -------------------------------------------------------------------------------------------------

/// Describes the type of match between path segments in the routing algorithm.
#[derive(Clone, Debug)]
enum RouteMatch {
    /// Matched to an exact segment.
    Exact,

    /// Matched to a parametrized segment. The `name` is the name of the parameter segment.
    Param { name: &'static str },

    /// Did not match.
    NoMatch,
}

/// A node in tree-like structure describing served HTTP resources.
#[derive(Debug)]
pub struct Route {
    segment: Segment,
    handler: Option<Box<dyn Handler>>,
    routes: Vec<Route>,
    label: Option<String>,
}

impl Route {
    /// Constructs a new exact `Route`.
    pub fn exact(name: &'static str) -> Route {
        Route::new(Segment::Exact { name })
    }

    /// Constructs a new parametrized `Route`.
    pub fn param(name: &'static str) -> Route {
        Route::new(Segment::Param { name })
    }

    /// Constructs a new index `Route`.
    pub fn index() -> Route {
        Route::new(Segment::Index)
    }

    /// Checks if the given route is an index route.
    pub fn is_index(&self) -> bool {
        match self.segment {
            Segment::Index => true,
            _ => false,
        }
    }

    /// Sets the handler for requests.
    pub fn with_view(mut self, view: Box<dyn Handler>) -> Route {
        self.handler = Some(view);
        self
    }

    /// Adds sub-routes.
    pub fn with_routes(mut self, routes: Vec<Route>) -> Route {
        self.routes = routes;
        self
    }

    /// Sets the label for the route. Label should be a unique identifier of the route. It can be
    /// use to obtain an object describing whole path to the HTTP resource represented by the
    /// `Route`.
    pub fn with_label(mut self, label: &str) -> Route {
        self.label = Some(label.to_string());
        self
    }

    /// Changes the type of this route to `exact`.
    pub fn as_exact(mut self, name: &'static str) -> Route {
        assert!(self.is_index());
        self.segment = Segment::Exact { name };
        self
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        let handler = {
            if let Some(ref handler) = self.handler {
                Some(handler.duplicate())
            } else {
                None
            }
        };

        Route {
            segment: self.segment.clone(),
            handler: handler,
            routes: self.routes.clone(),
            label: self.label.clone(),
        }
    }
}

impl Route {
    fn new(segment: Segment) -> Route {
        Route { segment: segment, handler: None, routes: Vec::new(), label: None }
    }

    fn get_handler(&self) -> Option<&Box<dyn Handler>> {
        self.handler.as_ref()
    }

    fn match_segment(&self, segment: &str) -> RouteMatch {
        match self.segment {
            Segment::Exact { name } => {
                if segment == name {
                    RouteMatch::Exact
                } else {
                    RouteMatch::NoMatch
                }
            }
            Segment::Param { name } => RouteMatch::Param { name },
            Segment::Index => {
                if segment.is_empty() {
                    RouteMatch::Exact
                } else {
                    RouteMatch::NoMatch
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// `Router` allows to find an appropriate handler for a request.
pub struct Router {
    routes: HashMap<Option<String>, (Host, Route)>,
    not_found_handler: Box<dyn Handler>,
}

impl Router {
    /// For a given request, basing on its path returns
    ///  - an appropriate handler for the request and
    ///  - a map parameters extracted from the path.
    pub fn route<'a, 'b>(&'a self, request: &'b Request) -> (&'a Box<dyn Handler>, ParamsMap) {
        let mut params = ParamsMap::new();
        let (host_name, path) = utils::extract_host_and_path(request);

        if let Some((host, toplevel_route)) = self.get_host(&host_name) {
            let mut routes = &toplevel_route.routes;
            let mut handler = toplevel_route.get_handler();
            for segment in path.split("/") {
                if segment.is_empty() {
                    continue;
                }
                handler = None;

                let mut found = false;
                for route in routes.iter() {
                    match route.match_segment(segment) {
                        RouteMatch::Exact => {
                            found = true;
                            routes = &route.routes;
                            handler = route.get_handler();
                            break;
                        }
                        RouteMatch::Param { name } => {
                            found = true;
                            routes = &route.routes;
                            params.insert(name, segment.to_string());
                            handler = route.get_handler();
                            break;
                        }
                        RouteMatch::NoMatch => {}
                    }
                }

                if !found {
                    return (host.get_not_found_handler(), params);
                }
            }

            if let Some(handler) = handler {
                (handler, params)
            } else {
                (host.get_not_found_handler(), params)
            }
        } else {
            log_error!("Received a request for not configured host '{:?}'", host_name);
            (&self.not_found_handler, params)
        }
    }
}

impl Router {
    fn get_host<'a, 'b>(&'a self, host_name: &'b Option<String>) -> Option<&'a (Host, Route)> {
        let result = self.routes.get(host_name);
        if result.is_none() && host_name.is_some() {
            self.routes.get(&None)
        } else {
            result
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Reverse router provides mapping from labels assigned to routes to and object allowing to
/// reconstruct the path to the resource they represent.
pub struct ReverseRouter {
    paths: HashMap<String, Path>,
}

impl ReverseRouter {
    /// Returns the path for given label if the label was defined.
    pub fn reverse(&self, label: &str) -> Option<&Path> {
        self.paths.get(label)
    }
}

// -------------------------------------------------------------------------------------------------

/// Builder for the server router and reverse router.
pub struct RoutingBuilder {
    routes: HashMap<Option<String>, (Host, Route)>,
}

impl RoutingBuilder {
    /// Constructs a new `RoutingBuilder`.
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    /// Adds a new host with its routes.
    pub fn insert(&mut self, host: Host, route: Route) {
        self.routes.insert(host.host_name.map(|name| name.to_string()), (host, route));
    }

    /// Builds the router and the reverse router.
    pub fn build(&self) -> (Router, ReverseRouter) {
        let not_found_handler = NotFoundHandler::new();
        let routes = self.routes.clone();

        let mut paths = HashMap::new();
        for (host, route) in self.routes.values() {
            let mut path = Path::new(host.get_host_name());
            Self::construct_paths(&mut paths, &route, &mut path);
        }

        (Router { routes, not_found_handler }, ReverseRouter { paths })
    }
}

impl RoutingBuilder {
    fn construct_paths(paths: &mut HashMap<String, Path>, route: &Route, path: &mut Path) {
        path.push(route.segment.clone());

        if let Some(ref label) = route.label {
            paths.insert(label.clone(), path.clone());
        }

        for r in route.routes.iter() {
            Self::construct_paths(paths, &r, path);
        }

        path.pop();
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{Path, Segment};
    use std::collections::HashMap;

    #[test]
    fn test_constructing_path() {
        let mut path = Path::new(Some("http://host".to_string()));
        path.push(Segment::Index);
        path.push(Segment::Exact { name: "abc" });
        path.push(Segment::Param { name: "xyz" });
        path.push(Segment::Exact { name: "ghi" });
        path.push(Segment::Param { name: "uvw" });
        path.push(Segment::Exact { name: "mno" });
        let mut params = HashMap::new();
        params.insert("xyz", "def".to_string());
        params.insert("uvw", "jkl".to_string());
        assert_eq!(path.as_path(&params), "/abc/def/ghi/jkl/mno");
        assert_eq!(path.as_path_no_params(), "/abc//ghi//mno");
        assert_eq!(path.as_uri(&params), "http://host/abc/def/ghi/jkl/mno");
        assert_eq!(path.as_uri_no_params(), "http://host/abc//ghi//mno");
    }
}
