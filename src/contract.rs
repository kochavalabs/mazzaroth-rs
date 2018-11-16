use super::{_fetch_input, _input_length, Request, Response, Stream};

/// This trait defines a function that will be called on a contract.
/// It is up to the contract to setup handlers and call the appropriate
/// function based on the Request params.
pub trait Contract {
    fn execute(&mut self, Request) -> Response;
}

/// Calls the execute function on a Contract with Request param pulled
/// from the runtime and returns the response.
/// TODO: Expand this to not need an execute function and to work with
/// various parameters/returns of contract functions.
pub fn dispatch(mut contract: Box<Contract>) -> Response {

    // Get Request from runtime
    let length = unsafe { _input_length() };
    let mut input: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        input.set_len(length as usize);
        _fetch_input(input.as_mut_ptr());
    }

    let mut stream = Stream::new(&input);
    let request = stream.pop().expect("request decoding failed");

    contract.execute(request)
}