use super::{Decoder, Request, Response, _fetch_input, _input_length};

/// This trait defines a function that will be called on a contract.
/// It is up to the contract to setup handlers and call the appropriate
/// function based on the Request params.
pub trait ContractInterface {
    fn execute(&mut self, payload: &[u8]) -> Vec<u8>;
}

/*
/// Calls the execute function on a Contract with Request param pulled
/// from the runtime and returns the response.
/// TODO: Expand this to not need an execute function and to work with
/// various parameters/returns of contract functions.
pub fn dispatch(mut contract: Box<ContractInterface>) -> Response {
    // Get Request from runtime
    let length = unsafe { _input_length() };
    let mut input: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        input.set_len(length as usize);
        _fetch_input(input.as_mut_ptr());
    }

    let mut decoder = Decoder::new(&input);
    let request = decoder.pop().expect("request decoding failed");

    contract.execute(request)
}
*/

/// Get the arguments encoded from the Runtime
pub fn Arguments() -> Vec<u8> {
    let length = unsafe { _input_length() };
    let mut args: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        args.set_len(length as usize);
        _fetch_input(args.as_mut_ptr());
    }

    args
}
