use super::externs::{
    _generate_key_pair, _keccak256, _sha256, _sha3_256, _sha3_512, _shake256, _sign_message,
    _validate_signature,
};

/// Calls a host function to Sha256 data and return the hash
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

/// Calls a host function to Sha3_512 data and return the hash
pub fn sha3_512(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(64 as usize); // 64 byte (512) hash
    unsafe { hash.set_len(64 as usize) };

    unsafe { _sha3_512(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to keccak256 data and return the hash
pub fn keccak256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _keccak256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to shake256 data and return the hash
pub fn shake256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _shake256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Host hashing function for generating a cryptographic key pair.
/// Currently returns a P256 elliptic curve key pair, 32byte private key
/// and 64 byte public key
pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let mut priv_key = Vec::with_capacity(32 as usize);
    let mut pub_key = Vec::with_capacity(64 as usize);
    unsafe { priv_key.set_len(32 as usize) };
    unsafe { pub_key.set_len(64 as usize) };

    unsafe { _generate_key_pair(priv_key.as_mut_ptr(), pub_key.as_mut_ptr()) };

    match priv_key.iter().any(|x| *x != 0x0u8) {
        true => Ok((priv_key, pub_key)),
        false => Err("Problem generating key pair."),
    }
}

/// Signs a message using the provided private key. You typically wouldn't be
/// signing something by sending your private key to the network, so this is
/// mostly for demonstration purposes.
/// It uses a 32 byte P256 elliptic curve private key and returns a 64 byte
/// signature.
pub fn sign_message(priv_key: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut signature = Vec::with_capacity(64 as usize);
    unsafe { signature.set_len(64 as usize) };

    unsafe {
        _sign_message(
            priv_key.as_ptr(),
            message.as_ptr(),
            message.len(),
            signature.as_mut_ptr(),
        )
    };

    match signature.iter().any(|x| *x != 0x0u8) {
        true => Ok(signature),
        false => Err("Problem signing message."),
    }
}

/// Validates a signature using the provided public key. A Mazzaroth user's
/// account address can be used as the public key to verify transactions sent
/// from that user.
/// We are currently using a 64 byte P256 elliptic curve public key and a 64
/// byte signature string.
/// 0 = False
/// 1 = True
pub fn validate_signature(pub_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> u32 {
    let result = unsafe {
        _validate_signature(
            pub_key.as_ptr(),
            message.as_ptr(),
            message.len(),
            signature.as_ptr(),
        )
    };
    result
}
