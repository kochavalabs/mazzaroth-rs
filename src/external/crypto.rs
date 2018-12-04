use super::externs::_get_key;

/// Calls a host function to Sha3_256 data and return the hash
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _get_key(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}
