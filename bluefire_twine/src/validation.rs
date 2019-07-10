// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Validation for user input.

/// A generic result of validation.
///
/// Provides a way to report more than one error if many problems were found.
pub struct ValidationResult<T> {
    errors: Vec<T>,
}

impl<T> ValidationResult<T> {
    /// Constructs a new result containing no errors.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Checks if the result contains any errors.
    pub fn has_errors(&self) -> bool {
        self.errors.len() != 0
    }

    /// Returns a list of errors.
    pub fn get_errors(&self) -> &Vec<T> {
        &self.errors
    }

    /// Consumes another validation result by appending its errors to oneself.
    pub fn join(&mut self, other: ValidationResult<T>) {
        self.errors.extend(other.errors);
    }

    /// Adds a new error.
    pub fn add(&mut self, error: T) {
        self.errors.push(error);
    }
}

/// Checks if the passed string is a valid e-mail.
pub fn validate_email(email: &String) -> bool {
    checkmail::validate_email(&email)
}

/// Prelude for `validation` module.
pub mod prelude {
    pub use super::{validate_email, ValidationResult};
}
