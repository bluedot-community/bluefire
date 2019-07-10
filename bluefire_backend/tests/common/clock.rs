// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Common definitions for tests using workers.

use std::sync::{Arc, Mutex};

use bluefire_backend::{scheduler::*, *};

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fingerprint {
    worker_id: u32,
    finger_id: u32,
}

impl Fingerprint {
    pub fn new(worker_id: u32, finger_id: u32) -> Self {
        Self { worker_id, finger_id }
    }
}

// -------------------------------------------------------------------------------------------------

pub struct State {
    is_running: bool,
    fingerprints: Vec<Fingerprint>,
}

impl State {
    pub fn new() -> Self {
        Self { is_running: true, fingerprints: Vec::new() }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn touch(&mut self, finger: Fingerprint) {
        self.fingerprints.push(finger);
    }

    pub fn get_fingerprints(&self) -> &Vec<Fingerprint> {
        &self.fingerprints
    }
}

impl GlobalState for State {
    fn is_running(&self) -> bool {
        self.is_running
    }
}

// -------------------------------------------------------------------------------------------------

pub struct FingerprintWorker {
    finger: Fingerprint,
    interval: chrono::Duration,
}

impl FingerprintWorker {
    pub fn new(worker_id: u32, interval: chrono::Duration) -> Self {
        Self { finger: Fingerprint { worker_id: worker_id, finger_id: 0 }, interval: interval }
    }
}

impl Worker<State> for FingerprintWorker {
    fn run(&mut self, state: &Arc<Mutex<State>>) -> Trigger {
        let mut state = state.lock().expect("Mutex lock");
        state.touch(self.finger.clone());
        self.finger.finger_id += 1;
        Trigger::In(self.interval.clone())
    }
}

// -------------------------------------------------------------------------------------------------

pub struct StopWorker;

impl StopWorker {
    pub fn new() -> Self {
        Self
    }
}

impl Worker<State> for StopWorker {
    fn run(&mut self, state: &Arc<Mutex<State>>) -> Trigger {
        let mut state = state.lock().expect("Mutex lock");
        state.stop();
        Trigger::In(chrono::Duration::hours(1))
    }
}
