// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_backend::router`.

pub mod common;

use bluefire_backend::{router::*, *};

mod env {
    use super::*;
    use crate::common::handlers::TestHandler;

    pub struct Env {
        pub wielder: BlueFireWielder,
    }

    impl Env {
        pub fn new() -> Env {
            let mut builder = RoutingBuilder::new();

            #[rustfmt::skip]
            builder.insert(
                Host::new_nameless(),
                Route::index()
                    .with_view(TestHandler::new("index"))
                    .with_routes(vec![
                        Route::exact("about")
                            .with_view(TestHandler::new("about"))
                            .with_label("label_about"),
                        Route::exact("projects")
                            .with_view(TestHandler::new("projects"))
                            .with_routes(vec![
                                Route::exact("project1")
                                    .with_view(TestHandler::new("project1"))
                                    .with_label("label_project1"),
                                Route::exact("project2")
                                    .with_view(TestHandler::new("project2"))
                                    .with_label("label_project2"),
                            ]),
                        Route::index()
                            .with_routes(vec![
                                Route::param("item_id")
                                    .with_view(TestHandler::new("item"))
                                    .with_label("label_item"),
                            ]).as_exact("items"),
                    ])
            );

            let wielder = BlueFireKindler::start(Box::new(builder)).kindle();
            Env { wielder }
        }

        pub fn exec(&mut self, uri: &str) -> Response {
            let request = http::request::Builder::new()
                .method(http::method::Method::GET)
                .uri(uri.parse::<http::uri::Uri>().expect("Parse URI"))
                .body("".into())
                .expect("Failed to build empty GET body");

            self.wielder.route(&request)
        }

        pub fn params(&self) -> &ParamsMap {
            &self.wielder.get_context().params()
        }
    }
}

#[test]
fn test_routing_for_index() {
    let mut env = env::Env::new();
    let response = env.exec("/");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(response.body(), "index");
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_first_level() {
    let mut env = env::Env::new();
    let response = env.exec("/about");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(response.body(), "about");
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_first_level_with_children() {
    let mut env = env::Env::new();
    let response = env.exec("/projects");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(response.body(), "projects");
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_second_level_first() {
    let mut env = env::Env::new();
    let response = env.exec("/projects/project1");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(response.body(), "project1");
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_second_level_last() {
    let mut env = env::Env::new();
    let response = env.exec("/projects/project2");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(response.body(), "project2");
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_node_without_handler() {
    let mut env = env::Env::new();
    let response = env.exec("/items");
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    assert!(env.params().is_empty());
}

#[test]
fn test_routing_for_node_with_param() {
    let mut env = env::Env::new();
    let response = env.exec("/items/12345");
    assert_eq!(response.status(), http::StatusCode::OK);
    assert_eq!(env.params().len(), 1);
    assert_eq!(env.params().get("item_id").expect("Item in params"), "12345");
}

#[test]
fn test_routing_for_not_existing_node() {
    let mut env = env::Env::new();
    let response = env.exec("/not_existing_node");
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    assert!(env.params().is_empty());
}

#[test]
#[should_panic]
fn test_setting_non_index_as_exact_should_assert() {
    Route::exact("abc").as_exact("efg");
}

#[test]
fn test_path_labels() {
    let env = env::Env::new();

    {
        let params = std::collections::HashMap::new();
        let path = env.wielder.get_context().reverse(&"label_about".to_string()).unwrap();
        assert_eq!(path.as_path(&params), "/about");
    }
    {
        let mut params = std::collections::HashMap::new();
        params.insert("item_id", "12345".to_string());
        let path = env.wielder.get_context().reverse(&"label_item".to_string()).unwrap();
        assert_eq!(path.as_path(&params), "/items/12345");
    }
}
