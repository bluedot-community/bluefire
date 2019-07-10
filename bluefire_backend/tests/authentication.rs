// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_backend::authentication` module.

pub mod common;

use bluefire_backend::{authentication::*, *};

mod env {
    use super::*;
    pub use crate::common::data_providers::{FakeAuthenticationDataProvider, FakeDatabase};
    pub use crate::common::data_providers::{INVALID_SESSION_ID, VALID_SESSION_ID};
    use crate::common::handlers::TestHandler;

    pub struct Env {
        pub wielder: BlueFireWielder,
    }

    impl Env {
        pub fn new() -> Env {
            let host = router::Host::new_nameless();
            let route = router::Route::index().with_view(TestHandler::new("index"));
            let mut routing_builder = Box::new(router::RoutingBuilder::new());
            routing_builder.insert(host, route);

            let db = FakeDatabase::new();
            let middleware = AuthenticationMiddleware::<FakeAuthenticationDataProvider>::new();
            let kindler = BlueFireKindler::start(routing_builder).extend(db).wire(middleware);

            Env { wielder: kindler.kindle() }
        }

        pub fn get(&mut self, uri: &str, cookie: Option<&str>) -> Response {
            let mut builder = http::request::Builder::new();
            builder
                .method(http::method::Method::GET)
                .uri(uri.parse::<http::uri::Uri>().expect("Parse URI"));

            if let Some(cookie) = cookie {
                builder.header(http::header::COOKIE, cookie);
            }

            let request = builder.body("".into()).expect("Failed to build empty GET body");
            self.wielder.serve(&request)
        }
    }
}

#[test]
fn test_authentication_middleware_without_session() {
    let mut env = env::Env::new();
    env.get("/", None);

    let user_info = env
        .wielder
        .get_context()
        .extension::<UserInfo>()
        .expect("Valid UserInfo should be returned");
    assert!(user_info.get_user().is_none());
}

#[test]
fn test_authentication_middleware_with_invalid_session() {
    let mut env = env::Env::new();
    let session_id = "SESSION_ID=".to_string() + env::INVALID_SESSION_ID;
    env.get("/", Some(&session_id));

    let user_info = env
        .wielder
        .get_context()
        .extension::<UserInfo>()
        .expect("Valid UserInfo should be returned");
    assert!(user_info.get_user().is_none());
}

#[test]
fn test_authentication_middleware_with_valid_session() {
    let mut env = env::Env::new();
    let session_id = "SESSION_ID=".to_string() + env::VALID_SESSION_ID;
    env.get("/", Some(&session_id));

    let user_info = env
        .wielder
        .get_context()
        .extension::<UserInfo>()
        .expect("Valid UserInfo should be returned");
    assert!(user_info.get_user().is_some());
    let user = user_info.get_user().clone().expect("Valid user should be returned");
    assert_eq!(user.username(), "Alice");
    assert_eq!(user.email(), "alice@bluedot.community");
}
