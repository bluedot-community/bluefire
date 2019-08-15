// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_macros`.

#[bluefire_macros::new_constant]
#[derive(Debug, PartialEq, Eq)]
struct SnakeIds {
    id_1: &'static str,
    id_2: &'static str,
}

#[bluefire_macros::new_constant(format = "kebab")]
#[derive(Debug, PartialEq, Eq)]
struct KebabIds {
    id_1: &'static str,
    id_2: &'static str,
}

#[test]
fn snake_ids() {
    let ids1 = SnakeIds::new_constant();
    let ids2 = SnakeIds { id_1: "id_1", id_2: "id_2" };
    assert_eq!(ids1, ids2);
}

#[test]
fn kebab_ids() {
    let ids1 = KebabIds::new_constant();
    let ids2 = KebabIds { id_1: "id-1", id_2: "id-2" };
    assert_eq!(ids1, ids2);
}
