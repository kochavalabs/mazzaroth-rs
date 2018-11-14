use super::{AbiType, Error, Sink, Stream, Request, Response};
use std::str;

impl AbiType for u32 {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
		// Returns UnexpectedEof if stream cannot advance
		let previous_position = stream.advance(4)?;

		let slice = &stream.payload()[previous_position..stream.position()];

		let result = (slice[0] as u32) +
			((slice[1] as u32) << 8) +
			((slice[2] as u32) << 16) +
			((slice[3] as u32) << 24);

		Ok(result)
	}

    fn encode(self, sink: &mut Sink) {
        let mut bytes = [0u8; 4];
	    bytes[0] = self as u8;
	    bytes[1] = (self >> 8) as u8;
	    bytes[2] = (self >> 16) as u8;
	    bytes[3] = (self >> 24) as u8;
		sink.values_mut().extend_from_slice(&bytes[..]);
	}

	const IS_FIXED: bool = true;
}

impl AbiType for u64 {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
		// Returns UnexpectedEof if stream cannot advance
		let previous_position = stream.advance(8)?;

		let slice = &stream.payload()[previous_position..stream.position()];

		let result = (slice[0] as u64) +
			((slice[1] as u64) << 8) +
			((slice[2] as u64) << 16) +
			((slice[3] as u64) << 24) +
			((slice[4] as u64) << 32) +
			((slice[5] as u64) << 40) +
			((slice[6] as u64) << 48) +
			((slice[7] as u64) << 56);

		Ok(result)
	}

    fn encode(self, sink: &mut Sink) {

        let mut bytes = [0u8; 8];
	    bytes[0] = self as u8;
	    bytes[1] = (self >> 8) as u8;
	    bytes[2] = (self >> 16) as u8;
	    bytes[3] = (self >> 24) as u8;
    	bytes[4] = (self >> 32) as u8;
	    bytes[5] = (self >> 40) as u8;
    	bytes[6] = (self >> 48) as u8;
	    bytes[7] = (self >> 56) as u8;

		sink.values_mut().extend_from_slice(&bytes[..]);
	}

	const IS_FIXED: bool = true;
}

impl AbiType for Vec<u8> {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
        // First decode the length, then the rest of the payload based on length
		let len = u32::decode(stream)? as usize;

		let result = stream.payload()[stream.position()..stream.position() + len].to_vec();

        // Advance stream pointer past the value
		// Returns UnexpectedEof if stream cannot advance
		stream.advance(len)?;

		Ok(result)
	}
    
    fn encode(self, sink: &mut Sink) {
		let val = self;
		let len = val.len();
		sink.push(len as u32);
		sink.values_mut().extend_from_slice(&val[..]);
	}

	const IS_FIXED: bool = false;
}

impl AbiType for String {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
        // First decode the length, then the rest of the payload based on length
		let len = u32::decode(stream)? as usize;

		let result = str::from_utf8(&stream.payload()[stream.position()..stream.position() + len]).unwrap().to_owned();

        // Advance stream pointer past the value
		// Returns UnexpectedEof if stream cannot advance
		stream.advance(len)?;

		Ok(result)
	}
    
    fn encode(self, sink: &mut Sink) {
		let val = self;
		let len = val.len();
		sink.push(len as u32);
		sink.values_mut().extend_from_slice(&val.into_bytes());
	}

	const IS_FIXED: bool = false;
}

// Request is just made up of a body of Vec<u8> bytes currently for decode and encode
impl AbiType for Request {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
		let body = Vec::decode(stream)?;

		let result = Request { body: body, };

		Ok(result)
	}

	fn encode(self, sink: &mut Sink) {
		let val = self.body;
		let len = val.len();
		sink.push(len as u32);
		sink.values_mut().extend_from_slice(&val[..]);
	}

	const IS_FIXED: bool = false;
}

// Response is just made up of a body of Vec<u8> bytes currently for decode and encode
impl AbiType for Response {
	fn decode(stream: &mut Stream) -> Result<Self, Error> {
		let body = Vec::decode(stream)?;

		let result = Response { body: body, };

		Ok(result)
	}

	fn encode(self, sink: &mut Sink) {
		let val = self.body;
		let len = val.len();
		sink.push(len as u32);
		sink.values_mut().extend_from_slice(&val[..]);
	}
	
	const IS_FIXED: bool = false;
}