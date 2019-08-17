// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! HTTP server functionality using `hyper`.

use futures::{future, Future, Stream};
use std::sync::{Arc, Mutex};

use crate::common;
use crate::context::{BlueFireKindler, BlueFireWielder};

pub struct BlueFireNewService {
    bluefire_kindler: BlueFireKindler,
}

impl BlueFireNewService {
    pub fn new(bluefire_kindler: BlueFireKindler) -> Self {
        BlueFireNewService { bluefire_kindler: bluefire_kindler }
    }
}

impl hyper::service::NewService for BlueFireNewService {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = hyper::Error;
    type InitError = hyper::Error;
    type Service = BlueFireService;
    type Future = Box<dyn future::Future<Item = Self::Service, Error = Self::InitError> + Send>;
    fn new_service(&self) -> Self::Future {
        log_debug!("BlueFire: staring a new service");
        let service = BlueFireService::new(Arc::new(Mutex::new(self.bluefire_kindler.kindle())));
        Box::new(future::ok(service))
    }
}

pub struct BlueFireService {
    bluefire_wielder: Arc<Mutex<BlueFireWielder>>,
}

impl BlueFireService {
    pub fn new(bluefire_wielder: Arc<Mutex<BlueFireWielder>>) -> Self {
        BlueFireService { bluefire_wielder }
    }
}

impl BlueFireService {
    fn repack_request(parts: http::request::Parts, data: Vec<u8>) -> common::Request {
        let new_body = String::from_utf8(data).unwrap();
        http::Request::from_parts(parts, new_body)
    }

    fn repack_response(resp: common::Response) -> http::Response<hyper::Body> {
        let (parts, original_body) = resp.into_parts();
        let new_body = hyper::Body::from(original_body);
        http::Response::from_parts(parts, new_body)
    }
}

impl hyper::service::Service for BlueFireService {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = hyper::Error;
    type Future =
        Box<dyn future::Future<Item = http::Response<hyper::Body>, Error = Self::Error> + Send>;

    fn call(&mut self, req: http::Request<Self::ReqBody>) -> Self::Future {
        let bluefire_wielder = self.bluefire_wielder.clone();
        let (parts, original_body) = req.into_parts();
        Box::new(original_body.concat2().and_then(move |data| {
            let req = Self::repack_request(parts, data.to_vec());
            let resp = {
                let mut bluefire = bluefire_wielder.lock().expect("Mutex lock");
                bluefire.serve(req)
            };
            let resp = Self::repack_response(resp);
            future::ok(resp)
        }))
    }
}
