// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! HTML elements.
//!
//! Structures contained in this module provide access to HTML element methods. These structures do
//! not create any new nodes, instead are just wrappers around existing nodes. They also can't
//! track lifetime of HTML elements, so are meant to only be a short-lived helpers.

/// Keyboard event key codes.
pub mod keycode {
    /// Code for `enter` key.
    pub const ENTER: u32 = 13;
}

macro_rules! on {
    ($self:ident, $event_name:literal, $callback:ident) => {
        if let Some(ref element) = $self.element {
            let closure = Closure::wrap($callback);
            let result = element
                .add_event_listener_with_callback($event_name, closure.as_ref().unchecked_ref());
            if let Err(err) = result {
                web_error!("bluefire: failed to add event listener: {:?}", err);
            }
            closure.forget();
        }
    };
}

/// This module contains functionality related to a generic HTML element.
pub mod element {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Represents a view into a generic HTML element.
    pub struct Element {
        element: Option<web_sys::Element>,
    }

    impl Element {
        /// Check if the two elements represent the same HTML node.
        pub fn are_the_same(element1: &Element, element2: &Element) -> bool {
            if let Some(ref e1) = element1.element {
                if let Some(ref e2) = element2.element {
                    e1.is_same_node(Some(&e2.clone().into()))
                } else {
                    false
                }
            } else {
                false
            }
        }

        /// Constructs a new `Element`.
        /// Prints a warning on the console if the element does not exist.
        pub fn get(id: &str) -> Self {
            let element = crate::web::document().get_element_by_id(id);
            if element.is_none() {
                web_warn!("bluefire: element '{}' does not exist", id);
            }
            Self { element }
        }

        /// Constructs a new `Element`.
        pub fn get_optional(id: &str) -> Self {
            Self { element: crate::web::document().get_element_by_id(id) }
        }

        /// Constructs a new `Element` from an event target.
        pub fn from_event(event: &web_sys::Event) -> Self {
            let element = if let Some(target) = event.target() {
                target.dyn_ref::<web_sys::Element>().map(|e| e.clone())
            } else {
                web_warn!("bluefire: event target does not exist");
                None
            };
            Self { element }
        }

        /// Checks if the HTML element exists.
        pub fn exists(&self) -> bool {
            self.element.is_some()
        }

        /// Returns a parent element.
        pub fn parent(&self) -> Element {
            if let Some(ref element) = self.element {
                Element { element: element.parent_element() }
            } else {
                Element { element: None }
            }
        }

        /// Returns a vector of children IDs.
        pub fn get_children_ids(&self) -> Vec<String> {
            if let Some(ref element) = self.element {
                let children = element.children();
                let mut ids = Vec::with_capacity(children.length() as usize);
                for i in 0..children.length() {
                    if let Some(element) = children.item(i) {
                        ids.push(element.id());
                    }
                }
                ids
            } else {
                Vec::new()
            }
        }

        /// Return a list of style classes if any.
        pub fn get_class_list(&self) -> Option<web_sys::DomTokenList> {
            self.element.as_ref().map(|element| element.class_list())
        }

        /// Returns the bounding client rectangle.
        pub fn get_bounding_client_rect(&self) -> Option<web_sys::DomRect> {
            self.element.as_ref().map(|element| element.get_bounding_client_rect())
        }

        /// Sets the text content of an element. The text will not be interpreted as HTML.
        pub fn set_text(&self, text: Option<&str>) {
            if let Some(ref element) = self.element {
                element.set_text_content(text);
            }
        }

        /// Sets the inner-HTML of an element. The text will be interpreted as HTML.
        pub fn set_html(&self, html: &str) {
            if let Some(ref element) = self.element {
                element.set_inner_html(html);
            }
        }

        /// Sets focus on the element.
        pub fn focus(&self) {
            if let Some(ref element) = self.element {
                match element.dyn_ref::<web_sys::HtmlElement>() {
                    Some(html_element) => {
                        let _ = html_element.focus();
                    }
                    None => web_warn!("bluefire: this element is not an HTML element"),
                }
            }
        }

        /// Inserts given HTML before of the element.
        pub fn insert_before(&self, html: &str) {
            if let Some(ref element) = self.element {
                if let Err(err) = element.insert_adjacent_html("beforebegin", html) {
                    web_error!("bluefire: insert element: {:?}", err);
                }
            }
        }
        /// Inserts given HTML before the first child of the elemennt.
        pub fn insert_front(&self, html: &str) {
            if let Some(ref element) = self.element {
                if let Err(err) = element.insert_adjacent_html("afterbegin", html) {
                    web_error!("bluefire: insert element: {:?}", err);
                }
            }
        }

