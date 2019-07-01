//! Provides access to contract account objects stored in state.

use super::externs::{_get_account_name, _get_account_name_length, _is_owner, _set_account_name};
use std::str;

/// Get the value associated with a string key from the persistent storage for this runtime.
///
/// # Arguments
///
/// * `key` - The public key of the account to access
///
/// # Returns
///
/// * `String` - The name stored in the account object
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::account;
/// let name = account::get_name(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn get_name(key: Vec<u8>) -> String {
    let len = unsafe { _get_account_name_length(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get_account_name(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    // Convert name to String
    let result = str::from_utf8(&val).unwrap().to_owned();
    result
}

/// Store an account name in the persistent storage for this runtime.
///
/// # Arguments
///
/// * `key` - The public key of the account to access
/// * `name` - The String name to store in the account
///
/// # Returns
///
/// * `None`
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::account;
/// account::set_name(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "name");
/// ```
pub fn set_name(key: Vec<u8>, name: String) {
    let val = name.into_bytes();
    unsafe { _set_account_name(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

/// Check if an account is the owner of the channel.
///
/// # Arguments
///
/// * `key` - The public key of the account to access
///
/// # Returns
///
/// * `Bool` - True if the account is the channel owner
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::account;
/// let is_owner = account::is_owner(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn is_owner(key: Vec<u8>) -> bool {
    let ret = unsafe { _is_owner(key.as_ptr(), key.len()) };
    ret
}
