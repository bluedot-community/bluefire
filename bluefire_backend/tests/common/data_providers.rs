// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of a fake database and data providers for test.

use std::error::Error;

use bluefire_twine::id::Id;

use bluefire_backend::authentication::prelude::*;
use bluefire_backend::database::{DataProvider, Database};
use bluefire_backend::Extension;

pub const INVALID_SESSION_ID: &str = "FFFFFFFFFFFFFFFFFFFFFFFF";
pub const VALID_SESSION_ID: &str = "0102030405060708090A0B0C";

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct FakeDatabase;

impl FakeDatabase {
    pub fn new() -> Self {
        Self {}
    }
}

impl Database for FakeDatabase {}

impl Extension for FakeDatabase {
    fn get_name(&self) -> &str {
        "BlueFire:FakeDatabase"
    }

    fn check(&self) -> Result<(), ()> {
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn Extension> {
        Box::new(self.clone())
    }

    fn destroy(&self) {
        // noting to do
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct FakeAuthenticationDataProvider {
    params: AuthenticationQueryParams,
    user: Option<Box<dyn UserTrait>>,
    session: Option<Box<dyn SessionTrait>>,
}

impl FakeAuthenticationDataProvider {
    fn new(
        params: AuthenticationQueryParams,
        user: Option<Box<dyn UserTrait>>,
        session: Option<Box<dyn SessionTrait>>,
    ) -> Self {
        FakeAuthenticationDataProvider { params, user, session }
    }
}

impl AuthenticationDataProvider for FakeAuthenticationDataProvider {
    fn get_user(&self) -> Option<&Box<dyn UserTrait>> {
        if let Some(ref user) = self.user {
            return Some(&user);
        } else {
            return None;
        }
    }

    fn get_session(&self) -> Option<&Box<dyn SessionTrait>> {
        if let Some(ref session) = self.session {
            return Some(&session);
        } else {
            return None;
        }
    }

    fn logout_user(&mut self, _db: &Self::Database) -> LogoutResult {
        Ok(LogoutOutcome::Success)
    }
}

impl DataProvider for FakeAuthenticationDataProvider {
    type Database = FakeDatabase;
    type QueryParams = AuthenticationQueryParams;

    fn create(_db: &Self::Database, params: &Self::QueryParams) -> Result<Self, Box<dyn Error>> {
        let (user, session): (Option<Box<dyn UserTrait>>, Option<Box<dyn SessionTrait>>) = {
            let valid_session_id = Id::from_str(VALID_SESSION_ID).expect("Session ID");
            if params.session_id == valid_session_id {
                let user_id = Id::new_random();
                let session_id = Id::new_random();
                let username = String::from("Alice");
                let email = String::from("alice@bluedot.community");
                let encoded_password = String::from("$$$$");
                let valid_to = chrono::Utc::now() + chrono::Duration::hours(1);
                let user = User::new(user_id, username, email, encoded_password, vec![], true);
                let session = Session::new(session_id, valid_to);
                (Some(Box::new(user)), Some(Box::new(session)))
            } else {
                (None, None)
            }
        };
        Ok(FakeAuthenticationDataProvider::new(params.clone(), user, session))
    }
}

impl Clone for FakeAuthenticationDataProvider {
    fn clone(&self) -> FakeAuthenticationDataProvider {
        FakeAuthenticationDataProvider {
            params: self.params.clone(),
            user: {
                if let Some(ref user) = self.user {
                    Some(user.duplicate())
                } else {
                    None
                }
            },
            session: {
                if let Some(ref session) = self.session {
                    Some(session.duplicate())
                } else {
                    None
                }
            },
        }
    }
}
