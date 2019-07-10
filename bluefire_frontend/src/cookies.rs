// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Cookie-related utilities.
//!
//! See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies

use wasm_bindgen::JsCast;

use crate::web;

/// Represents a lifetime of a cookie.
pub enum Lifetime {
    /// The lifetime is defined as number of seconds from creation time.
    MaxAgeSeconds(u32),

    /// The lifetime is defined as a fixed point in UTC.
    Expires(chrono::DateTime<chrono::Utc>),
}

impl Lifetime {
    fn to_string(&self) -> String {
        match self {
            Lifetime::MaxAgeSeconds(seconds) => String::from("max-age=") + &seconds.to_string(),
            Lifetime::Expires(datetime) => String::from("expires=") + &datetime.to_rfc2822(),
        }
    }
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::MaxAgeSeconds(3600)
    }
}

/// Represents a cookie.
pub struct Cookie {
    key: String,
    value: String,
    path: String,
    lifetime: Lifetime,
    domain: Option<String>,
    is_secure: bool,
}

impl Cookie {
    /// Constructs a new `Cookie`.
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key,
            value: value,
            path: String::from("/"),
            domain: None,
            lifetime: Lifetime::default(),
            is_secure: false,
        }
    }

    /// Sets the path of the cookie.
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Sets the lifetime of the cookie.
    pub fn with_lifetime(mut self, lifetime: Lifetime) -> Self {
        self.lifetime = lifetime;
        self
    }

    /// Sets the domain of the cookie.
    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Decides if the cookie should be secure. (Secure cookies are only sent over HTTPS protocol.)
    ///
    /// By default the `Cookie` is not secure.
    pub fn with_security(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    /// Builds the body of the cookie.
    pub fn to_string(&self) -> String {
        let mut string = self.key.clone() + "=" + &self.value + "; " + &self.lifetime.to_string();
        if let Some(ref domain) = self.domain {
            string += &("; domain=".to_string() + &domain);
        }
        if !self.path.is_empty() {
            string += &("; path=".to_string() + &self.path);
        }
        if self.is_secure {
            string += "; secure";
        }
        string
    }

    /// Creates the cookie.
    pub fn set(&self) {
        if let Some(document) = web::document().dyn_ref::<web_sys::HtmlDocument>() {
            match document.set_cookie(&self.to_string()) {
                Ok(..) => {}
                Err(err) => {
                    web_warn!("bluefire: failed to set a cookie: {:?}", err);
                }
            }
        } else {
            web_debug!("bluefire: cannot set a cookie (this document is not an HTML document)");
        }
    }
}

/// Returns contents of a cookie defining the passed key.
pub fn get_cookie(searched_key: &str) -> Option<String> {
    if let Some(document) = web::document().dyn_ref::<web_sys::HtmlDocument>() {
        match &document.cookie() {
            Ok(all_cookies) => parse_cookie_out(all_cookies, searched_key),
            Err(..) => None,
        }
    } else {
        None
    }
}

fn parse_cookie_out(all_cookies: &str, searched_key: &str) -> Option<String> {
    for key_value in all_cookies.split(";") {
        let mut iter = key_value.splitn(2, "=");
        let key = iter.next();
        let value = iter.next();
        if key.is_none() || value.is_none() {
            break;
        }
        let key = key.unwrap().trim();
        if key == searched_key {
            return Some(value.unwrap().trim().to_string());
        }
    }
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_cookies() {
        let value = super::parse_cookie_out("abc = def ; ghi = jkl ; mno = prs", "ghi").unwrap();
        assert_eq!(value, "jkl");
    }
}
