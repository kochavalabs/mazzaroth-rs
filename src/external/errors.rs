//! Provides the ExternalError definitions and a panic hook.
//!
//! # How to use panic hook
//!
//! Call the std::panic::set_hook function as the first line of main():
//!
//! ```ignore
//! std::panic::set_hook(Box::new(mazzaroth_wasm::external::errors::hook));
//! ```

/// Panic hook for host runtime that has a _log_error function defined.
/// https://github.com/rustwasm/console_error_panic_hook
#[cfg(target_arch = "wasm32")]
use super::externs::_log_error;

use std::fmt;
use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn hook_impl(info: &panic::PanicInfo) {
            let val = info.to_string().into_bytes();
            unsafe { _log_error(val.as_ptr(), val.len()) };
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}

/// A panic hook that logs panics to extern _error when building with wasm32
///
/// # How to use
///
/// Call the std::panic::set_hook function as the first line of main():
///
/// ```ignore
/// std::panic::set_hook(Box::new(mazzaroth_wasm::external::errors::hook));
/// ```
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}

/// Defines the various errors that can be returned when calling external functions
#[derive(Debug, PartialEq)]
pub enum ExternalError {
    /// Occurs when calling `get` with a key that does not exist in state
    MissingKeyError,
    /// Occurs when the crypto `generate_key_pair` function fails
    KeyPairGenerateError,
    /// Occurs when calling the crypto `sign_message` function with a key of the wrong length
    KeyLengthError,
    /// Occurs when the crypto `sign_message` function fails
    SignMessageError,
}

impl std::fmt::Display for ExternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            ExternalError::MissingKeyError => "Could not find key in state.",
            ExternalError::KeyPairGenerateError => "Problem generating key pair.",
            ExternalError::KeyLengthError => "Incorrect key length.",
            ExternalError::SignMessageError => "Problem signing message.",
        };
        write!(f, "{}", message)
    }
}
