// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Main back-end functionality:
//! - middleware
//! - extensions
//! - configuring the back-end
//! - request context

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(feature = "server")]
use futures::Future;
use traitobject;

#[cfg(feature = "server")]
use crate::server;

use crate::clock::{Clock, ClockExtension};
use crate::common;
use crate::router;

// -------------------------------------------------------------------------------------------------

/// A trait for additional request processors executed before the main request handler.
pub trait Middleware: Send {
    /// Notifies the middleware about the request.
    fn apply(&mut self, extensions: &mut Extensions, request: &common::Request);

    /// Makes a copy of the middleware.
    fn duplicate(&self) -> Box<dyn Middleware>;
}

/// A trait for extensions of the context functionality.
pub trait Extension: Any + Debug + Send {
    /// Returns the name of the extension.
    fn get_name(&self) -> &str;

    /// Checks if the extension is functional.
    fn check(&self) -> Result<(), ()>;

    /// Makes a copy of the extension.
    fn duplicate(&self) -> Box<dyn Extension>;

    /// Notifies the extension about the destruction.
    fn destroy(&self);
}

// -------------------------------------------------------------------------------------------------

/// A storage for extensions. The extensions are identified by the type so only one extension of a
/// given type can be stored at a time.
#[derive(Debug)]
pub struct Extensions {
    data: HashMap<TypeId, Box<dyn Extension>>,
}

impl Extensions {
    /// Adds an extension.
    pub fn add<E: Extension>(&mut self, extension: E) {
        self.data.insert(TypeId::of::<E>(), Box::new(extension));
    }

    /// Returns an extension as immutable.
    pub fn get<E: Extension>(&self) -> Option<&E> {
        match self.data.get(&TypeId::of::<E>()) {
            Some(e) => Some(unsafe { std::mem::transmute(traitobject::data(e.as_ref())) }),
            None => None,
        }
    }

    /// Returns an extension as mutable.
    pub fn get_mut<E: Extension>(&mut self) -> Option<&mut E> {
        match self.data.get_mut(&TypeId::of::<E>()) {
            Some(e) => Some(unsafe { std::mem::transmute(traitobject::data_mut(e.as_mut())) }),
            None => None,
        }
    }
}

impl Extensions {
    fn new() -> Self {
        Extensions { data: HashMap::new() }
    }

    fn duplicate(&self) -> Extensions {
        let mut extensions = HashMap::new();
        for (&type_id, extension) in self.data.iter() {
            extensions.insert(type_id, extension.duplicate());
        }

        Extensions { data: extensions }
    }
}

// -------------------------------------------------------------------------------------------------

/// Builder for `BlueFireWielder`.
pub struct BlueFireKindler {
    extensions: Extensions,
    middlewares: Vec<Box<dyn Middleware>>,
    router: Arc<router::Router>,
    reverse_router: Arc<router::ReverseRouter>,
}

impl BlueFireKindler {
    /// Constructs a new `BlueFireKindler`.
    pub fn start(routing_builder: Box<router::RoutingBuilder>) -> Self {
        let (router, reverse_router) = routing_builder.build();
        let mut extensions = Extensions::new();
        extensions.add(ClockExtension::new_utc());

        BlueFireKindler {
            extensions: extensions,
            middlewares: Vec::new(),
            router: Arc::new(router),
            reverse_router: Arc::new(reverse_router),
        }
    }

