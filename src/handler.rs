use super::{fetch_handler,handler_length,fetch_payload,payload_length,ret,Request,Response};
use std::collections::HashMap;

// Map u32 to functions
// TODO: Use String to map and change function signature
pub struct Handler {
    function_map: HashMap<String, fn(&mut Request) -> Response>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { function_map: HashMap::new() }
    }

    pub fn add(&mut self, handler_id: String, function: fn(&mut Request) -> Response) {
        self.function_map.insert(handler_id, function);
    }

    // handle gets the payload and id from the host params
    pub fn handle(&mut self) {
        let params = HandlerParams::read();

        // Call the handler function and return response
        let response = self.function_map[&params.handler_id](&mut Request{body: params.payload});

        // Call host to return response
        unsafe {ret(&response.body)}
    }
}

// TODO: Read 2 args to call, handler id and payload instead of parsing handler_id from the payload and use String instead of u32
pub struct HandlerParams {
    pub handler_id: String,
    pub payload: Vec<u8>,
}

impl HandlerParams {
    // TODO: Return Result to include error if problem reading params
    pub fn read() -> HandlerParams {
        // Fetch the input handler id first
        let length = unsafe { handler_length() };
        let mut input: Vec<u8> = Vec::with_capacity(length as usize);
        unsafe {
            input.set_len(length as usize);
            fetch_handler(input.as_mut_ptr());
        }

        let id = String::from_utf8(input).expect("error reading handler id");

        // Fetch the input payload next
        let length = unsafe { payload_length() };
        let mut payload: Vec<u8> = Vec::with_capacity(length as usize);
        unsafe {
            payload.set_len(length as usize);
            fetch_payload(payload.as_mut_ptr());
        }

        HandlerParams {
            handler_id: id,
            payload: payload,
        }
    }
}