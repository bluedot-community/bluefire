// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Pop-up communicates widget.

use maud::html;

use super::CLASS_NAMES as C;

use crate::elements::prelude::*;

static mut NEXT_ID: u32 = 0;

const COMMUNICATES_ID: &str = "communicates";

/// Visual type of the communicate.
pub enum CommunicateType {
    /// Error communicate.
    Error,

    /// Warning communicate.
    Warning,
}

impl CommunicateType {
    fn as_class_name(&self) -> &'static str {
        match self {
            CommunicateType::Error => C.bd_error,
            CommunicateType::Warning => C.bd_warning,
        }
    }
}

/// Represents the stack of communicates to the user.
pub struct Communicates;

impl Communicates {
    /// Adds a new communicate.
    pub fn push(text: &str, communicate_type: CommunicateType) {
        let id = unsafe {
            NEXT_ID += 1;
            NEXT_ID.to_string()
        };
        let bubble_id = String::from("bubble-") + &id;
        let close_button_id = String::from("close-bubble-button-") + &id;
        let class_name = communicate_type.as_class_name();
        Element::get(COMMUNICATES_ID).insert_end(
            &html! {
                div class=(class_name) id=(bubble_id) {
                    span.(C.bd_close_button) id=(close_button_id) {}
                    (text)
                }
            }
            .into_string(),
        );
        Element::get(&close_button_id).on_click(Box::new(move |_event| {
            Element::get(&bubble_id).remove();
        }));
    }
}
