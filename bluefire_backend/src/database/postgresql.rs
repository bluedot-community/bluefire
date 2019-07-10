// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Access to PostgreSQL databases.

use std::sync::{Arc, Mutex};

pub use postgres;

use super::{Database, DatabaseError};
use context::Extension;

#[derive(Clone, Debug)]
pub struct PostresqlDatabase {
    connection: Arc<Mutex<postgres::Connection>>,
}

impl PostresqlDatabase {
    pub fn new(params: &str) -> Result<PostresqlDatabase, DatabaseError> {
        let connection = postgres::Connection::connect(params, postgres::TlsMode::None)?;
        Ok(PostresqlDatabase {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl Database for PostresqlDatabase {
    type Connection = postgres::Connection;

    fn get_connection(&self) -> Arc<Mutex<Self::Connection>> {
        self.connection.clone()
    }
}

impl Extension for PostresqlDatabase {
    fn destroy(&self) {
        // noting to do
    }

    fn duplicate(&self) -> Box<Extension> {
        Box::new(self.clone())
    }
}

impl From<postgres::Error> for DatabaseError {
    fn from(error: postgres::Error) -> Self {
        DatabaseError::Other { description: error.to_string() }
    }
}

