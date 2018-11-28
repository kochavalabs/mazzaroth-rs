use super::{AbiType, Decoder, Encoder, Error, Request, Response};
use std::str;

impl AbiType for u32 {
    fn decode(slice: Vec<u8>) -> Result<Self, Error> {
        let result = (slice[0] as u32)
            + ((slice[1] as u32) << 8)
            + ((slice[2] as u32) << 16)
            + ((slice[3] as u32) << 24);

        Ok(result)
    }

    fn encode(self) -> Vec<u8>{
        let mut bytes = [0u8; 4];
        bytes[0] = self as u8;
        bytes[1] = (self >> 8) as u8;
        bytes[2] = (self >> 16) as u8;
        bytes[3] = (self >> 24) as u8;

        bytes.to_vec()
    }
}

impl AbiType for u64 {
    fn decode(slice: Vec<u8>) -> Result<Self, Error> {
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

    fn encode(self) -> Vec<u8> {
        let mut bytes = [0u8; 8];
        bytes[0] = self as u8;
        bytes[1] = (self >> 8) as u8;
        bytes[2] = (self >> 16) as u8;
        bytes[3] = (self >> 24) as u8;
        bytes[4] = (self >> 32) as u8;
        bytes[5] = (self >> 40) as u8;
        bytes[6] = (self >> 48) as u8;
        bytes[7] = (self >> 56) as u8;

        bytes.to_vec()
    }
}

impl AbiType for Vec<u8> {
    fn decode(bytes: Vec<u8>) -> Result<Self, Error> {
        Ok(bytes)
    }

    fn encode(self) -> Vec<u8> {
        self
    }
}

impl AbiType for String {
    fn decode(bytes: Vec<u8>) -> Result<Self, Error> {
        let result =
            str::from_utf8(&bytes)
                .unwrap()
                .to_owned();

        Ok(result)
    }

    fn encode(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl AbiType for Request {
    fn decode(bytes: Vec<u8>) -> Result<Self, Error> {
        let result = Request {
            body: bytes,
        };

        Ok(result)
    }

    fn encode(self) -> Vec<u8> {
        self.body
    }
}

impl AbiType for Response {
    fn decode(bytes: Vec<u8>) -> Result<Self, Error> {
        let result = Response { body: bytes };

        Ok(result)
    }

    fn encode(self) -> Vec<u8> {
        self.body
    }
}
