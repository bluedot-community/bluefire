// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Scheduling tasks to be executed at periodically at fixed times or after timeouts.

use std::sync::{Arc, Mutex};

use crate::clock::Clock;
use crate::common::GlobalState;

/// Represents a trigger to execute an action.
#[derive(Debug)]
pub enum Trigger {
    /// The action will be triggers after the given time passes.
    In(chrono::Duration),

    /// The action will be triggered at the given point in time.
    At(chrono::DateTime<chrono::Utc>),
}

impl Trigger {
    fn to_datetime(&self, clock: &Box<dyn Clock>) -> chrono::DateTime<chrono::Utc> {
        match self {
            Trigger::In(duration) => clock.now() + *duration,
            Trigger::At(datetime) => *datetime,
        }
    }
}

/// A worker that performs a scheduled action.
pub trait Worker<T>: Send
where
    T: GlobalState,
{
    /// Executes the workers action. Returns a condition to retrigger that action.
    fn run(&mut self, state: &Arc<Mutex<T>>) -> Trigger;
}

/// Defines an action to be executed and an event that triggers it.
struct Task<T>
where
    T: GlobalState,
{
    trigger: chrono::DateTime<chrono::Utc>,
    worker: Box<dyn Worker<T>>,
}

/// Manager for scheduling actions.
pub struct Scheduler<T>
where
    T: GlobalState,
{
    state: Arc<Mutex<T>>,
    schedule: Vec<Task<T>>,
    clock: Box<dyn Clock>,
}

impl<T> Scheduler<T>
where
    T: GlobalState,
{
    /// Constructs a new `Scheduler`.
    pub fn new(state: Arc<Mutex<T>>, clock: Box<dyn Clock>) -> Scheduler<T> {
        Scheduler { state: state, schedule: Vec::new(), clock: clock }
    }

    /// Adds a new worker with its initial trigger.
    pub fn add(&mut self, trigger: Trigger, worker: Box<dyn Worker<T>>) {
        self.schedule.push(Task { trigger: trigger.to_datetime(&self.clock), worker: worker });
    }

    /// Adds a new worker with its initial trigger.
    /// Same as `add` but with builder semantics.
    pub fn with(mut self, trigger: Trigger, worker: Box<dyn Worker<T>>) -> Self {
        self.add(trigger, worker);
        self
    }

    /// Runs the scheduler. This call blocks until the scheduler is stopped.
    pub fn run(&mut self) {
        let max_duration = chrono::Duration::seconds(10);
        self.sort();

        while self.should_run() {
            let task = self.schedule.first_mut().expect("No scheduler tasks");
            let now = self.clock.now();
            if now < task.trigger {
                let duration = task.trigger - now;
                let sleep_duration = if duration < max_duration { duration } else { max_duration };
                self.clock.sleep(sleep_duration);
            } else {
                let trigger = task.worker.run(&self.state);
                task.trigger = trigger.to_datetime(&self.clock);
                self.sort();
            }
        }
    }

    /// Runs the scheduler in a new thread.
    pub fn spawn(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || self.run())
    }

    fn sort(&mut self) {
        self.schedule.sort_unstable_by_key(|k| k.trigger.timestamp());
    }

    fn should_run(&self) -> bool {
        let state = self.state.lock().expect("Lock mutex");
        state.is_running()
    }
}
