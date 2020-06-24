//! Decodes encoded bytes into an XDR object.

use mazzaroth_xdr::Parameter;
use xdr_rs_serialize::de::{read_json_string, XDRIn};
use xdr_rs_serialize::error::Error;

/// Decode a single payload of bytes into an XDR object.
/// Value must implement XDRIn.
pub struct Decoder<'a> {
    payload: &'a str,
}

impl<'a> Decoder<'a> {
    /// New decoder for known payload
    pub fn new(raw: &'a str) -> Self {
        Decoder { payload: raw }
    }

    /// Pop next argument of known type
    pub fn pop<T: XDRIn>(&mut self) -> Result<T, Error> {
        let bytes = &self.payload[..];
        read_json_string(bytes.to_string())
    }
}

/// Decode a vector of Parameters into separate XDR object.
/// Values must implement XDRIn.
pub struct InputDecoder<'a> {
    payload: &'a Vec<Parameter>,
    position: usize,
}

impl<'a> InputDecoder<'a> {
    /// New decoder for known payload
    pub fn new(raw: &'a Vec<Parameter>) -> Self {
        InputDecoder {
            payload: raw,
            position: 0,
        }
    }

    /// Pop next argument of known type
    pub fn pop<T: XDRIn>(&mut self) -> Result<T, Error> {
        // grab bytes from parameter and advance 1
        let bytes = &self.payload[self.position].t[..];
        self.position += 1;

        read_json_string(bytes.to_string())
    }

    /// Current position for the decoder
    pub fn position(&self) -> usize {
        self.position
    }

    /// Decoder payload
    pub fn payload(&self) -> &'a Vec<Parameter> {
        self.payload
    }
}
