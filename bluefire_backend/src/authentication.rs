// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This create provides authentication functionality:
//!  - provides password hashing algorithms
//!  - provides user and session definitions and traits
//!  - provides traits for authentication related data providers
//!  - implements user authentication middleware

// TODO: Define expect messages as consts.

use std::fmt::Debug;

use bluefire_twine::constants::*;
use bluefire_twine::id::Id;

use crate::clock::{Clock, ClockExtension};
use crate::common::{self, BlueFireError};
use crate::context::{BlueFire, Extension, Extensions, Middleware};
use crate::database::DataProvider;

// -------------------------------------------------------------------------------------------------

/// Provides password encode and check methods.
mod hash {
    use rand::{self, Rng};

    fn random_salt() -> String {
        rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(12).collect()
    }

    pub mod pbkdf2_sha256 {
        use crate::common::BlueFireError;

        const ITERATIONS: u32 = 100000;
        const HASH_LEN: usize = 32;
        pub const NAME: &str = "pbkdf2_sha256";

        fn encode(password: &str, salt: &str, iterations: u32) -> String {
            let mut result = [0u8; HASH_LEN];
            let mut mac =
                crypto::hmac::Hmac::new(crypto::sha2::Sha256::new(), &password.as_bytes());
            crypto::pbkdf2::pbkdf2(&mut mac, &salt.as_bytes(), iterations, &mut result);
            base64::encode_config(&result, base64::STANDARD)
        }

        pub fn check(encoded: &str, password: &str) -> Result<bool, BlueFireError> {
            let parts: Vec<&str> = encoded.split("$").collect();
            if parts.len() == 3 {
                let (iterations_str, salt, hash1) = (parts[0], parts[1], parts[2]);
                let iterations =
                    iterations_str.parse().map_err(|_| BlueFireError::invalid_password_hash())?;
                let hash2 = encode(password, &salt, iterations);
                Ok(hash1 == hash2)
            } else {
                Err(BlueFireError::invalid_password_hash())
            }
        }

        pub fn make_password(password: &str) -> String {
            let salt = super::random_salt();
            let hash = encode(password, &salt, ITERATIONS);
            format!("{}${}${}${}", NAME, ITERATIONS, salt, hash)
        }
    }
}

/// Enumeration of available password hash methods.
pub enum Algorithm {
    /// PBKDF2-SHA256
    Pbkdf2Sha256,

    /// Default algorithm (PBKDF2-SHA256)
    Default,
}

/// Calculates a hash of given password with given algorithm.
pub fn make_password(password: &str, algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::Pbkdf2Sha256 => hash::pbkdf2_sha256::make_password(password),
        Algorithm::Default => hash::pbkdf2_sha256::make_password(password),
    }
}

/// Checks if given password matches with given encoded password.
pub fn check_password(encoded: &str, password: &str) -> Result<bool, BlueFireError> {
    let parts: Vec<&str> = encoded.splitn(2, "$").collect();
    if parts.len() == 2 {
        let (algorithm, encoded_part) = (parts[0], parts[1]);
        match algorithm {
            hash::pbkdf2_sha256::NAME => hash::pbkdf2_sha256::check(encoded_part, password),
            _ => Err(BlueFireError::unknown_authentication_algorithm(algorithm.to_owned())),
        }
    } else {
        Err(BlueFireError::unknown_authentication_algorithm("???".to_owned()))
    }
}

// -------------------------------------------------------------------------------------------------

/// Role of a user.
pub type Role = u32;

/// Trait providing basic information about a user.
pub trait UserTrait: Debug + Send + Sync {
    /// Returns users ID.
    fn id(&self) -> &Id;

    /// Returns the username.
    fn username(&self) -> &String; // TODO: change to &str

    /// Returns users e-mail.
    fn email(&self) -> &String;

    /// Returns an encoded version of the password.
    fn encoded_password(&self) -> &String;

    /// Checks if the user has the given role.
    fn has_role(&self, role: Role) -> bool;

