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

/// This module contains functionality related to a generic HTML element.
pub mod element {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Represents a view into a generic HTML element.
    pub struct Element<'a> {
        id: &'a str,
    }

    impl<'a> Element<'a> {
        /// Constructs a new `Element`.
        pub fn get(id: &'a str) -> Self {
            Self { id }
        }

        /// Checks if the HTML element exists.
        pub fn exists(&self) -> bool {
            crate::web::document().get_element_by_id(&self.id).is_some()
        }

        /// Sets the text content of an element. The text will not be interpreted as HTML.
        pub fn set_text(&self, text: Option<&str>) {
            if let Some(element) = crate::web::get_element(&self.id) {
                element.set_text_content(text);
            }
        }

        /// Sets the inner-HTML of an element. The text will be interpreted as HTML.
        pub fn set_html(&self, html: &str) {
            if let Some(element) = crate::web::get_element(&self.id) {
                element.set_inner_html(html);
            }
        }

        /// Sets focus on the element.
        pub fn focus(&self) {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(html_element) => {
                        let _ = html_element.focus();
                    }
                    Err(..) => web_warn!("bluefire: '{}' is not an HTML element", self.id),
                }
            }
        }

        /// Appends given HTML at the end of the element.
        pub fn append(&self, html: &str) {
            if let Some(element) = crate::web::get_element(&self.id) {
                if let Err(err) = element.insert_adjacent_html("beforeend", html) {
                    web_error!("bluefire: append element: {:?}", err);
                }
            }
        }

        /// Removes the element.
        pub fn remove(&self) {
            if let Some(element) = crate::web::get_element(&self.id) {
                element.remove();
            }
        }

        /// Sets a callback to be executed when the element is clicked.
        pub fn on_click(&self, callback: Box<dyn Fn()>) {
            if let Some(element) = crate::web::get_element(&self.id) {
                let closure = Closure::wrap(callback);
                let result = element
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                if let Err(err) = result {
                    web_error!("bluefire: failed to add event listener: {:?}", err);
                }
                closure.forget();
            }
        }

        /// Return a list of style classes if any.
        pub fn get_class_list(&self) -> Option<web_sys::DomTokenList> {
            if let Some(element) = crate::web::get_element(&self.id) {
                Some(element.class_list())
            } else {
                None
            }
        }
    }
}

/// This module contains functionality related to HTML `input` elements.
#[cfg(feature = "elements_input")]
pub mod input {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Represents a view into an HTML `input` element.
    pub struct Input<'a> {
        id: &'a str,
    }

    impl<'a> Input<'a> {
        /// Constructs a new `Input`.
        pub fn get(id: &'a str) -> Self {
            Self { id }
        }

        /// Checks if the HTML element exists and is an `input` element.
        pub fn exists(&self) -> bool {
            if let Some(element) = crate::web::document().get_element_by_id(&self.id) {
                element.dyn_into::<web_sys::HtmlInputElement>().is_ok()
            } else {
                false
            }
        }

        /// Returns the value of the input.
        pub fn get_value(&self) -> String {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => input_element.value(),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", self.id);
                        String::default()
                    }
                }
            } else {
                String::default()
            }
        }

        /// Returns the value of the input and clears it.
        pub fn take_value(&self) -> String {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => {
                        let value = input_element.value();
                        input_element.set_value("");
                        value
                    }
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", self.id);
                        String::default()
                    }
                }
            } else {
                String::default()
            }
        }

        /// Sets the value of the input.
        pub fn set_value(&self, value: &str) {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => {
                        input_element.set_value(value);
                    }
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", self.id);
                    }
                }
            }
        }

        /// Checks if the radio- or check-box input is checked.
        pub fn is_checked(&self) -> bool {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => input_element.checked(),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", self.id);
                        false
                    }
                }
            } else {
                false
            }
        }

        /// Sets the checked-state of a radio- or check-box input.
        pub fn set_checked(&self, checked: bool) {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(input_element) => {
                        input_element.set_checked(checked);
                    }
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an input", self.id);
                    }
                }
            }
        }

        /// Sets a callback to be executed when the value of the input changes.
        pub fn on_change(&self, callback: Box<dyn Fn()>) {
            if let Some(element) = crate::web::get_element(&self.id) {
                let closure = Closure::wrap(callback);
                let result = element
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                if let Err(err) = result {
                    web_error!("bluefire: failed to add event listener: {:?}", err);
                }
                closure.forget();
            }
        }

        /// Sets a callback to be executed when the value of the input changes.
        pub fn on_keyup(&self, callback: Box<dyn Fn()>) {
            if let Some(element) = crate::web::get_element(&self.id) {
                let closure = Closure::wrap(callback);
                let result = element
                    .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
                if let Err(err) = result {
                    web_error!("bluefire: failed to add event listener: {:?}", err);
                }
                closure.forget();
            }
        }
    }
}

/// This module contains functionality related to HTML `select` elements.
#[cfg(feature = "elements_select")]
pub mod select {
    use wasm_bindgen::JsCast;

    /// Represents a view into an HTML `select` element.
    pub struct Select<'a> {
        id: &'a str,
    }

    impl<'a> Select<'a> {
        /// Constructs a new `Select`.
        pub fn get(id: &'a str) -> Self {
            Self { id }
        }

        /// Checks if the HTML element exists and is a `select` element.
        pub fn exists(&self) -> bool {
            if let Some(element) = crate::web::document().get_element_by_id(&self.id) {
                element.dyn_into::<web_sys::HtmlSelectElement>().is_ok()
            } else {
                false
            }
        }

        /// Returns the value of the selected element.
        pub fn get_value(&self) -> String {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlSelectElement>() {
                    Ok(input_element) => input_element.value(),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not an select element", self.id);
                        String::default()
                    }
                }
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
    pub struct TextArea<'a> {
        id: &'a str,
    }

    impl<'a> TextArea<'a> {
        /// Constructs a new `TextArea`.
        pub fn get(id: &'a str) -> Self {
            Self { id }
        }

        /// Returns the text displayed inside the text area.
        pub fn get_value(&self) -> String {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    Ok(textarea_element) => textarea_element.value(),
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not a textarea", self.id);
                        String::default()
                    }
                }
            } else {
                String::default()
            }
        }

        /// Sets the text displayed inside the text area.
        pub fn set_value(&self, value: &str) {
            if let Some(element) = crate::web::get_element(&self.id) {
                match element.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    Ok(textarea_element) => {
                        textarea_element.set_value(value);
                    }
                    Err(..) => {
                        web_warn!("bluefire: '{}' is not a textarea", self.id);
                    }
                }
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
}
