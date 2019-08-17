// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_backend::rest` module.

use std::convert::TryInto;

use bluefire_backend::{rest::TypedRestHandler, BlueFire};

#[derive(Clone, Debug)]
struct View;

impl TypedRestHandler for View {
    bluefire_backend::default_get_method!();
    bluefire_backend::default_post_method!();
    bluefire_backend::default_put_method!();
    bluefire_backend::default_patch_method!();
    bluefire_backend::default_delete_method!();
}

bluefire_backend::impl_handler_via_typed_handler!(View);
