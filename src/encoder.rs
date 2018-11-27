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
        val.encode(self)
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