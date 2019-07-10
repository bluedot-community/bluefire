// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Binding request types with paths they should be sent to and respond types.

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
    type Request: serde::Serialize;

    /// Response type.
    type Response: serde::Serialize;
}
