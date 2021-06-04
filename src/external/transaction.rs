//! Provides access to transaction input and return values.

#[cfg(not(feature = "host-mock"))]
use super::externs::{_fetch_input, _fetch_sender, _input_length, _ret, PUBLIC_KEY_LENGTH};

#[cfg(feature = "host-mock")]
pub static mut ARGS: Option<Vec<u8>> = None;

#[cfg(feature = "host-mock")]
pub static mut SENDER: Option<Vec<u8>> = None;

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
/// ```ignore
/// use mazzaroth_rs::transaction;
/// let args = transaction::arguments();
/// let response = contract.execute(&args);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn arguments() -> Vec<u8> {
    let length = unsafe { _input_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        _fetch_input(args.as_mut_ptr());
    }

    args
}

#[cfg(feature = "host-mock")]
pub fn arguments() -> Vec<u8> {
    unsafe {
        match ARGS {
            Some(ref val) => val.clone(),
            None => vec![],
        }
    }
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
/// ```ignore
/// use mazzaroth_rs::transaction;
/// let response = contract.execute(&args);
/// transaction::ret(response);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn ret(values: Vec<u8>) {
    unsafe { _ret(values.as_ptr(), values.len()) };
}

#[cfg(feature = "host-mock")]
pub fn ret(_values: Vec<u8>) {}

#[cfg(not(feature = "host-mock"))]
pub fn sender() -> Vec<u8> {
    let mut args: Vec<u8> = Vec::with_capacity(PUBLIC_KEY_LENGTH);
    unsafe {
        args.set_len(PUBLIC_KEY_LENGTH);
        _fetch_sender(args.as_mut_ptr());
    }

    args
}

#[cfg(feature = "host-mock")]
pub fn sender() -> Vec<u8> {
    unsafe {
        match SENDER {
            Some(ref val) => val.clone(),
            None => vec![],
        }
    }
}

pub trait TransactionInfo {
    fn arguments(&self) -> Vec<u8>;
    fn ret(&self, _values: &[u8]);
    fn sender(&self) -> Vec<u8>;
}

#[derive(Clone)]
struct TransactionInfoImpl {}
impl TransactionInfo for TransactionInfoImpl {
    fn arguments(&self) -> Vec<u8> {
        arguments()
    }
    fn ret(&self, _values: &[u8]) {
        ret(_values.to_vec())
    }
    fn sender(&self) -> Vec<u8> {
        sender()
    }
}
impl Default for TransactionInfoImpl {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
#[cfg(feature = "host-mock")]
mod tests {
    use super::*;

    #[test]
    fn test_sender() {
        unsafe { SENDER = Some(vec![3, 3, 3]) };
        assert_eq!(vec![3, 3, 3], sender());
    }

    #[test]
    fn test_arguments() {
        unsafe { ARGS = Some(vec![4, 4, 4]) };
        assert_eq!(vec![4, 4, 4], arguments());
    }

    #[test]
    fn test_ret() {
        assert_eq!((), ret(vec![]));
    }
}
