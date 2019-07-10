// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Clock functionality.

use crate::context::Extension;

// -------------------------------------------------------------------------------------------------

/// Trait for clock implementations. Different implementation of clock are needed in production and
/// testing.
pub trait Clock: Send {
    /// Returns current time.
    fn now(&self) -> chrono::DateTime<chrono::Utc>;

    /// Sleeps the thread execution for a given duration.
    fn sleep(&self, duration: chrono::Duration);

    /// Clones the clock.
    fn duplicate(&self) -> Box<dyn Clock>;
}

// -------------------------------------------------------------------------------------------------

/// Provides the UTC time.
pub struct UtcClock;

impl UtcClock {
    /// Constructs a new `UtcClock`.
    pub fn new() -> Self {
        Self
    }
}

impl Clock for UtcClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    fn sleep(&self, duration: chrono::Duration) {
        std::thread::sleep(duration.to_std().expect("Cast duration"));
    }

    fn duplicate(&self) -> Box<dyn Clock> {
        Box::new(Self)
    }
}

// -------------------------------------------------------------------------------------------------

/// BlueFire context extension providing clock functionality.
pub struct ClockExtension {
    clock: Box<dyn Clock>,
}

impl ClockExtension {
    /// Constructs a new `ClockExtension` with provided clock implementation.
    pub fn new(clock: Box<dyn Clock>) -> Self {
        Self { clock }
    }

    /// Constructs a new `ClockExtension` with UTC clock.
    pub fn new_utc() -> Self {
        Self { clock: Box::new(UtcClock::new()) }
    }

    /// Constructs a new `ClockExtension` with fake clock for testing.
    pub fn new_testing() -> Self {
        Self { clock: Box::new(testing::TestClock::new()) }
    }

    /// Returns the clock implementation.
    pub fn get_implementation(&self) -> &Box<dyn Clock> {
        &self.clock
    }
}

impl Clock for ClockExtension {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.clock.now()
    }

    fn sleep(&self, duration: chrono::Duration) {
        self.clock.sleep(duration)
    }

    fn duplicate(&self) -> Box<dyn Clock> {
        Box::new(ClockExtension::new(self.clock.duplicate()))
    }
}

impl Extension for ClockExtension {
    fn get_name(&self) -> &str {
        "BlueFire:Clock"
    }

    fn check(&self) -> Result<(), ()> {
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn Extension> {
        Box::new(ClockExtension::new(self.clock.duplicate()))
    }

    fn destroy(&self) {
        // noting to do
    }
}

impl std::fmt::Debug for ClockExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ClockExtension")
    }
}

// -------------------------------------------------------------------------------------------------

/// Provides an implementation of the `Clock` for testing purposes.
pub mod testing {
    use std::sync::{Arc, Mutex};

    /// A fake clock for testing.
    #[derive(Clone)]
    pub struct TestClock {
        datetime: Arc<Mutex<chrono::DateTime<chrono::Utc>>>,
    }

    impl TestClock {
        /// Constructs a new `TestClock`.
        pub fn new() -> Self {
            Self { datetime: Arc::new(Mutex::new(chrono::Utc::now())) }
        }

        /// Advances the clock by given duration.
        pub fn advance(&mut self, duration: chrono::Duration) {
            let mut datetime = self.datetime.lock().expect("Mutex lock");
            *datetime = *datetime + duration;
        }
    }

    impl super::Clock for TestClock {
        fn now(&self) -> chrono::DateTime<chrono::Utc> {
            let datetime = self.datetime.lock().expect("Mutex lock");
            datetime.clone()
        }

        fn sleep(&self, _duration: chrono::Duration) {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        fn duplicate(&self) -> Box<dyn super::Clock> {
            Box::new(self.clone())
        }
    }
}
