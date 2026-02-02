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
            message: greet(ctx, &req.name),
        };

        out
    }
}

#[inline(never)]
pub fn greet(ctx: &Context, name: &str) -> String {
    format!("hello from {} with request-id={}", name, ctx.request_id())
}
