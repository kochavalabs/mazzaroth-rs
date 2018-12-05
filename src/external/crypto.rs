use super::externs::_sha256;

/// Calls a host function to Sha3_256 data and return the hash
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}