    /// Checks if the user is active.
    fn is_active(&self) -> bool;

    /// Duplicates the user.
    fn duplicate(&self) -> Box<dyn UserTrait>;
}

/// Simple structure implementing `UserTrait`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    id: Id,
    username: String,
    email: String,
    encoded_password: String,
    roles: Vec<Role>,
    is_active: bool,
}

impl User {
    /// Constructs a new `User`.
    pub fn new(
        id: Id,
        username: String,
        email: String,
        encoded_password: String,
        roles: Vec<Role>,
        is_active: bool,
    ) -> Self {
        Self { id, username, email, encoded_password, roles, is_active }
    }
}

impl UserTrait for User {
    fn id(&self) -> &Id {
        &self.id
    }

    fn username(&self) -> &String {
        &self.username
    }

    fn email(&self) -> &String {
        &self.email
    }

    fn encoded_password(&self) -> &String {
        &self.encoded_password
    }

    fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn duplicate(&self) -> Box<dyn UserTrait> {
        Box::new(self.clone())
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait providing basic information about a user session.
pub trait SessionTrait: Debug + Send + Sync {
    /// Returns the session ID.
    fn id(&self) -> Id;

    /// Returns the expiration date.
    fn valid_to(&self) -> chrono::DateTime<chrono::Utc>;

    /// Duplicates the session.
    fn duplicate(&self) -> Box<dyn SessionTrait>;
}

/// Simple structure implementing `SessionTrait`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    id: Id,
    valid_to: chrono::DateTime<chrono::Utc>,
}

impl Session {
    /// Constructs a new `Session`.
    pub fn new(id: Id, valid_to: chrono::DateTime<chrono::Utc>) -> Self {
        Self { id, valid_to }
    }
}

impl SessionTrait for Session {
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn valid_to(&self) -> chrono::DateTime<chrono::Utc> {
        self.valid_to
    }

