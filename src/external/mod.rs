pub mod externs;

pub mod persistence;

pub mod crypto;

/// Get the arguments encoded from the Runtime
pub fn arguments() -> Vec<u8> {
    let length = unsafe { externs::_input_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        externs::_fetch_input(args.as_mut_ptr());
    }

    args
}

/// Return encoded bytes to the runtime
pub fn ret(values: Vec<u8>) {
    unsafe { externs::_ret(values.as_ptr(), values.len()) };
}