        /// Inserts given HTML after the last child of the element.
        pub fn insert_end(&self, html: &str) {
            if let Some(ref element) = self.element {
                if let Err(err) = element.insert_adjacent_html("beforeend", html) {
                    web_error!("bluefire: insert element: {:?}", err);
                }
            }
        }

        /// Inserts given HTML after the element.
        pub fn insert_after(&self, html: &str) {
            if let Some(ref element) = self.element {
                if let Err(err) = element.insert_adjacent_html("afterend", html) {
                    web_error!("bluefire: insert element: {:?}", err);
                }
            }
        }

        /// Inserts given element before of the element.
        pub fn place_before(&self, element: &Element) {
            if let Some(ref target) = self.element {
                if let Some(ref source) = element.element {
                    if let Err(err) = target.before_with_node_1(source) {
                        web_error!("bluefire: place element: {:?}", err);
                    }
                }
            }
        }

        /// Inserts given element before the first child of the elemennt.
        pub fn place_front(&self, element: &Element) {
            if let Some(ref target) = self.element {
                if let Some(ref source) = element.element {
                    if let Err(err) = target.prepend_with_node_1(source) {
                        web_error!("bluefire: place element: {:?}", err);
                    }
                }
            }
        }

        /// Inserts given element after the last child of the element.
        pub fn place_end(&self, element: &Element) {
            if let Some(ref target) = self.element {
                if let Some(ref source) = element.element {
                    if let Err(err) = target.append_with_node_1(source) {
                        web_error!("bluefire: place element: {:?}", err);
                    }
                }
            }
        }

        /// Inserts given element after the element.
        pub fn place_after(&self, element: &Element) {
            if let Some(ref target) = self.element {
                if let Some(ref source) = element.element {
                    if let Err(err) = target.after_with_node_1(source) {
                        web_error!("bluefire: place element: {:?}", err);
                    }
                }
            }
        }

        /// Removes the element.
        pub fn remove(&self) {
            if let Some(ref element) = self.element {
                element.remove();
            }
        }

        /// Sets a callback to be executed when the element is clicked.
        pub fn on_click(&self, callback: Box<dyn Fn(web_sys::MouseEvent)>) {
            on!(self, "click", callback);
        }

        /// Sets a callback to be executed when the element is dragged.
        pub fn on_dragstart(&self, callback: Box<dyn Fn(web_sys::DragEvent)>) {
            on!(self, "dragstart", callback);
        }

        /// Sets a callback to be executed when the elements drag ends.
        pub fn on_dragend(&self, callback: Box<dyn Fn(web_sys::DragEvent)>) {
            on!(self, "dragend", callback);
        }

        /// Sets a callback to be executed when a dragged item hovers the element.
        pub fn on_dragenter(&self, callback: Box<dyn Fn(web_sys::DragEvent)>) {
            on!(self, "dragenter", callback);
        }

        /// Sets a callback to be executed when a dragged item hovers the element.
        pub fn on_dragover(&self, callback: Box<dyn Fn(web_sys::DragEvent)>) {
            on!(self, "dragover", callback);
        }

        /// Sets a callback to be executed when a dragged item is dropped on the element.
        pub fn on_drop(&self, callback: Box<dyn Fn(web_sys::DragEvent)>) {
            on!(self, "drop", callback);
        }
    }
}

/// This module contains functionality related to HTML `input` elements.
#[cfg(feature = "elements_input")]
pub mod input {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Represents a view into an HTML `input` element.
    pub struct Input {
        element: Option<web_sys::HtmlInputElement>,
    }

