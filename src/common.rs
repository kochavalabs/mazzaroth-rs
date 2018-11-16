use super::{AbiType, Decoder, Encoder, Error, Request, Response};
use std::str;

impl AbiType for u32 {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        // Returns UnexpectedEof if decoder cannot advance
        let previous_position = decoder.advance(4)?;

        let slice = &decoder.payload()[previous_position..decoder.position()];

        let result = (slice[0] as u32)
            + ((slice[1] as u32) << 8)
            + ((slice[2] as u32) << 16)
            + ((slice[3] as u32) << 24);

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        let mut bytes = [0u8; 4];
        bytes[0] = self as u8;
        bytes[1] = (self >> 8) as u8;
        bytes[2] = (self >> 16) as u8;
        bytes[3] = (self >> 24) as u8;
        encoder.values_mut().extend_from_slice(&bytes[..]);
    }
}

impl AbiType for u64 {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        // Returns UnexpectedEof if decoder cannot advance
        let previous_position = decoder.advance(8)?;

        let slice = &decoder.payload()[previous_position..decoder.position()];

        let result = (slice[0] as u64)
            + ((slice[1] as u64) << 8)
            + ((slice[2] as u64) << 16)
            + ((slice[3] as u64) << 24)
            + ((slice[4] as u64) << 32)
            + ((slice[5] as u64) << 40)
            + ((slice[6] as u64) << 48)
            + ((slice[7] as u64) << 56);

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        let mut bytes = [0u8; 8];
        bytes[0] = self as u8;
        bytes[1] = (self >> 8) as u8;
        bytes[2] = (self >> 16) as u8;
        bytes[3] = (self >> 24) as u8;
        bytes[4] = (self >> 32) as u8;
        bytes[5] = (self >> 40) as u8;
        bytes[6] = (self >> 48) as u8;
        bytes[7] = (self >> 56) as u8;

        encoder.values_mut().extend_from_slice(&bytes[..]);
    }
}

impl AbiType for Vec<u8> {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        // First decode the length, then the rest of the payload based on length
        let len = u32::decode(decoder)? as usize;

        let result = decoder.payload()[decoder.position()..decoder.position() + len].to_vec();

        // Advance decoder pointer past the value
        // Returns UnexpectedEof if decoder cannot advance
        decoder.advance(len)?;

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        let val = self;
        let len = val.len();
        encoder.push(len as u32);
        encoder.values_mut().extend_from_slice(&val[..]);
    }
}

impl AbiType for String {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        // First decode the length, then the rest of the payload based on length
        let len = u32::decode(decoder)? as usize;

        let result =
            str::from_utf8(&decoder.payload()[decoder.position()..decoder.position() + len])
                .unwrap()
                .to_owned();

        // Advance decoder pointer past the value
        // Returns UnexpectedEof if decoder cannot advance
        decoder.advance(len)?;

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        let val = self;
        let len = val.len();
        encoder.push(len as u32);
        encoder.values_mut().extend_from_slice(&val.into_bytes());
    }
}

impl AbiType for Request {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        let handler_id = String::decode(decoder)?;
        let body = Vec::decode(decoder)?;

        let result = Request {
            handler_id: handler_id,
            body: body,
        };

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        // Push handler_id (Strinng) first
        encoder.push(self.handler_id);

        // Push body (Vec<u8>) as second value
        encoder.push(self.body);
    }
}

impl AbiType for Response {
    fn decode(decoder: &mut Decoder) -> Result<Self, Error> {
        let body = Vec::decode(decoder)?;

        let result = Response { body: body };

        Ok(result)
    }

    fn encode(self, encoder: &mut Encoder) {
        // Push body (Vec<u8>) as only value
        encoder.push(self.body);
    }
}
