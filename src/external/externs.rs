#[no_mangle]
extern "C" {
    /// Fetches input from the Runtime.
    /// Parameter input should be the mut pointer to a vector with length and capacity allocated.
    /// Call _input_length first to get a length used to allocate the input vector.
    pub(crate) fn _fetch_input(input: *mut u8);

    /// Returns the length of input from the runtime.
    /// Use the return to set the capacity and length of a vector to call _fetch_input.
    pub(crate) fn _input_length() -> u32;

    /// Fetches sender from the Runtime.
    /// Parameter sender should be the mut pointer to a vector with length and capacity allocated.
    /// Call _input_length first to get a length used to allocate the input vector.
    pub(crate) fn _fetch_sender(sender: *mut u8);

    /// Returns the length of sender from the runtime.
    /// Use the return to set the capacity and length of a vector to call _fetch_sender.
    pub(crate) fn _sender_length() -> u32;

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

    /// Host hashing function: sha256
    pub(crate) fn _sha256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: sha3_256
    pub(crate) fn _sha3_256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: keccak256
    pub(crate) fn _keccak256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: shake256
    pub(crate) fn _shake256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Return error messages to the host runtime
    pub(crate) fn _log_error(msg: String);

    pub(crate) fn _log(msg: String);
}
