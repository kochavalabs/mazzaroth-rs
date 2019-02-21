extern crate sha3;
/// The native rust Crypto functions
extern crate test;
use self::sha3::{Digest, Sha3_256};

/// Returns the sha256 hash applied to the supplied data
/// Currently faster to use host implementation
/// Needs optimaztion on VM side
/*
fn sha256(data: Vec<u8>) -> Vec<u8> {
    let hash = Sha3_256::digest(&data);

    hash.to_vec()
}
*/

#[cfg(test)]
mod tests {
    use self::test::Bencher;
    use super::*;

    #[bench]
    fn bench_sha256(b: &mut Bencher) {
        b.iter(|| sha256(vec![1, 2, 3]));
    }
}
