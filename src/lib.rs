// Define the custom AbiType implementations
mod common;

// Persistence provides get and store functions to use host DB
pub mod persistence;
pub use persistence::{get, store};

// Contract trait and dispatch function for interacting with smart contracts.
pub mod contract;
pub use contract::{Arguments, ContractInterface};

// Encoder and Decoder used for passing data between runtime
// Can be used to create a custom AbiType.
mod encoder;
pub use encoder::Encoder;
mod decoder;
pub use decoder::Decoder;

// externals are used in this crate to interact with runtime
mod external;
pub(crate) use external::*;

/// Error for decoding rust types from decoder
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Unexpected end of the decoder
    UnexpectedEof,
}

/// AbiType trait
pub trait AbiType: Sized {
    /// Define how objects should be decoded.
    fn decode(decoder: &mut Decoder) -> Result<Self, Error>;

    /// Define how objects should be encoded.
    fn encode(self, encoder: &mut Encoder);
}

// Request and Response types used by Smart Contract funcs
pub struct Request {
    pub handler_id: String,
    pub body: Vec<u8>,
}

pub struct Response {
    pub body: Vec<u8>,
}

/// Return a response to the runtime
pub fn ret(response: Response) {
    // encode the Response and send as bytes
    let mut encoder = Encoder::new();

    encoder.push(response);

    let values = encoder.values();

    unsafe { _ret(values.as_ptr(), values.len()) };
}