    impl Input {
        /// Constructs a new `Input`.
        /// Prints a warning on the console if the element does not exist.
        pub fn get(id: &str) -> Self {
            let element = if let Some(element) = crate::web::document().get_element_by_id(id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => Some(input_element),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", id);
                        None
                    }
                }
            } else {
                web_error!("bluefire: element '{}' does not exist", id);
                None
            };
            Self { element }
        }

        /// Constructs a new `Input`.
        pub fn get_optional(id: &str) -> Self {
            let element = crate::web::document()
                .get_element_by_id(id)
                .map(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                .flatten();
            Self { element }
        }

        /// Checks if the HTML element exists and is an `input` element.
        pub fn exists(&self) -> bool {
            self.element.is_some()
        }

        /// Returns the value of the input.
        pub fn get_value(&self) -> String {
            if let Some(ref element) = self.element {
                element.value()
            } else {
                String::default()
            }
        }

        /// Returns the value of the input and clears it.
        pub fn take_value(&self) -> String {
            if let Some(ref element) = self.element {
                let value = element.value();
                element.set_value("");
                value
            } else {
                String::default()
            }
        }

        /// Sets the value of the input.
        pub fn set_value(&self, value: &str) {
            if let Some(ref element) = self.element {
                element.set_value(value);
            }
        }

        /// Checks if the radio- or check-box input is checked.
        pub fn is_checked(&self) -> bool {
            if let Some(ref element) = self.element {
                element.checked()
            } else {
                false
            }
        }

        /// Sets the checked-state of a radio- or check-box input.
        pub fn set_checked(&self, checked: bool) {
            if let Some(ref element) = self.element {
                element.set_checked(checked);
            }
        }

        /// Sets a callback to be executed when the value of the input changes.
        pub fn on_change(&self, callback: Box<dyn Fn(web_sys::Event)>) {
            on!(self, "change", callback);
        }

        /// Sets a callback to be executed when the value of the input changes.
        pub fn on_keyup(&self, callback: Box<dyn Fn(web_sys::KeyboardEvent)>) {
            on!(self, "keyup", callback);
        }
    }
}

/// This module contains functionality related to HTML `select` elements.
#[cfg(feature = "elements_select")]
pub mod select {
    use wasm_bindgen::JsCast;

    /// Represents a view into an HTML `select` element.
    pub struct Select {
        element: Option<web_sys::HtmlSelectElement>,
    }

    impl Select {
        /// Constructs a new `Select`.
        /// Prints a warning on the console if the element does not exist.
        pub fn get(id: &str) -> Self {
            let element = if let Some(element) = crate::web::document().get_element_by_id(id) {
                match element.dyn_into::<web_sys::HtmlSelectElement>() {
                    Ok(select_element) => Some(select_element),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not a select element", id);
                        None
                    }
                }
            } else {
                web_error!("bluefire: element '{}' does not exist", id);
                None
            };
            Self { element }
        }

        /// Constructs a new `Select`.
        pub fn get_optional(id: &str) -> Self {
            let element = crate::web::document()
                .get_element_by_id(id)
                .map(|element| element.dyn_into::<web_sys::HtmlSelectElement>().ok())
                .flatten();
            Self { element }
        }

        /// Checks if the HTML element exists and is a `select` element.
        pub fn exists(&self) -> bool {
            self.element.is_some()
        }

        /// Returns the value of the selected element.
        pub fn get_value(&self) -> String {
            if let Some(ref element) = self.element {
                element.value()
            } else {
                String::default()
            }
        }
    }
}

#[cfg(feature = "elements_textarea")]
mod textarea {
    use wasm_bindgen::JsCast;

    /// Represents a view into an HTML `textarea` element.
    pub struct TextArea {
        element: Option<web_sys::HtmlTextAreaElement>,
    }

    impl TextArea {
        /// Constructs a new `Select`.
        /// Prints a warning on the console if the element does not exist.
        pub fn get(id: &str) -> Self {
            let element = if let Some(element) = crate::web::document().get_element_by_id(id) {
                match element.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    Ok(select_element) => Some(select_element),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not a text area", id);
                        None
                    }
                }
            } else {
                web_error!("bluefire: element '{}' does not exist", id);
                None
            };
            Self { element }
        }

        /// Constructs a new `TextArea`.
        pub fn get_optional(id: &str) -> Self {
            let element = crate::web::document()
                .get_element_by_id(id)
                .map(|element| element.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                .flatten();
            Self { element }
        }

        /// Returns the text displayed inside the text area.
        pub fn get_value(&self) -> String {
            if let Some(ref element) = self.element {
                element.value()
            } else {
                String::default()
            }
        }

        /// Sets the text displayed inside the text area.
        pub fn set_value(&self, value: &str) {
            if let Some(ref element) = self.element {
                element.set_value(value);
            }
        }
    }
}

pub use self::element::Element;

#[cfg(feature = "elements_input")]
pub use self::input::Input;

#[cfg(feature = "elements_textarea")]
pub use self::textarea::TextArea;

/// Prelude for `elements` module.
pub mod prelude {
    pub use super::element::Element;

    #[cfg(feature = "elements_input")]
    pub use super::input::Input;

    #[cfg(feature = "elements_select")]
    pub use super::select::Select;

    #[cfg(feature = "elements_textarea")]
    pub use super::textarea::TextArea;

    pub use web_sys::{DragEvent, Event, KeyboardEvent, MouseEvent};
}
