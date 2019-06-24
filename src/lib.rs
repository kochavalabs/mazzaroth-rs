#![feature(test)]

/// Defines the Encoder and Decoder used to transmit XDR objects to and from the host VM.
pub mod abi;
pub use abi::decoder::{Decoder, InputDecoder};
pub use abi::encoder::Encoder;

// Contract trait definition
mod contract;
pub use contract::ContractInterface;

/// external modules used to interact with runtime
pub mod external;

#[macro_use]
extern crate cfg_if;

extern crate ex_dee;

extern crate mazzaroth_xdr;