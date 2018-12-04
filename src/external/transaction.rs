use super::externs::{_input_length, _fetch_input, _ret, _sender_length, _fetch_sender};

/// Get the arguments encoded from the Runtime
pub fn arguments() -> Vec<u8> {
    let length = unsafe { _input_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        _fetch_input(args.as_mut_ptr());
    }

    args
}

/// Return encoded bytes to the runtime
pub fn ret(values: Vec<u8>) {
    unsafe { _ret(values.as_ptr(), values.len()) };
}

pub fn sender() -> Vec<u8> {
    let length = unsafe { _sender_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        _fetch_sender(args.as_mut_ptr());
    }

    args
}