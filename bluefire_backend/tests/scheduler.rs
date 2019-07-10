// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `bluefire_backend::scheduler`.

pub mod common;

use std::sync::{Arc, Mutex};

use bluefire_backend::clock::testing::TestClock;
use bluefire_backend::scheduler::*;

use self::common::clock::*;

mod env {
    use super::*;

    pub struct Env {
        pub state: Arc<Mutex<State>>,
        pub clock: Box<TestClock>,
        scheduler: Option<Scheduler<State>>,
        thread_handle: Option<std::thread::JoinHandle<()>>,
    }

    impl Env {
        pub fn new() -> Env {
            let state = Arc::new(Mutex::new(State::new()));
            let clock = Box::new(TestClock::new());
            let mut scheduler = Scheduler::new(state.clone(), clock.clone());

            scheduler.add(Trigger::In(chrono::Duration::hours(1)), Box::new(StopWorker::new()));

            Env { state: state, clock: clock, scheduler: Some(scheduler), thread_handle: None }
        }

        pub fn start(&mut self) {
            let scheduler = self.scheduler.take().expect("Scheduler");
            scheduler.spawn();
        }

        pub fn stop(&mut self) {
            if let Some(thread_handle) = self.thread_handle.take() {
                self.clock.advance(chrono::Duration::hours(1));
                thread_handle.join().expect("Join thread");
            }
        }

        pub fn schedule(&mut self, trigger: Trigger, worker: Box<dyn Worker<State>>) {
            self.scheduler.as_mut().expect("Scheduler").add(trigger, worker);
        }

        pub fn assert_fingerprints(&mut self, expected_fingerprints: &Vec<Fingerprint>) {
            for _ in 1..100 {
                {
                    let state = self.state.lock().expect("Mutex lock");
                    if *state.get_fingerprints() == *expected_fingerprints {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }

            let fingerprints = {
                let state = self.state.lock().expect("Mutex lock");
                state.get_fingerprints().clone()
            };

            assert_eq!(fingerprints, *expected_fingerprints);
        }
    }

    impl Drop for Env {
        fn drop(&mut self) {
            self.stop();
        }
    }
}

#[test]
fn test_scheduling_stop_task() {
    let mut env = env::Env::new();
    env.start();
    env.stop();
}

#[test]
fn test_scheduling_one_task_short_jumps() {
    let mut fingerprints = Vec::new();
    let mut env = env::Env::new();
    env.schedule(
        Trigger::In(chrono::Duration::minutes(1)),
        Box::new(FingerprintWorker::new(1, chrono::Duration::minutes(1))),
    );
    env.start();
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(61));
    fingerprints.push(Fingerprint::new(1, 0));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(60));
    fingerprints.push(Fingerprint::new(1, 1));
    env.assert_fingerprints(&fingerprints);

    env.stop();
}

#[test]
fn test_scheduling_many_tasks_short_jumps() {
    let mut fingerprints = Vec::new();
    let mut env = env::Env::new();
    env.schedule(
        Trigger::In(chrono::Duration::minutes(1)),
        Box::new(FingerprintWorker::new(1, chrono::Duration::minutes(2))),
    );
    env.schedule(
        Trigger::In(chrono::Duration::minutes(2)),
        Box::new(FingerprintWorker::new(2, chrono::Duration::minutes(2))),
    );
    env.start();
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(61));
    fingerprints.push(Fingerprint::new(1, 0));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(60));
    fingerprints.push(Fingerprint::new(2, 0));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(60));
    fingerprints.push(Fingerprint::new(1, 1));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(60));
    fingerprints.push(Fingerprint::new(2, 1));
    env.assert_fingerprints(&fingerprints);

    env.stop();
}

#[test]
fn test_scheduling_many_tasks_long_jumps() {
    let mut fingerprints = Vec::new();
    let mut env = env::Env::new();
    env.schedule(
        Trigger::In(chrono::Duration::minutes(1)),
        Box::new(FingerprintWorker::new(1, chrono::Duration::minutes(2))),
    );
    env.schedule(
        Trigger::In(chrono::Duration::minutes(2)),
        Box::new(FingerprintWorker::new(2, chrono::Duration::minutes(2))),
    );
    env.start();
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(121));
    fingerprints.push(Fingerprint::new(1, 0));
    fingerprints.push(Fingerprint::new(2, 0));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(120));
    fingerprints.push(Fingerprint::new(2, 1));
    fingerprints.push(Fingerprint::new(1, 1));
    env.assert_fingerprints(&fingerprints);

    env.clock.advance(chrono::Duration::seconds(120));
    fingerprints.push(Fingerprint::new(1, 2));
    fingerprints.push(Fingerprint::new(2, 2));
    env.assert_fingerprints(&fingerprints);

    env.stop();
}
