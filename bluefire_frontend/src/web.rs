// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Various utilities.

use wasm_bindgen::JsCast;

// -------------------------------------------------------------------------------------------------

/// Prints a log to the console.
#[macro_export]
macro_rules! web_log {
    ($($arg:tt)*) => { web_sys::console::log_1(&format!($($arg)*).into()); }
}

/// Prints an error to the console.
#[macro_export]
macro_rules! web_error {
    ($($arg:tt)*) => { web_sys::console::error_1(&format!($($arg)*).into()); }
}

/// Prints a warning to the console.
#[macro_export]
macro_rules! web_warn {
    ($($arg:tt)*) => { web_sys::console::warn_1(&format!($($arg)*).into()); }
}

/// Prints an info to the console.
#[macro_export]
macro_rules! web_info {
    ($($arg:tt)*) => { web_sys::console::info_1(&format!($($arg)*).into()); }
}

/// Prints a debug to the console.
#[macro_export]
macro_rules! web_debug {
    ($($arg:tt)*) => { web_sys::console::debug_1(&format!($($arg)*).into()); }
}

// -------------------------------------------------------------------------------------------------

/// Returns the browser window object. Panics if the object does not exist.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("bluefire: web_sys::window()")
}

/// Returns the HTML document object. Panics if the object does not exist.
pub fn document() -> web_sys::Document {
    window().document().expect("bluefire: web_sys::window().document()")
}

/// Returns the browsing history object. Panics if the object does not exist.
pub fn history() -> web_sys::History {
    window().history().expect("bluefire: web_sys::window().history()")
}

/// Returns an HTML document with given ID if exists.
pub fn get_element(id: &str) -> Option<web_sys::Element> {
    let element = document().get_element_by_id(&id);
    if element.is_none() {
        web_warn!("bluefire: element '{}' does not exist", id);
    }
    element
}

/// Return a browser window size.
pub fn get_window_size() -> (f64, f64) {
    let window = web_sys::window().expect("bluefire: web_sys::window()");
    let width = window
        .inner_width()
        .expect("bluefire: get window inner width")
        .as_f64()
        .expect("bluefire: width should be a number");
    let height = window
        .inner_height()
        .expect("bluefire: get window inner height")
        .as_f64()
        .expect("bluefire: height should be a number");
    (width, height)
}

/// Sets a callback to be executed when the size of the browser window changes.
pub fn on_window_resize(callback: Box<dyn Fn()>) {
    let window = web_sys::window().expect("bluefire: web_sys::window()");
    let closure = wasm_bindgen::closure::Closure::wrap(callback);
    let result =
        window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    if let Err(err) = result {
        web_error!("bluefire: failed to add event listener: {:?}", err);
    }
    closure.forget();
}

/// Refreshes the page.
pub fn reload_location() {
    let doc = document();
    if let Some(location) = doc.location() {
        match location.reload() {
            Ok(..) => {}
            Err(err) => {
                web_error!("bluefire: failed to reload the location: {:?}", err);
            }
        }
    }
}

/// Changes the location to the given one.
pub fn goto(domain: &str, path: &str) {
    let target = domain.to_string() + path;
    let doc = document();
    if let Some(location) = doc.location() {
        match location.assign(&target) {
            Ok(..) => {}
            Err(err) => {
                web_error!("bluefire: failed to assign the location: {:?}", err);
            }
        }
    }
}

/// Goes one step back in the browsing history.
pub fn go_back() {
    match history().back() {
        Ok(..) => {}
        Err(..) => web_warn!("bluefire: failed to go back in history"),
    }
}
