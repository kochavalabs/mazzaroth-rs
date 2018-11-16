use super::{AbiType, Error};

/// Stream interpretation of incoming payload.
/// Values are expected to be encoded AbiTypes that are
/// decoded when popped.
pub struct Stream<'a> {
    payload: &'a [u8],
    position: usize,
}

impl<'a> Stream<'a> {
    /// New stream for known payload
    pub fn new(raw: &'a [u8]) -> Self {
        Stream {
            payload: raw,
            position: 0,
        }
    }

    /// Pop next argument of known type
    pub fn pop<T: AbiType>(&mut self) -> Result<T, Error> {
        T::decode(self)
    }

    /// Current position for the stream
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance stream position for `amount` bytes
    pub fn advance(&mut self, amount: usize) -> Result<usize, Error> {
        if self.position + amount > self.payload.len() {
            return Err(Error::UnexpectedEof);
        }

        let old_position = self.position;
        self.position += amount;
        Ok(old_position)
    }

    /// Stream payload
    pub fn payload(&self) -> &[u8] {
        self.payload
    }
}
