use super::externs::{_get_account_name, _get_account_name_length, _set_account_name, _get_account_balance, _set_account_balance, _is_owner};
use std::str;

/// Get the value associated with a string key from the persistent storage for this runtime.
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
pub fn set_name(key: Vec<u8>, name: String) {
    let val = name.into_bytes();
    unsafe { _set_account_name(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

/// Get the balance associate with an account in the persistent storage for this runtime.
pub fn get_balance(key: Vec<u8>) -> u64 {
    let bal = unsafe { _get_account_balance(key.as_ptr(), key.len()) };
    bal
}

/// Set the balance associate with an account in the persistent storage for this runtime.
pub fn set_balance(key: Vec<u8>, balance: u64) {
    unsafe { _set_account_balance(key.as_ptr(), key.len(), balance) };
}

/// Check if an account is the owner of the contract.
pub fn is_owner(key: Vec<u8>) -> bool {
    let ret = unsafe { _is_owner(key.as_ptr(), key.len()) };
    ret
}