//! Provides a set of cryptographic functions for use in contracts.
use super::externs::{
    _generate_key_pair, _keccak256, _sha256, _sha3_256, _sha3_512, _shake256, _sign_message,
    _validate_signature, PRIVATE_KEY_LENGTH, PUBLIC_KEY_LENGTH
};
use super::ExternalError;

/// Calls a host function to Sha256 data and return the hash
/// 
/// # Arguments
/// 
/// * `data` - The Vec<u8> data to hash
/// 
/// # Returns
/// 
/// * `Vec<u8>` - The cryptographic hash generated
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let hash = crypto::sha256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Sha3_256 data and return the hash
/// 
/// # Arguments
/// 
/// * `data` - The Vec<u8> data to hash
/// 
/// # Returns
/// 
/// * `Vec<u8>` - The cryptographic hash generated
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let hash = crypto::sha3_256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn sha3_256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha3_256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Sha3_512 data and return the hash
/// 
/// # Arguments
/// 
/// * `data` - The Vec<u8> data to hash
/// 
/// # Returns
/// 
/// * `Vec<u8>` - The cryptographic hash generated
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let hash = crypto::sha3_512(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn sha3_512(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(64 as usize); // 64 byte (512) hash
    unsafe { hash.set_len(64 as usize) };

    unsafe { _sha3_512(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Keccak256 data and return the hash
/// 
/// # Arguments
/// 
/// * `data` - The Vec<u8> data to hash
/// 
/// # Returns
/// 
/// * `Vec<u8>` - The cryptographic hash generated
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let hash = crypto::keccak256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn keccak256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _keccak256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Calls a host function to Shake256 data and return the hash
/// 
/// # Arguments
/// 
/// * `data` - The Vec<u8> data to hash
/// 
/// # Returns
/// 
/// * `Vec<u8>` - The cryptographic hash generated
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let hash = crypto::shake256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn shake256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _shake256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

/// Host hashing function for generating a cryptographic key pair.
/// 
/// # Arguments
/// 
/// * `None`
/// 
/// # Returns
/// 
/// Result<(Vec<u8>, Vec<u8>), ExternalError)
/// * `Vec<u8>` - The X25519 32 byte private key
/// * `Vec<u8>` - The X25519 32 byte public key
/// * `ExternalError` - Error if there is a problem generating key pair
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let (priv_key, pub_key) = crypto::generate_key_pair().unwrap();
/// ```
pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let mut priv_key = Vec::with_capacity(32 as usize);
    let mut pub_key = Vec::with_capacity(32 as usize);
    unsafe { priv_key.set_len(32 as usize) };
    unsafe { pub_key.set_len(32 as usize) };

    unsafe { _generate_key_pair(priv_key.as_mut_ptr(), pub_key.as_mut_ptr()) };

    match priv_key.iter().any(|x| *x != 0x0u8) {
        true => Ok((priv_key, pub_key)),
        false => Err("Problem generating key pair."),
    }
}

/// Signs a message using the provided private key. 
/// 
/// You typically wouldn't be signing something by sending your private key
/// to the network, so this is mostly for demonstration purposes.
/// 
/// # Arguments
/// 
/// * `priv_key` - The Vec<u8> 32 byte X25519 elliptic curve private key 
/// * `message` - The Vec<u8> message to sign
/// 
/// # Returns
/// 
/// Result<(Vec<u8>, ExternalError)
/// * `Vec<u8>` - The 64 byte signature
/// * `ExternalError` - Error if there is a problem signing message
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// let signature =  crypto::sign_message(priv_bytes, message.as_bytes().to_vec()).unwrap();
/// ```
pub fn sign_message(priv_key: Vec<u8>, message: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    if priv_key.len() != PRIVATE_KEY_LENGTH {
        return Err("Incorrect private key length.");
    }
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

/// Validates a signature using the provided public key. 
/// 
/// A Mazzaroth user's account address can be used as the public key
/// to verify transactions sent from that user.
/// 
/// # Arguments
/// 
/// * `pub_key` - The Vec<u8> 32 byte X25519 elliptic curve public key 
/// * `message` - The Vec<u8> message that was signed
/// * `signature` - The Vec<u8> 64 byte signature
/// 
/// # Returns
/// 
/// * `u32` - 1 if valid, 0 if invalid
/// 
/// # Example
/// 
/// ```
/// use mazzaroth_wasm::crypto;
/// match crypto::validate_signature(pub_bytes, message.as_bytes().to_vec(), sig_bytes) {
///    1 => "Valid".to_string(),
///    _ => "Invalid".to_string(),
/// }
/// ```
pub fn validate_signature(pub_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> u32 {
    if pub_key.len() != PUBLIC_KEY_LENGTH {
        return 0;
    }
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
