#[no_mangle]
extern {
	pub fn fetch_input(args: *mut u8);
    pub fn input_length() -> u32;

    pub fn ret(x: &Vec<u8>); // Gets pointer to encoded returns

	pub fn _store(key: *const u8, key_length: usize, value: *const u8, value_length: usize);

	pub fn _get(key: &Vec<u8>, value: &Vec<u8>); // value gets set in this host call

	pub fn get_length(key: &Vec<u8>) -> u32; // returns length needed to return the value from get
}