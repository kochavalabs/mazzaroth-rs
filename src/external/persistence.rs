use super::externs::{_get, _get_exists, _get_length, _store};
use super::ExternalError;

/// Get the value associated with a string key from the persistent storage for this runtime.
pub fn get(key: Vec<u8>) -> Result<Vec<u8>, ExternalError> {
    let exists = unsafe { _get_exists(key.as_ptr(), key.len()) };
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
pub fn store(key: Vec<u8>, val: Vec<u8>) {
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}
