// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Helper structures for handling static files.
//!
//! Mainly for use with `bluefire_static_files_macros`.

use crate::{
    common::{Handler, Request, Response},
    context::BlueFire,
};

/// Handler for static files. Takes care for adding content type and cache related headers.
#[derive(Clone, Debug)]
pub struct StaticHandler {
    /// The content to be returned.
    content: Vec<u8>,

    /// Content type.
    content_type: String,

    /// Time of creation of this handler.
    last_modified: String,
}

impl StaticHandler {
    /// Constructs a new `StaticHandler`.
    pub fn new(content: Vec<u8>, content_type: String) -> Self {
        let last_modified = chrono::Utc::now().format("%a, %d %m %Y %H:%M:%S GMT").to_string();
        Self { content, content_type, last_modified }
    }
}

impl Handler for StaticHandler {
    fn handle(&self, _context: &BlueFire, _request: Request) -> Response {
        http::response::Builder::new()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, &self.content_type)
            .header(http::header::LAST_MODIFIED, &self.last_modified)
            .header(http::header::CACHE_CONTROL, "public")
            .body(self.content.clone())
            .expect("Build response")
    }

    fn duplicate(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}
