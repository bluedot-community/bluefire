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

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Capitalize the given string. The first letter will be capitalized and all the others lowered.
pub fn capitalize(word: &str) -> String {
    let mut result = String::with_capacity(word.len());
    result += &word[..1].to_uppercase();
    result += &word[1..].to_lowercase();
    result
}

/// Represents an identifier name that can be formatted as snake- or camel-case.
#[derive(Clone, Debug, PartialEq, Eq)]
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
        let mut buffer = Vec::with_capacity(self.parts.len());
        for part in self.parts.iter() {
            buffer.push(part.to_lowercase());
        }
        buffer.join("_")
    }

    /// Returns a camel-case representation of the identifier name.
    pub fn camel_case(&self) -> String {
        let mut buffer = Vec::with_capacity(self.parts.len());
        for part in self.parts.iter() {
            buffer.push(capitalize(&part));
        }
        buffer.join("")
    }

    /// Returns a kebab-case representation of the identifier name.
    pub fn kebab_case(&self) -> String {
        let mut buffer = Vec::with_capacity(self.parts.len());
        for part in self.parts.iter() {
            buffer.push(part.to_lowercase());
        }
        buffer.join("-")
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.kebab_case())
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Name, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Name::new(&s))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        let name = super::Name::new("one-two-three");
        assert_eq!(name.camel_case(), "OneTwoThree");
        assert_eq!(name.snake_case(), "one_two_three");
        assert_eq!(name.kebab_case(), "one-two-three");
    }
}
