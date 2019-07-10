// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions of common types, logging macros and errors.

use std::collections::HashMap;

pub use log::Level;

use crate::context::BlueFire;

// -------------------------------------------------------------------------------------------------

/// Type of the request bodies.
pub type Body = String;

/// Type of the HTTP request.
pub type Request = http::Request<Body>;

/// Type of the HTTP responses.
pub type Response = http::Response<Body>;

/// A mapping from path parameter names to path parameter values.
pub type ParamsMap = HashMap<&'static str, String>;

/// A trait for request handlers.
pub trait Handler: std::fmt::Debug + Send + Sync {
    /// Handler the request.
    fn handle(&self, context: &BlueFire, request: &Request) -> Response;

    /// Clone the handler.
    fn duplicate(&self) -> Box<dyn Handler>;
}

/// A trait required to be implemented by a global state shared between all the worker threads.
pub trait GlobalState: Send + 'static {
    /// Tells if the application is running or terminating.
    ///
    /// Current use-cases:
    ///  - scheduler determines if the main loop should be stopped
    fn is_running(&self) -> bool;
}

// -------------------------------------------------------------------------------------------------

/// Prints a log with trace level.
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::log!(target: "bluefire", crate::common::Level::Trace, $($arg)*)
    }
}

/// Prints a log with debug level.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::log!(target: "bluefire", crate::common::Level::Debug, $($arg)*)
    }
}

/// Prints a log with info level.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::log!(target: "bluefire", crate::common::Level::Info, $($arg)*)
    }
}

/// Prints a log with warn level.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::log!(target: "bluefire", crate::common::Level::Warn, $($arg)*)
    }
}

/// Prints a log with error level.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::log!(target: "bluefire", crate::common::Level::Error, $($arg)*)
    }
}

// -------------------------------------------------------------------------------------------------

/// Errors returned from `bluefire` functions.
#[derive(Debug)]
pub enum BlueFireError {
    /// Database contained a password encoded using an unknown authentication algorithm.
    #[cfg(feature = "authentication")]
    UnknownAuthenticationAlgorithm {
        /// Name of the algorithm.
        algorithm_name: String,
    },
    /// The format of the password hash in database is not correct.
    #[cfg(feature = "authentication")]
    InvalidPasswordHash,

    /// Database returned a response containing unexpected number of entries.
    #[cfg(feature = "database")]
    UnexpectedResponseSize {
        /// Number of returned entries.
        size: usize,
    },
    /// Failed to build a request to a database.
    #[cfg(feature = "database")]
    DatabaseRequestEncode {
        /// Description of the error.
        description: String,
    },
    /// Failed to parse a request from a database.
    #[cfg(feature = "database")]
    DatabaseResponseDecode {
        /// Description of the error.
        description: String,
    },
    /// Database returned an unexpected response.
    #[cfg(feature = "database")]
    DatabaseQuery {
        /// Description of the error.
        description: String,
    },

    /// Other error.
    Other {
        /// Description of the error.
        description: String,
    },
}

impl BlueFireError {
    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "authentication")]
    pub fn unknown_authentication_algorithm(algorithm_name: String) -> Self {
        BlueFireError::UnknownAuthenticationAlgorithm { algorithm_name }
    }

    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "authentication")]
    pub fn invalid_password_hash() -> Self {
        BlueFireError::InvalidPasswordHash
    }

    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "database")]
    pub fn unexpected_response_size(size: usize) -> Self {
        BlueFireError::UnexpectedResponseSize { size }
    }

    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "database")]
    pub fn database_request_encode(description: String) -> Self {
        BlueFireError::DatabaseRequestEncode { description }
    }

    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "database")]
    pub fn database_response_decode(description: String) -> Self {
        BlueFireError::DatabaseResponseDecode { description }
    }

    /// Constructs a new `BlueFireError`.
    #[cfg(feature = "database")]
    pub fn database_query(description: String) -> Self {
        BlueFireError::DatabaseQuery { description }
    }

    /// Constructs a new `BlueFireError`.
    pub fn other(description: String) -> Self {
        BlueFireError::Other { description }
    }
}

impl std::error::Error for BlueFireError {}

impl std::fmt::Display for BlueFireError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            #[cfg(feature = "authentication")]
            BlueFireError::UnknownAuthenticationAlgorithm { algorithm_name } => {
                write!(f, "Unknown authentication algorith '{}'", algorithm_name)
            }
            #[cfg(feature = "authentication")]
            BlueFireError::InvalidPasswordHash {} => write!(f, "Invalid password hash"),
            #[cfg(feature = "database")]
            BlueFireError::UnexpectedResponseSize { size } => {
                write!(f, "Unexpected response size '{}'", size)
            }
            #[cfg(feature = "database")]
            BlueFireError::DatabaseRequestEncode { description } => {
                write!(f, "Failed to encode database request: {}", description)
            }
            #[cfg(feature = "database")]
            BlueFireError::DatabaseResponseDecode { description } => {
                write!(f, "Failed to decode database response: {}", description)
            }
            #[cfg(feature = "database")]
            BlueFireError::DatabaseQuery { description } => {
                write!(f, "Database query error: {}", description)
            }
            BlueFireError::Other { description } => write!(f, "{}", description),
        }
    }
}
