// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Communication with other servers.

/// Fetches a remote resource.
pub fn fetch(
    host: &str,
    message: &bluefire_twine::message::Message,
) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::new();

    let path = if message.query().is_empty() {
        String::from(host) + message.path()
    } else {
        String::from(host) + message.path() + "?" + message.query()
    };
    let body = if (message.method() != "GET") && (message.method() != "HEAD") {
        message.body().to_string()
    } else {
        "".to_string()
    };
    let method =
        reqwest::Method::from_bytes(message.method().as_bytes()).expect("Fetch: Create method");
    let url = reqwest::Url::parse(&path).expect("Fetch: Create URL");

    client.request(method, url).body(body).send()
}
