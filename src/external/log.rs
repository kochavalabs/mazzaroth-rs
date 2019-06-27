use super::externs::_log;

// public log function that wraps the host call
pub fn log(msg: String) {
    let val = msg.into_bytes();
    unsafe { _log(val.as_ptr(), val.len()) }
}
