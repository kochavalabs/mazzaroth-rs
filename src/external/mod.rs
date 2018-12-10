pub mod externs;

pub mod persistence;

pub mod crypto;

pub mod transaction;

pub mod errors;

// public log function that wraps the host call
pub fn log(msg: String) {
    unsafe { externs::_log(msg) }
}