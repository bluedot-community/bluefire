// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Widgets for bluefire frontend.
//!
//! The widgets defined here correspond to some CSS classes provided by `bluefire_static_files`
//! crate.

/// Contains names of all CSS classes provided by `bluefire_static_files` crate.
pub const CLASS_NAMES: bluefire_twine::ClassNames = bluefire_twine::ClassNames::new_constant();

#[cfg(feature = "widgets_communicates")]
pub mod communicates;

#[cfg(feature = "widgets_list")]
pub mod list;

#[cfg(feature = "widgets_overlay")]
pub mod overlay;

#[cfg(feature = "widgets_tag_area")]
pub mod tag_area;
