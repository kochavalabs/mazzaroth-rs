use super::{AbiError, AbiType};

/// Decode a payload of bytes.
/// Values are expected to be encoded AbiTypes that are
/// decoded when popped.
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
    pub fn pop<T: AbiType>(&mut self) -> Result<T, AbiError> {
        // TODO: Check if type is a fixed length, else grab length first
        let len_position = self.advance(4)?;
        let slice = &self.payload[len_position..self.position()];
        let len = u32::decode(slice.to_vec())?;

        // Now grab bytes and advance equal to length
        let bytes_position = self.advance(len as usize)?;
        let bytes = &self.payload[bytes_position..self.position()];

        T::decode(bytes.to_vec())
    }

    /// Current position for the decoder
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance decoder position for `amount` bytes
    pub fn advance(&mut self, amount: usize) -> Result<usize, AbiError> {
        if self.position + amount > self.payload.len() {
            return Err(AbiError::UnexpectedEof);
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
