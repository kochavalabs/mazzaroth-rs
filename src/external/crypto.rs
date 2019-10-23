//! Provides a set of cryptographic functions for use in contracts.
#[cfg(not(feature = "host-mock"))]
use super::externs::{
    _keccak256, _sha256, _sha3_256, _sha3_512, _shake256, _validate_signature, PUBLIC_KEY_LENGTH,
};

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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// let hash = crypto::sha256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

#[cfg(feature = "host-mock")]
pub fn sha256(__data: Vec<u8>) -> Vec<u8> {
    vec![]
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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// let hash = crypto::sha3_256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn sha3_256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _sha3_256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

#[cfg(feature = "host-mock")]
pub fn sha3_256(_data: Vec<u8>) -> Vec<u8> {
    vec![]
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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// let hash = crypto::sha3_512(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn sha3_512(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(64 as usize); // 64 byte (512) hash
    unsafe { hash.set_len(64 as usize) };

    unsafe { _sha3_512(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

#[cfg(feature = "host-mock")]
pub fn sha3_512(_data: Vec<u8>) -> Vec<u8> {
    vec![]
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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// let hash = crypto::keccak256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn keccak256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _keccak256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

#[cfg(feature = "host-mock")]
pub fn keccak256(_data: Vec<u8>) -> Vec<u8> {
    vec![]
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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// let hash = crypto::shake256(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
#[cfg(not(feature = "host-mock"))]
pub fn shake256(data: Vec<u8>) -> Vec<u8> {
    // Create Vec to store hash
    let mut hash = Vec::with_capacity(32 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(32 as usize) };

    unsafe { _shake256(data.as_ptr(), data.len(), hash.as_mut_ptr()) };

    hash
}

#[cfg(feature = "host-mock")]
pub fn shake256(_data: Vec<u8>) -> Vec<u8> {
    vec![]
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
/// ```ignore
/// use mazzaroth_rs::external::crypto;
/// match crypto::validate_signature(pub_bytes, message.as_bytes().to_vec(), sig_bytes) {
///    1 => "Valid".to_string(),
///    _ => "Invalid".to_string(),
/// }
/// ```
#[cfg(not(feature = "host-mock"))]
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

#[cfg(feature = "host-mock")]
pub fn validate_signature(_pub_key: Vec<u8>, _message: Vec<u8>, _signature: Vec<u8>) -> u32 {
    0
}

#[cfg(test)]
#[cfg(feature = "host-mock")]
mod tests {

    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(0, validate_signature(vec![], vec![], vec![]));
    }

    #[test]
    fn test_hashes() {
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, sha256(vec![]));
        assert_eq!(expected, sha3_256(vec![]));
        assert_eq!(expected, sha3_512(vec![]));
        assert_eq!(expected, keccak256(vec![]));
        assert_eq!(expected, shake256(vec![]));
    }
}
