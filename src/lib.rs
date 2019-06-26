//! # Mazzaroth Rust Library
//! 
//! The Mazzaroth Rust Library is a rust library that includes host bindings 
//! and everything needed to compile rust contracts to Web Assembly, compatible
//! with the Mazzaroth VM.  Here you will find the necessary abi encoders and 
//! decoders used to pass and return arguments to contract functions as well as
//! the external host functions available to use.
//! 
//! ## How to use
//!
//! The first step to using this library is to include the necessary dependencies.
//! The following 3 dependencies should be included in your Cargo.toml:
//! 
//! mazzaroth-wasm
//! mazzaroth-wasm-derive
//! mazzaroth-wasm-xdr
//! 
//! Every contract will have a similar base layout for the main function and the contract trait definition.
//! `main()` is used as the entrypoint and has several important features.  It will instantiate the contract,
//! call a host function to retrieve function input, execute the function, and return a response.
//! 
//! Here is a basic Hello World contract example:
//! ```
//! // must include the ContractInterface and mazzaroth_abi for compiling the macro
//! extern crate mazzaroth_wasm;
//! extern crate mazzaroth_wasm_derive;
//! use mazzaroth_wasm::ContractInterface;
//! use mazzaroth_wasm_derive::mazzaroth_abi;
//! 
//! // using specific external host modules
//! use mazzaroth_wasm::external::{transaction, log};
//! 
//! #[no_mangle]
//! pub fn main() {
//!     // panic hook is set to call the host error log function when a panic occurs
//!     std::panic::set_hook(Box::new(mazzaroth_wasm::external::errors::hook));
//! 
//!     // Creates a new instance of the ABI generated around the Contract
//!     let mut contract = HelloWorld::new(Hello {});
//! 
//!     // Use a host function to get arguments
//!     let args = transaction::arguments();
//! 
//!     // Execute calls one of the functions defined in the contract
//!     // Input for the function to call and it's params comes from the Runtime
//!     let response = contract.execute(&args);
//! 
//!     // Provide return value through host call
//!     transaction::ret(response);
//! }
//! 
//! 
//! #[mazzaroth_abi(HelloWorld)]
//! pub trait HelloWorldContract {
//!     // hello() defined as a readonly function
//!     #[readonly]
//!     fn hello(&mut self) -> u32;
//! }
//! 
//! // Struct used to implement the contract trait
//! pub struct Hello {}
//! 
//! // Actual contract implementation
//! impl HelloWorldContract for Hello {
//!     fn hello(&mut self) -> u32 {
//!         log("Hello World!".to_string());
//!         14
//!     }
//! }
//! ```

#![feature(test)]

/// Defines the Encoder and Decoder used to transmit XDR objects to and from the host VM.
pub mod abi;
pub use abi::decoder::{Decoder, InputDecoder};
pub use abi::encoder::Encoder;

// Contract trait definition
mod contract;
pub use contract::ContractInterface;

/// Defines the external modules used to interact with host runtime.
pub mod external;

#[macro_use]
extern crate cfg_if;

extern crate ex_dee;

extern crate mazzaroth_xdr;