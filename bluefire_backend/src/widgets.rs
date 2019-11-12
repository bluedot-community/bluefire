// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains names of CSS classes and HTML generation helpers for elements provided by
//! `bluefire_static_files` crate.

use maud::html;

/// Contains names of all CSS classes provided by `bluefire_static_files` crate.
pub const CLASS_NAMES: bluefire_twine::ClassNames = bluefire_twine::ClassNames::new_constant();

// -------------------------------------------------------------------------------------------------

/// A helper strcture aiding in generation of simple HTML elements.
pub struct Widgets;

impl Widgets {
    /// Constructs new `Widgets`.
    pub fn new() -> Self {
        Self
    }

    /// Clickable button.
    pub fn button(&self, id: &str, text: &str) -> String {
        (html! { div.(CLASS_NAMES.bd_button)#(id) { (text) } }).into_string()
    }

    /// Editable entry field.
    pub fn field(&self, id: &str, placeholder: &str) -> String {
        (html! { input#(id).(CLASS_NAMES.bd_field) type="text" placeholder=(placeholder) {} })
            .into_string()
    }

    /// Editable text area.
    pub fn text_area(&self, id: &str) -> String {
        (html! { textarea#(id).(CLASS_NAMES.bd_field) {} }).into_string()
    }

    /// Simple box with predefined title and a text.
    pub fn board(&self, title: &str, text: &str) -> String {
        (html! {
            div.(CLASS_NAMES.bd_box).(CLASS_NAMES.bd_center) {
                div.(CLASS_NAMES.bd_title) { (title) }
                p { (text) }
            }
        })
        .into_string()
    }

    /// A big loader with ID.
    pub fn loader(&self, id: &str) -> String {
        (html! { div.(CLASS_NAMES.bd_loader_big)#(id) { "…" } }).into_string()
    }

    /// A big loader.
    pub fn loader_big(&self) -> String {
        (html! { div.(CLASS_NAMES.bd_loader_big) { "…" } }).into_string()
    }

    /// An overlay containing a big loader.
    pub fn loader_overlay(&self, id: &str) -> String {
        (html! { div#(id).(CLASS_NAMES.bd_hidden) { div.(CLASS_NAMES.bd_loader_big) { "…" } } })
            .into_string()
    }

    /// Encircled question mark with a help note.
    pub fn help_note(&self, text: &str) -> String {
        (html! {
            div.(CLASS_NAMES.bd_help) {
                div.(CLASS_NAMES.bd_help_center) { (text) }
            }
        })
        .into_string()
    }
}
