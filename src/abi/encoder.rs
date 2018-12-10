use super::AbiType;

/// Encoder for returning a number of arguments.
/// To push a value to the encoder it must implement
/// the AbiType trait for encoding.
pub struct Encoder {
    values: Vec<u8>,
}

impl Encoder {
    /// New encoder that will grow as items are pushed
    pub fn new() -> Self {
        Encoder { values: Vec::new() }
    }

    /// Consume `val` to the Encoder
    pub fn push<T: AbiType>(&mut self, val: T) {
        let bytes = val.encode();
        // Push a u32 Length for each encoded item
        // TODO: Check for fixed length items and don't include this (u32, u64, etc.)
        let len = bytes.len() as u32;
        self.values_mut().extend_from_slice(&len.encode());

        // Append bytes after the length
        self.values_mut().extend_from_slice(&bytes[..]);
    }

    /// Mutable reference to the Encoder vector
    pub fn values_mut(&mut self) -> &mut Vec<u8> {
        &mut self.values
    }

    /// return the vector of values
    pub fn values(self) -> Vec<u8> {
        let result = self.values;
        result
    }
}