    fn duplicate(&self) -> Box<dyn SessionTrait> {
        Box::new(self.clone())
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure proving information about user and session.
///
/// This structure implements `Extension` trait and is set by `AuthenticationMiddleware` as and
/// extension to the context.
#[derive(Debug)]
pub struct UserInfo {
    user: Option<Box<dyn UserTrait>>,
    session: Option<Box<dyn SessionTrait>>,
}

impl UserInfo {
    /// Constructs a new `UserInfo`.
    pub fn new(user: Box<dyn UserTrait>, session: Box<dyn SessionTrait>) -> Self {
        Self { user: Some(user), session: Some(session) }
    }

    /// Constructs a new empty `UserInfo` .
    pub fn new_empty() -> Self {
        Self { user: None, session: None }
    }

    /// Tells if a user is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.session.is_some()
    }

    /// Returns information about user if any authenticated.
    pub fn get_user(&self) -> Option<&Box<dyn UserTrait>> {
        self.user.as_ref()
    }

    /// Returns information about user session if available.
    pub fn get_session(&self) -> Option<&Box<dyn SessionTrait>> {
        self.session.as_ref()
    }
}

impl Extension for UserInfo {
    fn get_name(&self) -> &str {
        "BlueFire:UserInfo"
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

impl Clone for UserInfo {
    fn clone(&self) -> Self {
        Self {
            user: {
                if let Some(ref user) = self.user {
                    Some(user.duplicate())
                } else {
                    None
                }
            },
            session: {
                if let Some(ref session) = self.session {
                    Some(session.duplicate())
                } else {
                    None
                }
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// User info for use in templates.
pub struct UserTemplateInfo {
    /// Tells if the user is authenticated.
    pub is_authenticated: bool,

    /// Username. Empty is the user is not authenticated.
    pub username: String,
}

impl UserTemplateInfo {
    /// Constructs a new `UserTemplateInfo` basing on information from `UserInfo` extension.
    pub fn new(context: &BlueFire) -> Self {
        if let Some(info) = context.extension::<UserInfo>() {
            if let Some(user) = info.get_user() {
                Self::new_authenticated(user.username().to_string())
            } else {
                Self::new_not_authenticated()
            }
        } else {
            Self::new_not_authenticated()
        }
    }

    /// Constructs a new authenticated `UserTemplateInfo`.
    pub fn new_authenticated(username: String) -> Self {
        Self { is_authenticated: true, username: username }
    }

    /// Constructs a new unauthenticated `UserTemplateInfo`.
    pub fn new_not_authenticated() -> Self {
        Self { is_authenticated: false, username: "".to_string() }
    }
}

// -------------------------------------------------------------------------------------------------

/// Describes an outcome of user creation.
#[derive(Debug)]
pub enum CreationOutcome {
    /// Creation succeeded.
    Success {
        /// Info about newly created user.
        user: Box<dyn UserTrait>,
        /// Activation token.
        activation_token: String,
    },
    /// A user is already authenticated.
    UserAuthenticated,
    /// A user with requested credentials already exists in the database.
    UserAlreadyExists,
    /// A user with requested e-mail already exists in the database.
    EmailAlreadyExists,
    /// Other error.
    InternalError,
}

impl CreationOutcome {
    /// Constructs a new successful result.
    pub fn success(user: Box<dyn UserTrait>, activation_token: String) -> Self {
        CreationOutcome::Success { user, activation_token }
    }

    /// Checks if the creation was successful.
    pub fn is_success(&self) -> bool {
        if let CreationOutcome::Success { .. } = self {
            true
        } else {
            false
        }
    }
}

/// Describes a result of user creation.
pub type CreationResult = Result<CreationOutcome, Box<dyn std::error::Error>>;

/// Describes a result of user activation.
#[derive(Debug, PartialEq, Eq)]
pub enum ActivationOutcome {
    /// Activation succeeded
    Success,
    /// A user is already authenticated.
    UserAuthenticated,
    /// The user is already active.
    UserAlreadyActive,
    /// The used activation token does not exists in the database or already expired.
    TokenDoesNotExistOrExpired,
    /// Other error.
    InternalError,
}

impl ActivationOutcome {
    /// Constructs a new successful result.
    pub fn success() -> Self {
        ActivationOutcome::Success
    }

    /// Checks if the activation was successful.
    pub fn is_success(&self) -> bool {
        *self == ActivationOutcome::Success
    }

    /// Checks if a user was authenticated.
    pub fn is_authenticated(&self) -> bool {
        *self == ActivationOutcome::UserAlreadyActive
    }

    /// Checks if the user was already active.
    pub fn is_already_active(&self) -> bool {
        *self == ActivationOutcome::UserAlreadyActive
    }

    /// Checks if the token already expired.
    pub fn is_expired(&self) -> bool {
        *self == ActivationOutcome::TokenDoesNotExistOrExpired
    }
}

/// Describes a result of user activation.
pub type ActivationResult = Result<ActivationOutcome, Box<dyn std::error::Error>>;

/// Describes an outcome of user login.
#[derive(Debug)]
pub enum LoginOutcome {
    /// Login succeeded
    Success {
        /// Info about newly logged user.
        user: Box<dyn UserTrait>,
        /// The new session ID.
        session_id: Id,
    },
    /// The user was already logged in.
    UserAlreadyLoggedIn,
    /// The account has not been activated yet.
    AccountInactive,
    /// Given password did not match the user or user does not exist.
    WrongUsernameOrPassword,
    /// Other error.
    InternalError,
}

impl LoginOutcome {
    /// Constructs a new successful result.
    pub fn success(user: Box<dyn UserTrait>, session_id: Id) -> Self {
        LoginOutcome::Success { user: user, session_id: session_id }
    }

    /// Checks if the login was successful.
    pub fn is_success(&self) -> bool {
        if let LoginOutcome::Success { .. } = self {
            true
        } else {
            false
        }
    }

    /// Checks if the user was already logged in.
    pub fn is_already_logged_in(&self) -> bool {
        if let LoginOutcome::UserAlreadyLoggedIn { .. } = self {
            true
        } else {
            false
        }
    }

    /// Checks if the account has not been activated yet.
    pub fn is_account_inactive(&self) -> bool {
        if let LoginOutcome::AccountInactive { .. } = self {
            true
        } else {
            false
        }
    }

    /// Checks is the given password did not match the user or user does not exist.
    pub fn is_wrong_username_or_password(&self) -> bool {
        if let LoginOutcome::WrongUsernameOrPassword { .. } = self {
            true
        } else {
            false
        }
    }
}

/// Describes a result of user login.
pub type LoginResult = Result<LoginOutcome, Box<dyn std::error::Error>>;

/// Describes an outcome of user logout.
#[derive(Debug, PartialEq, Eq)]
pub enum LogoutOutcome {
    /// Logout succeeded
    Success,
    /// No user was logged in.
    NotLoggedIn,
    /// Other error.
    InternalError,
}

impl LogoutOutcome {
    /// Constructs a new successful result.
    pub fn success() -> Self {
        LogoutOutcome::Success
    }

    /// Checks if the login was successful.
    pub fn is_success(&self) -> bool {
        *self == LogoutOutcome::Success
    }
}

/// Describes a result of user logout.
pub type LogoutResult = Result<LogoutOutcome, Box<dyn std::error::Error>>;

// -------------------------------------------------------------------------------------------------

/// Example query parameters for `UserDataProvider`.
/// If needed different parameters may be used when implementing the provider.
#[derive(Clone, Debug)]
pub struct UserQueryParams {
    /// Username.
    pub username: String,
}

impl UserQueryParams {
    /// Constructs a new `UserQueryParams`.
    pub fn new(username: String) -> Self {
        UserQueryParams { username: username }
    }
}

/// Trait for providing access to database for user related tasks.
///
/// The functionality of checking password, user activeness, and so on is provided by `create_user`
/// and `login_user`. The programmer is only required to implement access to the database in
/// `*_unchecked` methods. They are marked as `unsafe` as they should not be used alone.
pub trait UserDataProvider: DataProvider + Clone {
    /// Return user information.
    fn get_user(&self) -> Option<&Box<dyn UserTrait>>;

    /// Create a new user without checking validity of this operation.
    ///
    /// This method will be called by `create_user` if:
    ///  - the user is not found in the database or
    ///  - is found, but is inactive.
    ///
    /// There is no check for whether a user is already logged in.
    unsafe fn create_user_unchecked(
        &mut self,
        db: &Self::Database,
        email: &str,
        password: &str,
        clock: &Box<dyn Clock>,
    ) -> CreationResult;

    /// Log user in without checking validity of this operation.
    ///
    /// This method is called if:
    ///  - the user was found in the database and
    ///  - it is active and
    ///  - the password matches with the one in the database.
    ///
    /// There is no check for whether a user is already logged in.
    unsafe fn login_user_unchecked(
        &mut self,
        db: &Self::Database,
        user: &Box<dyn UserTrait>,
        clock: &Box<dyn Clock>,
    ) -> LoginResult;

    /// Create a new user.
    fn create_user(
        &mut self,
        db: &Self::Database,
        email: &str,
        password: &str,
        clock: &Box<dyn Clock>,
    ) -> CreationResult {
        if let Some(user) = self.get_user() {
            if user.is_active() {
                Ok(CreationOutcome::UserAlreadyExists)
            } else {
                unsafe { self.create_user_unchecked(db, email, password, clock) }
            }
        } else {
            unsafe { self.create_user_unchecked(db, email, password, clock) }
        }
    }

    /// Log the user in.
    fn login_user(
        &mut self,
        db: &Self::Database,
        password: &str,
        clock: &Box<dyn Clock>,
    ) -> LoginResult {
        let user = {
            if let Some(user) = self.get_user() {
                if user.is_active() {
                    if check_password(&user.encoded_password(), password)? {
                        user.duplicate()
                    } else {
                        return Ok(LoginOutcome::WrongUsernameOrPassword);
                    }
                } else {
                    return Ok(LoginOutcome::AccountInactive);
                }
            } else {
                // The user does not exists
                return Ok(LoginOutcome::WrongUsernameOrPassword);
            }
        };

        unsafe { self.login_user_unchecked(db, &user, clock) }
    }
}

// -------------------------------------------------------------------------------------------------

/// Example query parameters for `ActivationDataProvider`.
/// If needed different parameters may be used when implementing the provider.
#[derive(Clone, Debug)]
pub struct ActivationQueryParams {
    /// Activation token.
    pub token: Id,
}

impl ActivationQueryParams {
    /// Constructs a new `ActivationQueryParams`.
    pub fn new(token: Id) -> Self {
        ActivationQueryParams { token }
    }
}

/// Trait for providing access to database for user activation related tasks.
///
/// The functionality of checking the validity of the user is provided by `activate_user`. The
/// programmer is only required to implement access to the database in `*_unchecked` methods. They
/// are marked as `unsafe` as they should not be used alone.
pub trait ActivationDataProvider: DataProvider + Clone {
    /// Return user information.
    fn get_user(&self) -> Option<&Box<dyn UserTrait>>;

    /// Activate the user without checking validity of this operation.
    ///
    /// This method is called if:
    ///  - the user was found in the database and
    ///  - it is not active.
    unsafe fn activate_user_unchecked(
        &mut self,
        db: &Self::Database,
        clock: &Box<dyn Clock>,
    ) -> ActivationResult;

    /// Activate the user.
    fn activate_user(&mut self, db: &Self::Database, clock: &Box<dyn Clock>) -> ActivationResult {
        if let Some(user) = self.get_user() {
            if user.is_active() {
                Ok(ActivationOutcome::UserAlreadyActive)
            } else {
                unsafe { self.activate_user_unchecked(db, clock) }
            }
        } else {
            Ok(ActivationOutcome::TokenDoesNotExistOrExpired)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Example query parameters for `AuthenticationDataProvider`.
/// If needed different parameters may be used when implementing the provider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationQueryParams {
    /// The ID of the session.
    pub session_id: Id,
}

impl AuthenticationQueryParams {
    /// Constructs a new `AuthenticationQueryParams`.
    pub fn new(session_id: Id) -> Self {
        AuthenticationQueryParams { session_id: session_id }
    }
}

/// The trait implmented by structs providing data required to authenticate a user.
pub trait AuthenticationDataProvider:
    DataProvider<QueryParams = AuthenticationQueryParams> + Clone
{
    /// Returns user information.
    fn get_user(&self) -> Option<&Box<dyn UserTrait>>;

    /// Returns user session information.
    fn get_session(&self) -> Option<&Box<dyn SessionTrait>>;

    /// Logs the user out.
    fn logout_user(&mut self, db: &Self::Database) -> LogoutResult;

    /// Returns the user info.
    fn get_user_info(&self) -> UserInfo {
        UserInfo {
            user: self.get_user().map(|user| user.duplicate()),
            session: self.get_session().map(|session| session.duplicate()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// `AuthenticationMiddleware` reads session cookie and if it matches with sessions in database
/// updates user data in `UserInfo` context extension.
#[derive(Clone, Debug)]
pub struct AuthenticationMiddleware<P>
where
    P: AuthenticationDataProvider,
{
    phantom: std::marker::PhantomData<P>,
}

impl<P> AuthenticationMiddleware<P>
where
    P: AuthenticationDataProvider,
{
    /// Constructs a new `AuthenticationDataProvider`.
    pub fn new() -> Box<Self> {
        Box::new(Self { phantom: std::marker::PhantomData })
    }
}

impl<P> Middleware for AuthenticationMiddleware<P>
where
    P: AuthenticationDataProvider + Send + Sync + 'static,
{
    fn apply(&mut self, extensions: &mut Extensions, request: &common::Request) {
        let info = {
            if let Some(session_id) = self.get_session_id(request) {
                let authentication_query_params = AuthenticationQueryParams::new(session_id);
                let clock = extensions
                    .get::<ClockExtension>()
                    .expect("Expected clock extension not provided");
                let db = extensions
                    .get::<P::Database>()
                    .expect("Expected database implementation not provided");
                let data_provider = P::create(&db, &authentication_query_params);
                match data_provider {
                    Ok(data_provider) => {
                        let user_info = {
                            if let Some(session) = data_provider.get_session() {
                                if clock.now() < session.valid_to() {
                                    data_provider.get_user_info()
                                } else {
                                    UserInfo::new_empty()
                                }
                            } else {
                                UserInfo::new_empty()
                            }
                        };
                        user_info
                    }
                    Err(err) => {
                        log_error!("Failed to construct authentication provider: {}", err);
                        UserInfo::new_empty()
                    }
                }
            } else {
                UserInfo::new_empty()
            }
        };

        extensions.add(info);
    }

    fn duplicate(&self) -> Box<dyn Middleware> {
        Box::new(self.clone())
    }
}

impl<P> AuthenticationMiddleware<P>
where
    P: AuthenticationDataProvider + Send + Sync + 'static,
{
    fn get_session_id_from_cookie(&self, request: &common::Request) -> Option<Id> {
        let cookies = request.headers().get_all(http::header::COOKIE);
        for cookie in cookies.iter() {
            if let Ok(cookie_str) = cookie.to_str() {
                if cookie_str.starts_with(SESSION_COOKIE_PREFIX) {
                    let id_str = &cookie_str[SESSION_COOKIE_PREFIX.len()..];
                    let result = Id::from_str(&id_str);
                    match result {
                        Ok(id) => return Some(id),
                        Err(err) => {
                            log_warn!("Wrong session cookie: {} ({})", err, cookie_str);
                        }
                    }
                }
            }
        }
        None
    }

    fn get_session_id_from_header(&self, request: &common::Request) -> Option<Id> {
        if let Some(token) = request.headers().get(BLUEFIRE_TOKEN_HEADER) {
            match token.to_str() {
                Ok(value) => Id::from_str(value).ok(),
                Err(..) => None,
            }
        } else {
            None
        }
    }

    fn get_session_id(&self, request: &common::Request) -> Option<Id> {
        let session_id = self.get_session_id_from_header(request);
        if session_id.is_some() {
            session_id
        } else {
            self.get_session_id_from_cookie(request)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper method for logging a user out.
pub fn logout_user<P>(context: &BlueFire) -> Result<LogoutOutcome, Box<dyn UserTrait>>
where
    P: AuthenticationDataProvider,
{
    let user_info = context.extension::<UserInfo>().expect("Get UserInfo");
    if user_info.is_authenticated() {
        let user = user_info.get_user().expect("Get user");
        let session = user_info.get_session().expect("Get session");

        let db =
            context.extension::<<P as DataProvider>::Database>().expect("Database not provided");
        let params = P::QueryParams::new(session.id());
        let provider = P::create(&db, &params);
        match provider {
            Ok(mut provider) => provider.logout_user(&db).map_err(|_| user.duplicate()),
            Err(err) => {
                log::error!("Failed to logout the user: {}", err);
                Err(user.duplicate())
            }
        }
    } else {
        Ok(LogoutOutcome::NotLoggedIn)
    }
}

// -------------------------------------------------------------------------------------------------

/// Prelude for `authentication` module.
pub mod prelude {
    pub use super::{
        logout_user, make_password, ActivationDataProvider, ActivationOutcome,
        ActivationQueryParams, ActivationResult, Algorithm, AuthenticationDataProvider,
        AuthenticationMiddleware, AuthenticationQueryParams, CreationOutcome, CreationResult,
        LoginOutcome, LoginResult, LogoutOutcome, LogoutResult, Session, SessionTrait, User,
        UserDataProvider, UserQueryParams, UserTrait,
    };
}
