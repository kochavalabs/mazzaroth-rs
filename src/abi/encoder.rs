//! Encodes XDR objects into a byte slice.

use xdr_rs_serialize::ser::XDROut;

/// Encoder for returning a number of arguments.
/// To push a value to the encoder it must implement the Serialize trait for
/// encoding.
pub struct Encoder {
    values: Vec<u8>,
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder { values: Vec::new() }
    }
}

impl Encoder {
    /// Consume `val` to the Encoder
    pub fn push<T: XDROut>(&mut self, val: T) {
        let mut val_bytes: Vec<u8> = Vec::new();
        val.write_json(&mut val_bytes).unwrap();

        // Append bytes after the length
        self.values_mut().extend_from_slice(&val_bytes);
    }

    /// Mutable reference to the Encoder vector
    pub fn values_mut(&mut self) -> &mut Vec<u8> {
        &mut self.values
    }

    /// return the vector of values
    pub fn values(self) -> Vec<u8> {
        self.values
    }
}
