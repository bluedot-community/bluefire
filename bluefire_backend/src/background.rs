// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Delegating side-tasks to be executed in the background thread.

// TODO: Reimplement using async/await when stabilized.

use std::sync::{
    mpsc::{sync_channel, Receiver, SyncSender},
    {Arc, Mutex},
};

use crate::context::{BlueFire, Extension};

// -------------------------------------------------------------------------------------------------

struct Task {
    worker: Mutex<Option<Box<dyn FnOnce() -> () + Send>>>,
}

// -------------------------------------------------------------------------------------------------

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    /// Constructs a new `Executor`.
    fn new(receiver: Receiver<Arc<Task>>) -> Self {
        Self { receiver }
    }

    /// Runs the executor. This operation is blocking the current thread.
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut worker = task.worker.lock().unwrap();
            if let Some(worker) = worker.take() {
                worker()
            }
        }
    }

    /// Runs the executor in a new thread.
    fn spawn(self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || self.run())
    }
}

// -------------------------------------------------------------------------------------------------

/// Manager of background jobs.
///
/// Implements `Extension`.
#[derive(Clone, Debug)]
pub struct Background {
    sender: SyncSender<Arc<Task>>,
}

impl Background {
    /// Constructs a new `Background`.
    pub fn new() -> Self {
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (sender, receiver) = sync_channel(MAX_QUEUED_TASKS);

        Executor::new(receiver).spawn();

        Background { sender }
    }

    /// Send the worker to be executed in the background thread.
    pub fn send(&self, worker: Box<dyn FnOnce() -> () + Send>) {
        let task = Arc::new(Task { worker: Mutex::new(Some(worker)) });
        self.sender.send(task).expect("too many tasks queued");
    }
}

impl Extension for Background {
    fn get_name(&self) -> &str {
        "BlueFire:Background"
    }

    fn check(&self) -> Result<(), ()> {
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn Extension> {
        Box::new(self.clone())
    }

    fn destroy(&self) {
        // nothing to do
    }
}

// -------------------------------------------------------------------------------------------------

impl BlueFire {
    /// Returns `Background` extension.
    pub fn get_background(&self) -> Option<&Background> {
        self.extension::<Background>()
    }

    /// Returns `Background` extension. Panic if not found.
    pub fn get_background_unchecked(&self) -> &Background {
        self.extension::<Background>().expect("Background extension")
    }
}
