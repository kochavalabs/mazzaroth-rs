use super::externs::{_get_account_name, _get_account_name_length, _set_account_name, _get_account_balance, _set_account_balance};

/// Get the value associated with a string key from the persistent storage for this runtime.
pub fn get_account_name(key: Vec<u8>) -> Vec<u8> {
    let len = unsafe { _get_account_name_length(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get_account_name(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    val
}

/// Store an account name in the persistent storage for this runtime.
pub fn set_account_name(key: Vec<u8>, val: Vec<u8>) {
    unsafe { _set_account_name(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}

/// Get the balance associate with an account in the persistent storage for this runtime.
pub fn get_account_balance(key: Vec<u8>) -> u64 {
    let bal = unsafe { _get_account_balance(key.as_ptr(), key.len()) };
    bal
}

/// Set the balance associate with an account in the persistent storage for this runtime.
pub fn set_account_balance(key: Vec<u8>, balance: u64) {
    unsafe { _set_account_balance(key.as_ptr(), key.len(), balance) };
}