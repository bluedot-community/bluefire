// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Access to mongodb.

// TODO: Provide better enum variants for casting into `DatabaseError`.

use std::sync::{Arc, Mutex};

use mongodb::ThreadedClient;

use super::{Database, DatabaseError};
use crate::context::Extension;

/// This struct provides access to Mongo databases by implementing `Database` and `Extension`
/// traits.
#[derive(Clone)]
pub struct MongoDatabase {
    connection: Arc<Mutex<mongodb::db::Database>>,
}

impl MongoDatabase {
    /// Constructs a new `MongoDatabase`.
    pub fn new(host: &str, port: u16, database: &str) -> Result<MongoDatabase, DatabaseError> {
        let connection = mongodb::Client::connect(host, port)?.db(database);
        Ok(MongoDatabase {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl Database for MongoDatabase {
    type Connection = mongodb::db::Database;

    fn get_connection(&self) -> Arc<Mutex<Self::Connection>> {
        self.connection.clone()
    }
}

impl Extension for MongoDatabase {
    fn destroy(&self) {
        // nothing to do
    }

    fn duplicate(&self) -> Box<Extension> {
        Box::new(self.clone())
    }
}

impl std::fmt::Debug for MongoDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MongoDatabase")
    }
}

impl From<mongodb::Error> for DatabaseError {
    fn from(error: mongodb::Error) -> Self {
        DatabaseError::other(error.to_string())
    }
}

