#[cfg(not(feature = "host-mock"))]
use super::externs::_log;

/// Write a message to the host defined log location.
#[cfg(not(feature = "host-mock"))]
pub fn log(msg: String) {
    let val = msg.into_bytes();
    unsafe { _log(val.as_ptr(), val.len()) }
}

#[cfg(feature = "host-mock")]
pub fn log(msg: String) {
    println!("log {}", msg);
}

#[cfg(test)]
#[cfg(feature = "host-mock")]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        log("asdf".to_string());
    }
}
