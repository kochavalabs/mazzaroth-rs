use super::externs::{_sha256, _sha3_256, _keccak256, _shake256};

/// Calls a host function to Sha3_256 data and return the hash
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Sha3_256 data and return the hash
pub fn sha3_256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha3_256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Sha3_256 data and return the hash
pub fn keccak256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _keccak256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Sha3_256 data and return the hash
pub fn shake256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _shake256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}
