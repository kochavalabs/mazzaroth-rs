
use super::{fetch_input, input_length, Request, Response, Stream};

pub trait Contract {
    fn execute(&mut self, Request) -> Response;
}

pub fn dispatch(mut contract: Box<Contract>) -> Response {

    // Get Request from runtime
    let length = unsafe { input_length() };
    let mut input: Vec<u8> = Vec::with_capacity(length as usize);
    unsafe {
        input.set_len(length as usize);
        fetch_input(input.as_mut_ptr());
    }

    let mut stream = Stream::new(&input);
    let request = stream.pop().expect("request decoding failed");

    contract.execute(request)
}