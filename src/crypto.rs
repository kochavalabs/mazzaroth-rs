extern crate sha3;
use self::sha3::{Sha3_256, Digest};

/// Returns the sha256 hash applied to the supplied data
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    let hash = Sha3_256::digest(&data);

    hash.to_vec()
}