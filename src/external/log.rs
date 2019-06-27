use super::externs::_log;

/// Write a message to the host defined log location.
pub fn log(msg: String) {
    let val = msg.into_bytes();
    unsafe { _log(val.as_ptr(), val.len()) }
}
