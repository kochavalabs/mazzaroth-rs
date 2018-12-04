/// The native rust Crypto functions

extern crate test;
extern crate sha3;
use self::sha3::{Sha3_256, Digest};

/// Returns the sha256 hash applied to the supplied data
pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    let hash = Sha3_256::digest(&data);

    hash.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    #[bench]
    fn bench_sha256(b: &mut Bencher) {
        b.iter(|| sha256(vec![1, 2, 3]));
    }
}