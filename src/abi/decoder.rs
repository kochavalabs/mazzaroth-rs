use ex_dee::de::XDRIn;
use ex_dee::error::Error;

/// Decode a payload of bytes.
/// Values are expected to be implement Deserialize to be properly popped.
pub struct Decoder<'a> {
    payload: &'a [u8],
    position: usize,
}

impl<'a> Decoder<'a> {
    /// New decoder for known payload
    pub fn new(raw: &'a [u8]) -> Self {
        Decoder {
            payload: raw,
            position: 0,
        }
    }

    /// Pop next argument of known type
    pub fn pop<T: XDRIn<&'a [u8]>>(&mut self) -> Result<T, Error> {
        let mut bytes = &self.payload[..];

        Ok(T::read_xdr(&mut bytes)?.0)
    }

    /// Current position for the decoder
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance decoder position for `amount` bytes
    pub fn advance(&mut self, amount: usize) -> Result<usize, Error> {
        if self.position + amount > self.payload.len() {
            return Err(Error::UnknownError);
        }

        let old_position = self.position;
        self.position += amount;
        Ok(old_position)
    }

    /// Decoder payload
    pub fn payload(&self) -> &[u8] {
        self.payload
    }
}
