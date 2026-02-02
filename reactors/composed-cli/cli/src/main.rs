mod bindgen {
    wit_bindgen::generate!({
        world: "helloworld-lib",
        path: "../guest/wit",
    });
}

use crate::bindgen::sammyne::helloworld::greeter::{self, HelloRequest};

fn main() {
    let request = HelloRequest {
        name: "sammyne".to_owned(),
    };

    let reply = greeter::say_hello(&request);
    println!("reply: {}", reply.message);
}
