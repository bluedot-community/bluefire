// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Database related definitions.

use crate::context::Extension;

// -------------------------------------------------------------------------------------------------

/// Trait for database context extensions.
pub trait Database: Extension + std::fmt::Debug + Sized {}

/// `BlueFire` does not provide an ORM. This trait allows to abstract the database access if needed
/// in high-level components.
pub trait DataProvider: std::fmt::Debug + Sized {
    /// Database connection.
    type Database: Database;
    /// Query parameters.
    type QueryParams; // TODO: Rename to `Params`

    /// Constructs a new provider using `Database`.
    fn create(
        db: &Self::Database,
        params: &Self::QueryParams,
    ) -> Result<Self, Box<dyn std::error::Error>>;
}
