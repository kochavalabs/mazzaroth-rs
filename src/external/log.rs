use super::externs::_log;

// public log function that wraps the host call
pub fn log(msg: String) {
    unsafe { _log(msg) }
}
