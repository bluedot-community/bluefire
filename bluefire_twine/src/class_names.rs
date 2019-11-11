// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Provides names of all CSS classes provided by `bluefire_static_files` crate.

use serde_derive::{Deserialize, Serialize};

/// Names of all CSS classes provided by `bluefire_static_files` crate.
#[bluefire_macros::new_constant(format = "kebab")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassNames {
    /// Attention text class.
    pub bd_attention_text: &'static str,
    /// Regular-box class.
    pub bd_box: &'static str,
    /// Button class.
    pub bd_button: &'static str,
    /// Class for a tab cards.
    pub bd_cards: &'static str,
    /// Class for an active tab card.
    pub bd_card_active: &'static str,
    /// Class for an inactive tab card.
    pub bd_card_inactive: &'static str,
    /// Centring class.
    pub bd_center: &'static str,
    /// Class for close button with "X" symbol.
    pub bd_close_button: &'static str,
    /// Columns class.
    pub bd_columns: &'static str,
    /// Class for pop-up communicates.
    pub bd_communicates: &'static str,
    /// Class for action elements on lists, etc.
    pub bd_elements: &'static str,
    /// Class for button action elements.
    pub bd_elements_buttons: &'static str,
    /// Error pop-up class.
    pub bd_error: &'static str,
    /// Expanding layout class.
    pub bd_expand: &'static str,
    /// Class for text inputs.
    pub bd_field: &'static str,
    /// Makes an element float right.
    pub bd_float_right: &'static str,
    /// Class for main element taking whole width.
    pub bd_full_main: &'static str,
    /// Class for help question mark.
    pub bd_help: &'static str,
    /// Class for text for help question mark positioned centrally.
    pub bd_help_center: &'static str,
    /// Class for text for help question mark positioned to the left.
    pub bd_help_left: &'static str,
    /// Class for text for help question mark positioned to the right.
    pub bd_help_right: &'static str,
    /// Class for hidden elements.mod const
    pub bd_hidden: &'static str,
    /// Horizontal layout justified to the center.
    pub bd_horizontal_center: &'static str,
    /// Horizontal layout justified left.
    pub bd_horizontal_left: &'static str,
    /// Horizontal layout justified right.
    pub bd_horizontal_right: &'static str,
    /// Class for a big loader/spinner.
    pub bd_loader_big: &'static str,
    /// Class for a small loader/spinner.
    pub bd_loader_small: &'static str,
    /// Class for main element with maximal width.
    pub bd_main: &'static str,
    /// Major text class.
    pub bd_major_text: &'static str,
    /// Class for maps.
    pub bd_map: &'static str,
    /// Navigation bar class.
    pub bd_navbar: &'static str,
    /// Note text class.
    pub bd_note_text: &'static str,
    /// Class for semi-transparent overlays.
    pub bd_overlay: &'static str,
    /// Class for almost opaque overlays for pop-ups.
    pub bd_overlay_fixed: &'static str,
    /// Class for almost opaque overlays.
    pub bd_overlay_strong: &'static str,
    /// Rows class
    pub bd_rows: &'static str,
    /// Stretch-box class.
    pub bd_sbox: &'static str,
    /// Settings class.
    pub bd_settings: &'static str,
    /// Class for tabs.
    pub bd_tabs: &'static str,
    /// Class for an active tab.
    pub bd_tab_active: &'static str,
    /// Class for an inactive tab.
    pub bd_tab_inactive: &'static str,
    /// Tag class.
    pub bd_tag: &'static str,
    /// Tag area class.
    pub bd_tag_area: &'static str,
    /// Transparent box class.
    pub bd_tbox: &'static str,
    /// Class for a big text area.
    pub bd_textarea_big: &'static str,
    /// Class for a small text area.
    pub bd_textarea_small: &'static str,
    /// Title class.
    pub bd_title: &'static str,
    /// Vertical layout stretched left and right.
    pub bd_vertical: &'static str,
    /// Vertical layout justified left.
    pub bd_vertical_left: &'static str,
    /// Vertical layout justified right.
    pub bd_vertical_right: &'static str,
    /// Warning pop-up class.
    pub bd_warning: &'static str,
}
