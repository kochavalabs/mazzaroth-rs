#[no_mangle]
extern {
    /// Fetches input from the Runtime.
    /// Parameter input should be the mut pointer to a vector with length and capacity allocated.
    /// Call _input_length first to get a length used to allocate the input vector.
	pub(crate) fn _fetch_input(input: *mut u8);

    /// Returns the length of input from the runtime.
    /// Use the return to set the capacity and length of a vector to call _fetch_input.
    pub(crate) fn _input_length() -> u32;

    /// Returns ptr to bytes to the runtime if a call needs to return a value.
    pub(crate) fn _ret(bytes: *const u8, bytes_length: usize);

    /// Store a key/value in the persistent DB provided by the runtime.
	pub(crate) fn _store(key: *const u8, key_length: usize, value: *const u8, value_length: usize);

    /// Get a Value for key from the persistent DB provided by the runtime.
    /// Parameter value should be the mut pointer to a vector with length and capacity allocated.
    /// Call _get_length first to get a length to allocate the value vector.
	pub(crate) fn _get(key: *const u8, key_length: usize, value: *mut u8);

    /// Returns the length of the value associated with the key from the persistent DB.
    /// Use the return to set the capacity and length of a vector to call _get.
	pub(crate) fn _get_length(key: *const u8, key_length: usize) -> u32;
}