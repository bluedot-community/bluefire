// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module facilitates the use of JavaScript promises in Rust.

use js_sys::Promise;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

// -------------------------------------------------------------------------------------------------

/// A Trait for all elements of the flow.
pub trait Plumbing {
    /// A type returned by `and` method.
    type AndHose: Plumbing;

    /// Adds a handler called if the previous action finished successfully.
    fn and<TC>(self, callback: TC) -> Self::AndHose
    where
        TC: Valve + 'static;
}

/// A result of execution of one step in a flow.
///
/// An error value will stop the flow. If an ok value with a promise is retuned and the floe
/// defines further action, the flow will be continued.
pub type FlowResult = Result<Option<Promise>, ()>;

// -------------------------------------------------------------------------------------------------

/// A trait that facilitates using both closures and full structures as callbacks in flow.
pub trait Valve {
    /// Executes the callback.
    fn valve(&mut self, value: JsValue) -> FlowResult;
}

impl<F> Valve for F
where
    F: Fn(JsValue) -> FlowResult,
{
    fn valve(&mut self, value: JsValue) -> FlowResult {
        self.call((value,))
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure helps defining and directing the flow of execution.
pub struct Flow {
    promise: Promise,
}

impl Flow {
    /// Constructs a new `Flow` from a `Promise`.
    pub fn start(promise: Promise) -> Flow {
        Self { promise }
    }
}

impl Plumbing for Flow {
    type AndHose = Hose;

    fn and<TC>(self, callback: TC) -> Self::AndHose
    where
        TC: Valve + 'static,
    {
        let hose = Hose::new(callback);
        let closure_stream = hose.stream.clone();
        let closure = Closure::new(move |arg| {
            let mut stream = closure_stream.borrow_mut();
            let result = stream.callback.valve(arg);
            stream.flow(result);
        });
        self.promise.then(&closure);
        hose.stream.borrow_mut().closure = Some(closure);
        hose
    }
}

// -------------------------------------------------------------------------------------------------

/// A helper structure for defining and directing the flow of execution.
#[derive(Clone)]
pub struct Hose {
    stream: Rc<RefCell<Stream>>,
}

impl Hose {
    fn new<TC>(callback: TC) -> Hose
    where
        TC: Valve + 'static,
    {
        Hose { stream: Rc::new(RefCell::new(Stream::new(Box::new(callback)))) }
    }
}

impl Plumbing for Hose {
    type AndHose = Hose;

    fn and<TC>(self, callback: TC) -> Self::AndHose
    where
        TC: Valve + 'static,
    {
        let hose = Hose::new(callback);
        self.stream.borrow_mut().success = Some(hose.clone());
        hose
    }
}

// -------------------------------------------------------------------------------------------------

struct Stream {
    closure: Option<Closure<dyn FnMut(JsValue)>>,
    callback: Box<dyn Valve>,
    success: Option<Hose>,
}

impl Stream {
    pub fn new(callback: Box<dyn Valve>) -> Self {
        Self { closure: None, callback: callback, success: None }
    }

    pub fn flow(&mut self, result: FlowResult) {
        match result {
            Ok(Some(promise)) => {
                if let Some(hose) = self.success.take() {
                    let closure_stream = hose.stream.clone();
                    let closure = Closure::new(move |arg| {
                        let mut stream = closure_stream.borrow_mut();
                        let result = stream.callback.valve(arg);
                        stream.flow(result);
                    });
                    promise.then(&closure);
                    hose.stream.borrow_mut().closure = Some(closure);
                } else {
                    web_debug!("bluefire flow: unused promise");
                }
            }
            Ok(None) => {
                if self.success.take().is_some() {
                    web_debug!("bluefire flow: leaking hose");
                } else {
                    // Everything's went fine.
                    // web_debug!("bluefire flow: ok");
                }
            }
            Err(..) => {
                web_debug!("bluefire flow: unhandled error");
            }
        }
        self.closure = None;
    }
}

// -------------------------------------------------------------------------------------------------

/// Prelude for `flow` module.
pub mod prelude {
    pub use super::{Flow, FlowResult, Hose, Plumbing, Valve};
}
