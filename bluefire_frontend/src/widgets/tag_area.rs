// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tag-area widget.

use maud::html;

use crate::{elements::prelude::*, web_error};

use super::CLASS_NAMES as C;

const FORMAT: &str = "application/x-ulangi-element-id";
const EFFECT: &str = "move";

// -------------------------------------------------------------------------------------------------

/// Conteins the information about a tag to be displayed.
pub struct Tag<'a> {
    id: &'a str,
    text: &'a str,
    click_callback: Option<Box<dyn Fn(MouseEvent)>>,
}

impl<'a> Tag<'a> {
    /// Constructs a new `Tag`.
    pub fn new(id: &'a str, text: &'a str) -> Self {
        Self { id, text, click_callback: None }
    }

    /// Sets a handler for `click` event.
    pub fn on_click(mut self, callback: Box<dyn Fn(MouseEvent)>) -> Self {
        self.click_callback = Some(callback);
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents a "tag area" widget.
pub struct TagArea {
    element: Element,
    reordarable: bool,
}

impl TagArea {
    /// Add a new tag.
    pub fn add_tag<'a>(&self, tag: Tag<'a>) {
        let draggable = if self.reordarable { "true" } else { "false" };
        let html = html! { li.(C.bd_tag)#(tag.id) draggable=(draggable) { (tag.text) } };
        self.element.insert_end(&html.into_string());

        let element = Element::get(&tag.id);

        if let Some(callback) = tag.click_callback {
            element.on_click(callback);
        }

        if self.reordarable {
            Self::bind_drag_and_drop_events(&element, tag.id);
        }
    }

    /// Returns IDs of all tags present in the tag area.
    pub fn get_tag_ids(&self) -> Vec<String> {
        self.element.get_children_ids()
    }

    /// Removes all tags.
    pub fn clean(&self) {
        self.element.set_text(None);
    }
}

impl TagArea {
    fn bind_drag_and_drop_events(element: &Element, id: &str) {
        let id2 = id.to_string();
        element.on_dragstart(Box::new(move |event| {
            if let Some(data_transfer) = event.data_transfer() {
                data_transfer.set_effect_allowed(EFFECT);
                let _ = data_transfer.set_data(FORMAT, &id2);
            // TODO: Hide the element
            } else {
                Self::warn_no_data_transfer();
            }
        }));

        element.on_dragend(Box::new(move |event| {
            if let Some(data_transfer) = event.data_transfer() {
                if data_transfer.drop_effect() != EFFECT {
                    // TODO: Unhide the element
                }
            } else {
                Self::warn_no_data_transfer();
            }
        }));

        element.on_dragover(Box::new(move |event| {
            if let Some(data_transfer) = event.data_transfer() {
                if let Ok(source_element_id) = data_transfer.get_data(FORMAT) {
                    let source_tag = Element::get(&source_element_id);
                    let target_tag = Element::from_event(&event);
                    if Element::are_the_same(&source_tag.parent(), &target_tag.parent()) {
                        data_transfer.set_drop_effect(EFFECT);
                        event.prevent_default();
                    }
                }
            } else {
                Self::warn_no_data_transfer();
            }
        }));

        element.on_drop(Box::new(move |event| {
            if let Some(data_transfer) = event.data_transfer() {
                if let Ok(source_element_id) = data_transfer.get_data(FORMAT) {
                    let source_tag = Element::get(&source_element_id);
                    let target_tag = Element::from_event(&event);

                    if Element::are_the_same(&source_tag.parent(), &target_tag.parent()) {
                        if let Some(rect) = target_tag.get_bounding_client_rect() {
                            data_transfer.set_drop_effect(EFFECT);
                            if (2.0 * event.offset_x() as f64) < rect.width() {
                                target_tag.place_before(&source_tag);
                            } else {
                                target_tag.place_after(&source_tag);
                            }
                        }
                    }
                }
            } else {
                Self::warn_no_data_transfer();
            }
        }));
    }

    fn warn_no_data_transfer() {
        web_error!("ulangi: data transfer missing");
    }
}

// -------------------------------------------------------------------------------------------------

/// Builder for the `TagArea` widget.
pub struct TagAreaBuilder {
    element: Element,
    filter: Option<Input>,
    reordarable: bool,
}

impl TagAreaBuilder {
    /// Constructs a new `TagAreaBuilder` to setup the tag area over the given element.
    pub fn over(element_id: &str) -> Self {
        Self { element: Element::get(element_id), filter: None, reordarable: false }
    }

    /// Sets the passed text input as a filter for tags.
    pub fn over_filter(mut self, element_id: &str) -> Self {
        self.filter = Some(Input::get(element_id));
        self
    }

    /// Tell is the tag area chould be reorderable using drag and drop.
    pub fn reordarable(mut self, reordarable: bool) -> Self {
        self.reordarable = reordarable;
        self
    }

    /// Builds a new `TagArea`
    pub fn build(self) -> TagArea {
        self.element.set_class(C.bd_tag_area);
        self.element.set_text(None);

        if let Some(filter) = self.filter {
            let area = self.element.clone();
            filter.on_keyup(Box::new(move |event| {
                let value = Input::from_event(&event).get_value();
                for child in area.get_children_elements() {
                    if let Some(text) = child.get_text() {
                        if text.contains(&value) {
                            child.unhide();
                        } else {
                            child.hide();
                        }
                    } else {
                        child.hide();
                    }
                }
            }));
        }

        TagArea { element: self.element, reordarable: self.reordarable }
    }
}
