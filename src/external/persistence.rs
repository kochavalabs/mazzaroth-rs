//! Provides access to the contract state to store and get key values

#[cfg(not(feature = "host-mock"))]
use super::externs::{_delete, _get, _get_length, _key_exists, _store};

#[cfg(feature = "host-mock")]
pub static mut STORE: Option<std::collections::HashMap<Vec<u8>, Vec<u8>>> = None;

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
/// ```ignore
/// use mazzaroth_wasm::persistence;
/// let value = persistence::get(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
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

#[cfg(feature = "host-mock")]
pub fn get(key: Vec<u8>) -> Result<Vec<u8>, ExternalError> {
    unsafe {
        match STORE {
            Some(ref store) => match store.get(&key) {
                Some(ref val) => Ok(val.clone().to_vec()),
                None => Err(ExternalError::MissingKeyError),
            },
            None => Err(ExternalError::MissingKeyError),
        }
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
/// ```ignore
/// use mazzaroth_wasm::persistence;
/// persistence::store(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn store(key: Vec<u8>, val: Vec<u8>) {
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

#[cfg(feature = "host-mock")]
pub fn store(key: Vec<u8>, val: Vec<u8>) {
    unsafe {
        match STORE {
            Some(ref mut store) => {
                store.insert(key, val);
            }
            None => {
                STORE = Some(std::collections::HashMap::new());
                store(key, val);
            }
        }
    }
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
/// ```ignore
/// use mazzaroth_wasm::persistence;
/// persistence::delete(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn delete(key: Vec<u8>) -> Result<(), ExternalError> {
    let exists = unsafe { _key_exists(key.as_ptr(), key.len()) };
    if exists {
        unsafe { _delete(key.as_ptr(), key.len()) };
        Ok(())
    } else {
        Err(ExternalError::MissingKeyError)
    }
}

#[cfg(feature = "host-mock")]
pub fn delete(key: Vec<u8>) -> Result<(), ExternalError> {
    unsafe {
        match STORE {
            Some(ref mut store) => match store.remove(&key) {
                Some(_) => Ok(()),
                None => Err(ExternalError::MissingKeyError),
            },
            None => Err(ExternalError::MissingKeyError),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "host-mock")]
mod tests {
    use super::*;

    #[test]
    fn test_get_miss() {
        assert_eq!(Err(ExternalError::MissingKeyError), get(vec![1, 2, 3]));
    }

    #[test]
    fn test_get() {
        store(vec![1, 2], vec![1, 1, 1, 1]);
        assert_eq!(Ok(vec![1, 1, 1, 1]), get(vec![1, 2]));
    }

    #[test]
    fn test_get_delete() {
        store(vec![3, 2], vec![1, 1, 1, 1]);
        delete(vec![3, 2]).unwrap();
        assert_eq!(Err(ExternalError::MissingKeyError), get(vec![3, 2]));
    }

}