    /// Adds a middleware.
    pub fn wire(mut self, middleware: Box<dyn Middleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Add an extensions.
    pub fn extend<E: Extension>(mut self, extension: E) -> Self {
        self.extensions.add::<E>(extension);
        self
    }

    /// Checks if all extensions are functional.
    pub fn perform_checks(&self) {
        log::info!(" => Checking the extensions:");
        for (_id, extension) in self.extensions.data.iter() {
            if extension.check().is_ok() {
                log::info!("  -> {}: ok", extension.get_name());
            } else {
                log::info!("  -> {}: NOT OK", extension.get_name());
            }
        }
    }

    /// Constructs a new `BlueFireWielder`.
    pub fn kindle(&self) -> BlueFireWielder {
        // TODO: Optimize context copying
        BlueFireWielder {
            middlewares: self.duplicate_middlewares(),
            router: self.router.clone(),
            context: BlueFire {
                extensions: self.duplicate_extensions(),
                params: common::ParamsMap::default(),
                reverse_router: self.reverse_router.clone(),
            },
        }
    }
}

impl Clone for BlueFireKindler {
    fn clone(&self) -> BlueFireKindler {
        BlueFireKindler {
            extensions: self.duplicate_extensions(),
            middlewares: self.duplicate_middlewares(),
            router: self.router.clone(),
            reverse_router: self.reverse_router.clone(),
        }
    }
}

#[cfg(feature = "server")]
impl BlueFireKindler {
    /// Starts an HTTP server on the given address.
    pub fn ignite_server(self, addr: &std::net::SocketAddr) {
        self.perform_checks();
        log::info!(" => Ignite the BlueFire");
        let srv = hyper::Server::bind(addr)
            .serve(server::BlueFireNewService::new(self))
            .map_err(|e| log_error!("Server error: {}", e));

        hyper::rt::run(srv);
    }
}

impl BlueFireKindler {
    /// Clones the extensions.
    fn duplicate_extensions(&self) -> Extensions {
        self.extensions.duplicate()
    }

    /// Clones the middlewares.
    fn duplicate_middlewares(&self) -> Vec<Box<dyn Middleware>> {
        let mut middlewares = Vec::with_capacity(self.middlewares.len());
        for middleware in self.middlewares.iter() {
            middlewares.push(middleware.duplicate());
        }
        middlewares
    }
}

// -------------------------------------------------------------------------------------------------

/// Main server object. Handles requests and prepares handler context - `BlueFire`.
pub struct BlueFireWielder {
    middlewares: Vec<Box<dyn Middleware>>,
    router: Arc<router::Router>,
    context: BlueFire,
}

impl BlueFireWielder {
    /// Notifies all the middlewares about the request.
    pub fn apply_middlewares(&mut self, request: &common::Request) {
        for ref mut middleware in self.middlewares.iter_mut() {
            middleware.apply(&mut self.context.extensions, request);
        }
    }

    /// Finds a handler for the request basing on the request path and executes it.
    pub fn route(&mut self, request: common::Request) -> common::Response {
        let (handler, params) = self.router.route(&request);
        self.context.params = params;
        handler.handle(&mut self.context, request)
    }

    /// Handles the request - notifies the middlewares and executes the handler.
    pub fn serve(&mut self, request: common::Request) -> common::Response {
        self.apply_middlewares(&request);
        self.route(request)
    }

    /// Returns immutable handler context.
    pub fn get_context(&self) -> &BlueFire {
        &self.context
    }

    /// Returns mutable handler context.
    pub fn get_context_mut(&mut self) -> &mut BlueFire {
        &mut self.context
    }
}

// -------------------------------------------------------------------------------------------------

/// The context passed to handlers of all requests.
pub struct BlueFire {
    extensions: Extensions,
    params: common::ParamsMap,
    reverse_router: Arc<router::ReverseRouter>,
}

impl BlueFire {
    /// Adds an extension.
    pub fn extend<E: Extension>(&mut self, extension: E) {
        self.extensions.add::<E>(extension);
    }

    /// Returns an immutable extension.
    pub fn extension<E: Extension>(&self) -> Option<&E> {
        self.extensions.get::<E>()
    }

    /// Returns a mutable extension.
    pub fn extension_mut<E: Extension>(&mut self) -> Option<&mut E> {
        self.extensions.get_mut::<E>()
    }

    /// The parameters extracted from the path of the currently handled request.
    pub fn params(&self) -> &common::ParamsMap {
        &self.params
    }

    /// Returns a path for given label. `Path` allows to build a path to an HTTP resource.
    pub fn reverse(&self, label: &str) -> Option<&router::Path> {
        self.reverse_router.reverse(label)
    }

    /// Returns and implementation of clock. (Needed for testing.)
    pub fn clock(&self) -> &Box<dyn Clock> {
        self.extension::<ClockExtension>().expect("No clock extension").get_implementation()
    }
}
