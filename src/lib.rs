#![feature(test)]

// Define the custom AbiType implementations
mod common;

// Contract trait definition
mod contract;
pub use contract::ContractInterface;

// Utils for converting values
pub mod utils;

// Crypto for Sha3 256 hashing
pub mod crypto;

// Encoder and Decoder used for passing data between runtime
// Can be used to create a custom AbiType.
mod encoder;
pub use encoder::Encoder;
mod decoder;
pub use decoder::Decoder;

// externals are used in this crate to interact with runtime
pub mod external;

/// Error for decoding rust types from decoder
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Unexpected end of the decoder
    UnexpectedEof,
}

/// AbiType trait
pub trait AbiType: Sized {
    /// Define how objects should be decoded.
    fn decode(bytes: Vec<u8>) -> Result<Self, Error>;

    /// Define how objects should be encoded.
    fn encode(self) -> Vec<u8>;
}