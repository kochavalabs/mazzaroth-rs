pub(crate) const PUBLIC_KEY_LENGTH: usize = 32;

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

    /// Returns ptr to bytes to the runtime if a call needs to return a value.
    pub(crate) fn _ret(bytes: *const u8, bytes_length: usize);

    /// Store a key/value in the persistent DB provided by the runtime.
    pub(crate) fn _store(key: *const u8, key_length: usize, value: *const u8, value_length: usize);

    /// Delete a key/value in the persistent DB provided by the runtime.
    pub(crate) fn _delete(key: *const u8, key_length: usize);

    /// Get a Value for key from the persistent DB provided by the runtime.
    /// Parameter value should be the mut pointer to a vector with length and capacity allocated.
    /// Call _get_length first to get a length to allocate the value vector.
    pub(crate) fn _get(key: *const u8, key_length: usize, value: *mut u8);

    /// Returns the length of the value associated with the key from the persistent DB.
    /// Use the return to set the capacity and length of a vector to call _get.
    pub(crate) fn _get_length(key: *const u8, key_length: usize) -> u32;

    /// Returns if the key exists in the persistent DB.
    pub(crate) fn _key_exists(key: *const u8, key_length: usize) -> bool;

    /// Get an account name from the persistent DB provided by the runtime.
    /// Parameter value should be the mut pointer to a vector with length and capacity allocated.
    /// Call _get_account_name_length first to get a length to allocate the value vector.
    pub(crate) fn _get_account_name(key: *const u8, key_length: usize, value: *mut u8);

    /// Set an account name in the persistent DB provided by the runtime.
    pub(crate) fn _set_account_name(
        key: *const u8,
        key_length: usize,
        value: *const u8,
        value_length: usize,
    );

    /// Returns the length of the name associated with the account key from the persistent DB.
    /// Use the return to set the capacity and length of a vector to call _get_account_name.
    pub(crate) fn _get_account_name_length(key: *const u8, key_length: usize) -> u32;

    /// Check if a particular account is the owner of the contract.
    pub(crate) fn _is_owner(key: *const u8, key_length: usize) -> bool;

    /// Host hashing function: sha256
    pub(crate) fn _sha256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: sha3_256
    pub(crate) fn _sha3_256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: sha3_512
    pub(crate) fn _sha3_512(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: keccak256
    pub(crate) fn _keccak256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function: shake256
    pub(crate) fn _shake256(data: *const u8, data_length: usize, hash: *mut u8);

    /// Host hashing function for generating a cryptographic key pair.
    /// Currently returns a X25519 elliptic curve key pair, 32 byte private key
    /// and 32 byte public key
    pub(crate) fn _generate_key_pair(priv_key: *mut u8, pub_key: *mut u8);

    /// Signs a message using the provided private key. You typically wouldn't be
    /// signing something by sending your private key to the network, so this is
    /// mostly for demonstration purposes.
    /// It uses a 32 byte X25519 elliptic curve private key and returns a 64 byte
    /// signature.
    pub(crate) fn _sign_message(
        priv_key: *const u8,
        message: *const u8,
        message_length: usize,
        signature: *mut u8,
    );

    /// Validates a signature using the provided public key. A Mazzaroth user's
    /// account address can be used as the public key to verify transactions sent
    /// from that user.
    /// We are currently using a 32 byte X25519 elliptic curve public key and a 64
    /// byte signature string.
    pub(crate) fn _validate_signature(
        pub_key: *const u8,
        message: *const u8,
        message_length: usize,
        signature: *const u8,
    ) -> u32;

    /// Return error messages to the host runtime
    pub(crate) fn _log_error(msg: *const u8, msg_length: usize);

    /// Log message to host runtime
    pub(crate) fn _log(msg: *const u8, msg_length: usize);

    /// Inserts a value into a keyquery managed database.
    /// Should be a serialized keyquery.xdr.Insert
    pub(crate) fn _kq_insert(insert: *const u8, insert_length: usize);

    /// Queries and returns its length and a 16 byte hash to look fetch the
    /// result with by running _kq_query_fetch
    pub(crate) fn _kq_query_run(query: *const u8, query_length: usize, hash: *const u8) -> u32;

    /// Fetches the results of a _kq_query_run execution 
    pub(crate) fn _kq_query_fetch(result: *const u8, hash: *const u8);
}
