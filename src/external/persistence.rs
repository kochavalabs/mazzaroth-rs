//! Provides access to the contract state to store and get key values

use super::externs::{_get, _get_length, _key_exists, _delete, _store};
use super::ExternalError;

/// Get the value associated with a string key from the persistent storage for this runtime.
///
/// # Arguments
///
/// * `key` - The Vec<u8> key used to get a value from state
///
/// # Returns
///
/// Result<Vec<u8>, ExternalError>
/// * `Vec<u8>` - The value stored in state if found
/// * `ExternalError` - Error if there is a problem getting the value stored in state
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::persistence;
/// let value = persistence::get(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn get(key: Vec<u8>) -> Result<Vec<u8>, ExternalError> {
    let exists = unsafe { _key_exists(key.as_ptr(), key.len()) };
    if exists {
        let len = unsafe { _get_length(key.as_ptr(), key.len()) };
        let mut val = Vec::with_capacity(len as usize);
        unsafe { val.set_len(len as usize) };
        unsafe { _get(key.as_ptr(), key.len(), val.as_mut_ptr()) };
        Ok(val)
    } else {
        Err(ExternalError::MissingKeyError)
    }
}

/// Store a key/value pair in the persistent storage for this runtime.
///
/// # Arguments
///
/// * `key` - The Vec<u8> key used to store a value in state
/// * `value` - The Vec<u8> value to store
///
/// # Returns
///
/// * `None`
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::persistence;
/// persistence::store(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn store(key: Vec<u8>, val: Vec<u8>) {
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

/// Delete a key from the contract state.
///
/// # Arguments
///
/// * `key` - The Vec<u8> key to be deleted
///
/// # Returns
///
/// Result<(), ExternalError>
/// * `Void` - simply returns OK if the delete was successful.
/// * `ExternalError` - Error if there is a problem deleting the value stored in state
///
/// # Example
///
/// ```
/// use mazzaroth_wasm::persistence;
/// persistence::delete(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn delete(key: Vec<u8>) -> Result<(), ExternalError> {
    let exists = unsafe { _key_exists(key.as_ptr(), key.len()) };
    if exists {
        unsafe { _delete(key.as_ptr(), key.len()) };
        Ok(())
    } else {
        Err(ExternalError::MissingKeyError)
    }
}
