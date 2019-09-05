// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_protogen`.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Param {
    number: u32,
    string: Vec<String>,
}

/// `serde_urlencoded` does not support serializing vectors. This test will start failing if that
/// changes.
#[test]
#[should_panic(expected = "unsupported value")]
fn serde_urlencoded_serialize() {
    let param = Param { number: 3, string: vec!["string1".to_string(), "string2".to_string()] };
    let expected = "number=3&string=string1&string=string2";
    let serialized = serde_urlencoded::to_string(&param).unwrap();
    assert_eq!(serialized, expected);
}

/// `serde_urlencoded` does not support deserializing vectors. This test will start failing if that
/// changes.
#[test]
#[should_panic(expected = "expected a sequence")]
fn serde_urlencoded_deserialize() {
    let string = "number=3&string=string1&string=string2";
    let expected = Param { number: 3, string: vec!["string1".to_string(), "string2".to_string()] };
    let deserialized: Param = serde_urlencoded::from_str(&string).unwrap();
    assert_eq!(deserialized, expected);
}
