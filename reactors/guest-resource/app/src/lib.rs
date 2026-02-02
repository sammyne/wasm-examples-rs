mod bindings {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use crate::bindings::exports::helloworld::example::greeter_api::{self, GuestGreeter};
use bindings::exports::helloworld::example::api::Guest;

struct App;

struct Greeter;

impl Guest for App {
    fn hi(who: String) {
        let g = greeter_api::Greeter::new(Greeter::new());
        Self::hello_world(g, who);
    }

    fn hello_world(g: greeter_api::Greeter, who: String) {
        g.get::<Greeter>().greet(who);
    }
}

impl greeter_api::Guest for App {
    type Greeter = Greeter;
}

impl GuestGreeter for Greeter {
    fn new() -> Self {
        Greeter
    }

    fn greet(&self, who: String) {
        println!("hello {who}");
    }
}
