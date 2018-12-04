use super::externs::{_get, _get_length, _store};

/// Get the value associated with a string key from the persistent storage for this runtime.
pub fn get(key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    let len = unsafe { _get_length(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    val
}

pub fn get_bytes_key(key: Vec<u8>) -> Vec<u8> {
    let len = unsafe { _get_length(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    val
}

/// Store a key/value pair in the persistent storage for this runtime.
pub fn store(key: &str, val: Vec<u8>) {
    let key = key.as_bytes();
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

pub fn store_bytes_key(key: Vec<u8>, val: Vec<u8>) {
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}
