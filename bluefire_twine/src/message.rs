// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Provides definition for `Message`.

use serde_derive::{Deserialize, Serialize};

/// A serialized message ready to be sent over HTTP.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// HTTP method of the request.
    pub method: &'static str,

    /// HTTP path.
    pub path: String,

    /// Query part of the URL.
    pub query: String,

    /// Content of the message.
    pub body: String,
}

impl Message {
    /// Constructs a new `Message`.
    pub fn new(method: &'static str, path: String, query: String, body: String) -> Self {
        Self { method, path, query, body }
    }

    /// Returns the HTTP method of the message.
    pub fn method(&self) -> &'static str {
        self.method
    }

    /// Returns the HTTP path of the message.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the query of the message.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the contents of the message.
    pub fn body(&self) -> &str {
        &self.body
    }
}
