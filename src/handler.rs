use super::{ret,Request,Response};
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
    pub fn handle(&mut self, mut request: Request) -> Response {
        // Call the handler function and return response
        self.function_map[&request.handler_id](&mut request)
    }
}