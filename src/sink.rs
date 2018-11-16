use super::AbiType;

/// Sink for returning a number of arguments.
/// To push a value to the sink it must implement
/// the AbiType trait for encoding.
pub struct Sink {
	values: Vec<u8>,
}

impl Sink {
	/// New sink that will grow as items are pushed
	pub fn new() -> Self {
		Sink {
			values: Vec::new(),
		}
	}

	/// Consume `val` to the Sink
	pub fn push<T: AbiType>(&mut self, val: T) {
        val.encode(self)
	}

	/// Mutable reference to the Sink vector
	pub fn values_mut(&mut self) -> &mut Vec<u8> {
		&mut self.values
	}

    /// return the vector of values
	pub fn values(self) -> Vec<u8> {
        let result = self.values;
		result
	}
}