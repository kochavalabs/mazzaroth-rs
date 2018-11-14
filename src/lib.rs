
mod common;

// Persistence provides get and store functions to use host DB
pub mod persistence;
pub use persistence::{get,store};

// Trait to create contracts
pub mod contract;
pub use contract::{Contract, dispatch};

mod sink;
pub use self::sink::Sink;

mod stream;
pub use stream::Stream;

#[no_mangle]
extern {
	pub fn fetch_input(args: *mut u8);
    pub fn input_length() -> u32;

    pub fn ret(x: &Vec<u8>); // Gets pointer to encoded returns

	pub fn _store(key: *const u8, key_length: usize, value: *const u8, value_length: usize);

	pub fn _get(key: &Vec<u8>, value: &Vec<u8>); // value gets set in this host call

	pub fn get_length(key: &Vec<u8>) -> u32; // returns length needed to return the value from get
}

/// Error for decoding rust types from stream
#[derive(Debug, PartialEq, Eq)] // Allows us to use expect() for these Error types
pub enum Error {
	/// Unexpected end of the stream
	UnexpectedEof,
}

/// Abi type trait
pub trait AbiType : Sized {
	/// Insantiate type from data stream
	/// Should never be called manually! Use stream.pop()
	fn decode(stream: &mut Stream) -> Result<Self, Error>;

	/// Push type to data sink
	/// Should never be called manually! Use sink.push(val)
	fn encode(self, sink: &mut Sink);

	/// Whether type has fixed length or not
	const IS_FIXED: bool;
}

/// Parameters to the call function, specify the handler to call and payload for args
pub struct CallParams {
    pub handler_id: String,
    pub payload: Vec<u8>,
}

// Request and Response types used by Smart Contract funcs
pub struct Request {
	pub handler_id: String,
    pub body: Vec<u8>,
}

pub struct Response {
    pub body: Vec<u8>,
}