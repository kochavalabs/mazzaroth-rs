#![feature(test)]

/// Defines the Encoder and Decoder used to transmit XDR objects to and from the host VM.
pub mod abi;

// Contract trait definition
mod contract;
pub use contract::ContractInterface;

// Encoder and Decoder used for passing data between runtime
// Can be used to create a custom AbiType.
pub use abi::decoder::Decoder;
pub use abi::decoder::InputDecoder;
pub use abi::encoder::Encoder;


// externals are used in this crate to interact with runtime
pub mod external;

#[macro_use]
extern crate cfg_if;

extern crate ex_dee;

extern crate mazzaroth_xdr;