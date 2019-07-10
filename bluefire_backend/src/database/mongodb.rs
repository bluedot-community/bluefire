// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Access to mongodb.

// TODO: Provide better enum variants for casting into `BlueFireError`.

use std::sync::Arc;

use bson::{bson, doc};

use mongo_driver::client::{ClientPool, Uri};
use mongo_driver::collection::Collection;

use crate::common::BlueFireError;
use crate::context::Extension;
use crate::database::Database;

/// This struct provides access to Mongo databases by implementing `Database` and `Extension`
/// traits.
#[derive(Clone)]
pub struct MongoDatabase {
    uri: Uri,
    database_name: String,
    client_pool: Arc<ClientPool>,
}

impl MongoDatabase {
    /// Constructs a new `MongoDatabase`.
    pub fn new(uri: String, database: String) -> Result<MongoDatabase, BlueFireError> {
        let uri = Uri::new(uri).expect("Parse MongoDB URI");
        let pool = ClientPool::new(uri.clone(), None);
        Ok(MongoDatabase { uri: uri, database_name: database, client_pool: Arc::new(pool) })
    }

    /// Checks connection to the server.
    pub fn check_server_status(&self) -> Result<(), ()> {
        let client = self.client_pool.pop();
        match client.get_server_status(None) {
            Ok(..) => Ok(()),
            Err(err) => {
                log::error!("Check for MongoDB connection failed: {}", err);
                Err(())
            }
        }
    }

    /// Returns underlying `mongo_driver` database object.
    pub fn db<'a>(&'a self) -> mongo_driver::database::Database<'a> {
        let client = self.client_pool.pop();
        client.take_database(self.database_name.as_bytes())
    }

    /// Returns a view to the given collection.
    pub fn collection<'a>(&'a self, collection: &str) -> Collection<'a> {
        let client = self.client_pool.pop();
        let db = client.take_database(self.database_name.as_bytes());
        db.take_collection(collection)
    }

    /// Creates the given collection.
    pub fn create_collection(&self, collection: &str) -> Result<(), BlueFireError> {
        let client = self.client_pool.pop();
        let db = client.get_database(self.database_name.as_bytes());
        if !db.has_collection(collection)? {
            db.create_collection(collection, None)?;
        }
        Ok(())
    }

    /// Drops the given collection.
    pub fn drop_collection(&self, collection: &str) -> Result<(), BlueFireError> {
        let client = self.client_pool.pop();
        let db = client.get_database(self.database_name.as_bytes());
        if db.has_collection(collection)? {
            let mut collection = db.get_collection(collection);
            collection.drop()?;
        }
        Ok(())
    }

    /// Create indexes.
    pub fn create_indexes(
        &self,
        collection: &str,
        keys: Vec<(&str, bson::Document)>,
    ) -> Result<(), BlueFireError> {
        let mut indexes = Vec::new();
        for key in keys {
            indexes.push(bson::Bson::Document(doc! { "name": key.0, "key": key.1 }));
        }

        let command = doc! {
            "createIndexes": collection,
            "indexes": bson::Bson::Array(indexes),
        };

        let client = self.client_pool.pop();
        let db = client.get_database(self.database_name.as_bytes());
        db.command_simple(command, None)?;
        Ok(())
    }
}

impl Extension for MongoDatabase {
    fn get_name(&self) -> &str {
        "BlueFire:MongoDatabase"
    }

    fn check(&self) -> Result<(), ()> {
        self.check_server_status()
    }

    fn duplicate(&self) -> Box<dyn Extension> {
        Box::new(self.clone())
    }

    fn destroy(&self) {
        // nothing to do
    }
}

impl Database for MongoDatabase {}

impl std::fmt::Debug for MongoDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MongoDatabase")
    }
}

impl From<bson::EncoderError> for BlueFireError {
    fn from(error: bson::EncoderError) -> Self {
        BlueFireError::database_request_encode(format!("{:?}", error))
    }
}

impl From<bson::DecoderError> for BlueFireError {
    fn from(error: bson::DecoderError) -> Self {
        BlueFireError::database_response_decode(format!("{:?}", error))
    }
}

impl From<bson::oid::Error> for BlueFireError {
    fn from(error: bson::oid::Error) -> Self {
        BlueFireError::database_response_decode(format!("{:?}", error))
    }
}

impl From<mongo_driver::MongoError> for BlueFireError {
    fn from(error: mongo_driver::MongoError) -> Self {
        BlueFireError::database_query(format!("{:?}", error))
    }
}
