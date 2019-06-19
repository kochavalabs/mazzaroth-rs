use ex_dee::de::XDRIn;
use ex_dee::error::Error;
use mazzaroth_xdr::Parameter;

/// Decode a payload of bytes.
/// Values are expected to be implement Deserialize to be properly popped.
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
    pub fn pop<T: XDRIn<&'a [u8]>>(&mut self) -> Result<T, Error> {
        // grab bytes from parameter and advance 1
        let mut bytes = &self.payload[self.position].t[..];
        self.position += 1;

        Ok(T::read_xdr(&mut bytes)?.0)
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
