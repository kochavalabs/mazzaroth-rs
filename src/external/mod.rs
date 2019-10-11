//! Defines the external modules used to interact with host runtime.
//!
//! These modules and functions can be used to interact with the Mazzaroth VM.

// Externs includes the direct calls to host runtime
// Functions are wrapped by functions in other modules
#[cfg(not(feature = "host-mock"))]
pub(crate) mod externs;

pub mod account;

pub mod persistence;

pub mod query;

pub mod crypto;

pub mod transaction;

pub mod errors;
pub use self::errors::ExternalError;

mod log;
pub use self::log::log;
