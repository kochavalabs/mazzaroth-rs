#![feature(test)]

// Define the custom AbiType implementations
pub mod abi;

// Contract trait definition
mod contract;
pub use contract::ContractInterface;

// Crypto for Sha3 256 hashing
pub mod crypto;

// Encoder and Decoder used for passing data between runtime
// Can be used to create a custom AbiType.
pub use abi::decoder::Decoder;
pub use abi::encoder::Encoder;

// externals are used in this crate to interact with runtime
pub mod external;

#[macro_use]
extern crate cfg_if;

extern crate rust_xdr;
extern crate serde;
