// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Functionality related to e-mail sending.

use std::cell::RefCell;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::context::Extension;

use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre_email::{EmailBuilder, Mailbox};

// -------------------------------------------------------------------------------------------------

/// Represents an e-mail.
#[derive(Clone, Debug)]
pub struct EMail {
    sender: String,
    from_address: String,
    to_addresses: Vec<String>,
    subject: String,
    body: String,
}

impl EMail {
    /// Constructs a new `EMail`.
    pub fn new(
        sender: String,
        from_address: String,
        to_addresses: Vec<String>,
        subject: String,
        body: String,
    ) -> Self {
        Self { sender, from_address, to_addresses, subject, body }
    }

    /// Returns the sender.
    pub fn get_sender(&self) -> &String {
        &self.sender
    }

    /// Returns the "from address".
    pub fn get_from_address(&self) -> &String {
        &self.from_address
    }

    /// Returns the "to address".
    pub fn get_to_addresses(&self) -> &Vec<String> {
        &self.to_addresses
    }

    /// Returns the subject.
    pub fn get_subject(&self) -> &String {
        &self.subject
    }

    /// Returns the e-mails body.
    pub fn get_body(&self) -> &String {
        &self.body
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for mailing managers.
pub trait Mailer: Debug + Send {
    /// Sends an e-mail.
    fn send(&mut self, email: EMail) -> Result<(), ()>;
}

// -------------------------------------------------------------------------------------------------

/// A fake mailer. Does not send e-mails, just logs them. Useful for debugging and testing.
#[derive(Clone, Debug)]
pub struct FakeMailer;

impl FakeMailer {
    /// Constructs a new `FakeMailer`.
    pub fn new() -> Self {
        Self
    }
}

impl Mailer for FakeMailer {
    fn send(&mut self, email: EMail) -> Result<(), ()> {
        log::info!(
            "Faking sending an email '{}' to {:?}",
            email.get_subject(),
            email.get_to_addresses()
        );
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// A mailer for sending e-mails over SMTP.
pub struct SmtpMailer {
    transport: SmtpTransport,
}

impl SmtpMailer {
    /// Constructs a new `SmtpMailer`.
    pub fn new() -> Self {
        let transport = SmtpClient::new_unencrypted_localhost()
            .expect("BlueFire: Construct SMTP client")
            .transport();
        Self { transport }
    }
}

impl Mailer for SmtpMailer {
    fn send(&mut self, email: EMail) -> Result<(), ()> {
        log::info!(
            "Sending an email '{}' from '{}' to {:?}",
            email.get_subject(),
            email.get_from_address(),
            email.get_to_addresses()
        );
        let mut builder = EmailBuilder::new()
            .from(Mailbox::new_with_name(email.sender, email.from_address))
            .subject(email.subject)
            .html(email.body);
        for recipient in email.to_addresses {
            builder = builder.to(recipient);
        }

        match self.transport.send(builder.build().expect("Build an email").into()) {
            Ok(..) => Ok(()),
            Err(err) => {
                log::error!("Failed to send an email: {}", err);
                Err(())
            }
        }
    }
}

impl std::fmt::Debug for SmtpMailer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SmtpMailer")
    }
}

// -------------------------------------------------------------------------------------------------

/// A mailing state. Useful for tests.
#[derive(Clone, Debug)]
pub struct MailingState {
    number_of_sent_emails: u32,
    last_email: Option<EMail>,
}

impl MailingState {
    /// Notification about sent email.
    fn sent(&mut self, email: EMail) {
        self.number_of_sent_emails += 1;
        self.last_email = Some(email);
    }
}

// -------------------------------------------------------------------------------------------------

/// The mailing manager.
#[derive(Debug)]
pub struct MailingManager {
    mailer: Box<dyn Mailer>,
    state: RefCell<MailingState>,
}

impl MailingManager {
    /// Constructs a new `MailingManager`.
    pub fn new(mailer: Box<dyn Mailer>) -> Self {
        Self {
            mailer: mailer,
            state: RefCell::new(MailingState { number_of_sent_emails: 0, last_email: None }),
        }
    }

    /// Sends the given e-mail using the mailer.
    pub fn send(&mut self, email: EMail) -> Result<(), ()> {
        let result = self.mailer.send(email.clone());
        if result.is_ok() {
            self.state.borrow_mut().sent(email);
        }
        result
    }

    /// Returns the number of sent e-mails since creation.
    pub fn get_number_of_sent_emails(&self) -> u32 {
        self.state.borrow().number_of_sent_emails
    }

    /// Returns last sent e-mail.
    pub fn get_last_email(&self) -> Option<EMail> {
        self.state.borrow().last_email.clone()
    }
}

// -------------------------------------------------------------------------------------------------

/// BlueFire context extension for sending e-mails.
#[derive(Clone, Debug)]
pub struct MailingExtention {
    manager: Arc<Mutex<MailingManager>>,
}

impl MailingExtention {
    /// Constructs a new `MailingExtention`.
    pub fn new(manager: Arc<Mutex<MailingManager>>) -> Self {
        Self { manager }
    }

    /// Returns a mutex-secured reference to the mailing manager.
    pub fn lock_manager(&self) -> MutexGuard<MailingManager> {
        self.manager.lock().expect("BlueFire: Lock Mailing Manager")
    }
}

impl Extension for MailingExtention {
    fn get_name(&self) -> &str {
        "BlueFire:Mailing"
    }

    // TODO: Check connection with SMTP server.
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
