// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `bluefire` macros.

#![warn(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

mod new_constant;

/// Generates a constructor initializing a structure composed only of `&'static str` fields using
/// the field names. Useful for defining serializable bundles of constants.
///
/// ## Example
///
/// In case of
/// ```
/// #[bluefire_macros::new_constant]
/// struct MyIds {
///     id_1: &'static str,
///     id_2: &'static str,
/// }
/// ```
/// the macro will generate
/// ``` ignore
/// impl MyIds {
///     pub const fn new_constant() -> Self {
///         Self {
///             id_1: "id_1",
///             id_2: "id_2",
///         }
///     }
/// }
/// ```
///
/// ## Attributes
///
/// `format` - determines whether names should be reformatted (assuming they are provided in snake
/// case):
///   - `snake` - leaves without change (`field_name`)
///   - `kebab` - reformats to kebab case (`field-name`)
#[proc_macro_attribute]
pub fn new_constant(
    attr: proc_macro::TokenStream,
    stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    new_constant::new_constant(attr, stream)
}
