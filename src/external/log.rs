use super::externs::_log;

/// Write a message to the host defined log location.
pub fn log(msg: String) {
    unsafe { _log(msg) }
}
