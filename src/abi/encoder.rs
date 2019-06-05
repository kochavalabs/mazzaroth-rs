use ex_dee::ser::XDROut;

/// Encoder for returning a number of arguments.
/// To push a value to the encoder it must implement the Serialize trait for
/// encoding.
pub struct Encoder {
    values: Vec<u8>,
}

impl Encoder {
    /// New encoder that will grow as items are pushed
    pub fn new() -> Self {
        Encoder { values: Vec::new() }
    }

    /// Consume `val` to the Encoder
    pub fn push<T: XDROut<Vec<u8>>>(&mut self, val: T) {
        let mut val_bytes: Vec<u8> = Vec::new();
        val.write_xdr(&mut val_bytes).unwrap();
        // Push a u32 Length for each encoded item
        // TODO: Check for fixed length items and don't include this (u32, u64, etc.)
        let len = val_bytes.len() as u32;
        let mut len_bytes: Vec<u8> = Vec::new();
        len.write_xdr(&mut len_bytes).unwrap();
        self.values_mut().extend_from_slice(&len_bytes);

        // Append bytes after the length
        self.values_mut().extend_from_slice(&val_bytes);
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
