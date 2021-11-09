//! Provides access to contract account objects stored in state.

#[cfg(not(feature = "host-mock"))]
use super::externs::_is_owner;

#[cfg(feature = "host-mock")]
pub static mut OWNER: bool = false;

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
}
