
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

// externals are used in this crate to interact with runtime
mod external;
pub(crate) use external::*;

/// Error for decoding rust types from stream
#[derive(Debug, PartialEq, Eq)]
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
	let mut sink = Sink::new();

	sink.push(response);

	unsafe {_ret(&sink.values())};
}