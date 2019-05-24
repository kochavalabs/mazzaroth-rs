use rust_xdr::de::from_bytes;
use rust_xdr::error::Error;
use serde::de::Deserialize;

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
    pub fn pop<T: Deserialize<'a>>(&mut self) -> Result<T, Error> {
        // TODO: Check if type is a fixed length, else grab length first
        let len_position = self.advance(4)?;
        let slice = &self.payload[len_position..self.position()];
        let len = from_bytes::<u32>(slice)?;

        // Now grab bytes and advance equal to length
        let bytes_position = self.advance(len as usize)?;
        let bytes = &self.payload[bytes_position..self.position()];

        from_bytes(bytes)
    }

    /// Current position for the decoder
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance decoder position for `amount` bytes
    pub fn advance(&mut self, amount: usize) -> Result<usize, Error> {
        if self.position + amount > self.payload.len() {
            return Err(Error::TrailingCharacters);
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
