//! Provides access to transaction input and return values.

use super::externs::{_fetch_input, _fetch_sender, _input_length, _ret, PUBLIC_KEY_LENGTH};

/// Get the arguments encoded from the runtime input to be supplied to contract execute
///
/// # Arguments
///
/// * `None`
///
/// # Returns
///
/// * `Vec<u8>` - The encoded Vec<u8> argument value
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::transaction;
/// let args = transaction::arguments();
/// let response = contract.execute(&args);
/// ```
pub fn arguments() -> Vec<u8> {
    let length = unsafe { _input_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        _fetch_input(args.as_mut_ptr());
    }

    args
}

/// Return encoded bytes to the runtime
///
/// # Arguments
///
/// * `Vec<u8>` - The encoded Vec<u8> return values
///
/// # Returns
///
/// * `None`
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::transaction;
/// let response = contract.execute(&args);
/// transaction::ret(response);
/// ```
pub fn ret(values: Vec<u8>) {
    unsafe { _ret(values.as_ptr(), values.len()) };
}

pub fn sender() -> Vec<u8> {
    let mut args: Vec<u8> = Vec::with_capacity(PUBLIC_KEY_LENGTH);
    unsafe {
        args.set_len(PUBLIC_KEY_LENGTH);
        _fetch_sender(args.as_mut_ptr());
    }

    args
}
