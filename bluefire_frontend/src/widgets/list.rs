// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Elements widget.

use maud::html;

use crate::elements::prelude::*;

use super::CLASS_NAMES as C;

// -------------------------------------------------------------------------------------------------

/// Represents an action button.
pub struct Action<'a> {
    text: &'a str,
    click_callback: Option<Box<dyn Fn(MouseEvent)>>,
}

impl<'a> Action<'a> {
    /// Constants a new `Action`.
    pub fn new(text: &'a str) -> Self {
        Self { text, click_callback: None }
    }

    /// Sets a handler for `click` event.
    pub fn on_click(mut self, callback: Box<dyn Fn(MouseEvent)>) -> Self {
        self.click_callback = Some(callback);
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents a list of items.
pub struct List {
    element: Element,
}

impl List {
    /// Constructs a new `List` using the element with passed ID.
    pub fn over(element_id: &str) -> Self {
        Self::onto(Element::get(element_id))
    }

    /// Constructs a new `List` using the passed element.
    pub fn onto(element: Element) -> Self {
        element.set_class(C.bd_elements);
        Self { element }
    }

    /// Adds a new item to the list.
    pub fn add_item<'a>(&self, actions: Vec<Action<'a>>) -> Element {
        let item = Element::new("li", "", None);
        let content = Element::new("div", "", None);
        let buttons = Element::new("div", &C.bd_elements_buttons, None);

        for action in actions {
            let button = Element::new("a", "", Some(action.text));
            if let Some(callback) = action.click_callback {
                button.on_click(callback);
            }
            buttons.place_end(&button);
        }

        item.place_end(&buttons);
        item.place_end(&content);
        self.element.place_end(&item);

        content
    }

    /// Removes all the items, optionaly setting a text.
    pub fn clean(&self, text: Option<&str>) {
        if let Some(text) = text {
            self.element.set_html(&html! { p { (text) } }.into_string())
        } else {
            self.element.set_html("");
        }
    }
}
