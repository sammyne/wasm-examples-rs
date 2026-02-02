mod bindings {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindings::exports::sammyne::helloworld::greeter::Guest;
use bindings::sammyne::helloworld::types::{HelloReply, HelloRequest};

struct App;

impl Guest for App {
    fn say_hello(req: HelloRequest) -> HelloReply {
        let out = HelloReply {
            message: format!("hello from {}", req.name,),
        };

        out
    }
}
