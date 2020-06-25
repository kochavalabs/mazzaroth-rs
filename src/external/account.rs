//! Provides access to contract account objects stored in state.

#[cfg(not(feature = "host-mock"))]
use super::externs::{_get_account_name, _get_account_name_length, _is_owner, _set_account_name};

#[cfg(not(feature = "host-mock"))]
use std::str;

#[cfg(feature = "host-mock")]
pub static mut NAME: Option<String> = None;

#[cfg(feature = "host-mock")]
pub static mut OWNER: bool = false;

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
/// ```ignore
/// use mazzaroth_rs::account;
/// let name = account::get_name(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn get_name(key: Vec<u8>) -> String {
    let len = unsafe { _get_account_name_length(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get_account_name(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    // Convert name to String
    str::from_utf8(&val).unwrap().to_owned()
}

#[cfg(feature = "host-mock")]
pub fn get_name(_key: Vec<u8>) -> String {
    unsafe {
        match NAME.clone() {
            Some(name) => name,
            None => "".to_string(),
        }
    }
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
/// ```ignore
/// use mazzaroth_rs::account;
/// account::set_name(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "name");
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn set_name(key: Vec<u8>, name: String) {
    let val = name.into_bytes();
    unsafe { _set_account_name(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

#[cfg(feature = "host-mock")]
pub fn set_name(_key: Vec<u8>, name: String) {
    unsafe {
        NAME = Some(name.clone());
    }
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
/// ```ignore
/// use mazzaroth_rs::account;
/// let is_owner = account::is_owner(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn is_owner(key: Vec<u8>) -> bool {
    unsafe { _is_owner(key.as_ptr(), key.len()) }
}

#[cfg(feature = "host-mock")]
pub fn is_owner(_key: Vec<u8>) -> bool {
    unsafe { OWNER }
}

#[cfg(test)]
#[cfg(feature = "host-mock")]
mod tests {
    use super::*;

    #[test]
    fn test_is_owner_true() {
        unsafe {
            OWNER = true;
        }
        assert_eq!(is_owner(vec![]), true);
    }

    #[test]
    fn test_get_name_set() {
        set_name(vec![], "asdf".to_string());
        assert_eq!(get_name(vec![]), "asdf".to_string());
    }
}
