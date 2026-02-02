mod bindings {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindings::exports::sammyne::helloworld::greeter::Guest;
use bindings::sammyne::helloworld::types::{Context, HelloReply, HelloRequest};

struct App;

impl Guest for App {
    fn say_hello(ctx: &Context, req: HelloRequest) -> HelloReply {
        let out = HelloReply {
            message: format!(
                "hello from {} with request-id={}",
                req.name,
                ctx.request_id()
            ),
        };

        out
    }
}
