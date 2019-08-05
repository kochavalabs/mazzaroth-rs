#[cfg(not(test))]
use super::externs::_log;

/// Write a message to the host defined log location.
#[cfg(not(test))]
pub fn log(msg: String) {
    let val = msg.into_bytes();
    unsafe { _log(val.as_ptr(), val.len()) }
}

#[cfg(test)]
pub fn log(msg: String) {
    println!("log {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        log("asdf".to_string());
    }

}
