// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Overlay widget.

use crate::elements::prelude::*;

use super::CLASS_NAMES as C;

// -------------------------------------------------------------------------------------------------

/// Represents an overlay element.
pub struct Overlay {
    element: Element,
}

impl Overlay {
    /// Constructs a new `Overlay` using the passed element.
    pub fn onto(element: Element) -> Self {
        Self { element }
    }

    /// Constructs a new `Overlay` using the element with passed ID.
    pub fn over(placeholder_id: &str) -> Self {
        let element = Element::get(placeholder_id);
        Self::onto(element)
    }

    /// Shows the overlay.
    pub fn show(&self) {
        if let Some(list) = self.element.get_class_list() {
            let _ = list.replace(C.bd_hidden, C.bd_overlay_fixed);
        }
    }

    /// Hides the overlay.
    pub fn hide(&self) {
        if let Some(list) = self.element.get_class_list() {
            let _ = list.replace(C.bd_overlay_fixed, C.bd_hidden);
        }
    }
}
