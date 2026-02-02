mod bindings {
    wit_bindgen::generate!();
}

use bindings::exports::sammyne::app::greeter::Guest;

// Bring the imported add function into scope
use bindings::docs::adder::add;

struct Component;

impl Guest for Component {
    fn greet() {
        println!("hello world :)");
        println!("1 + 1 = {}", add::add(1, 1));
    }
}

bindings::export!(Component with_types_in bindings);
