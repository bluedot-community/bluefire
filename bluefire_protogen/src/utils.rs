// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Formatting utilities.
//!
//! ## Format string format
//!
//! The strings passed to `Name::new`, `snake_case` and `camel_case` must contain only letters,
//! numbers or hyphens. These string will be used to create identifiers in generated code. The
//! format string will be split in places of hyphens and resulted vector of strings will be used to
//! generate snake- or came-cased identifier names.

/// Capitalize the given string. The first letter will be capitalized and all the others lowered.
pub fn capitalize(word: &str) -> String {
    let mut result = String::new();
    result += &word[..1].to_uppercase();
    result += &word[1..].to_lowercase();
    result
}

/// Returns a snake-cased version of the passed string.
pub fn snake_case(name: &str) -> String {
    Name::new(name).snake_case()
}

/// Returns a camel-cased version of the passed string.
pub fn camel_case(name: &str) -> String {
    Name::new(name).camel_case()
}

/// Represents an identifier name that can be formatted as snake- or camel-case.
pub struct Name {
    parts: Vec<String>,
}

impl Name {
    /// Constructs as new `Name` from as specification string.
    pub fn new(spec_name: &str) -> Self {
        if spec_name.contains(" ") {
            panic!("Name '{}' contains spaces", spec_name);
        }
        Self { parts: spec_name.split("-").map(|s| s.to_owned()).collect() }
    }

    /// Constructs as new `Name` from a vector of strings.
    pub fn from_parts(parts: Vec<&str>) -> Self {
        Self { parts: parts.iter().map(|s| s.to_string()).collect() }
    }

    /// Returns a snake-case representation of the identifier name.
    pub fn snake_case(&self) -> String {
        let mut result = String::new();
        for part in self.parts.iter() {
            result += &part.to_lowercase();
            result += "_";
        }
        result.pop();
        result
    }

    /// Returns a camel-case representation of the identifier name.
    pub fn camel_case(&self) -> String {
        let mut result = String::new();
        for part in self.parts.iter() {
            result += &capitalize(&part);
        }
        result
    }
}
