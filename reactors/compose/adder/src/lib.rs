mod bindings {
    wit_bindgen::generate!({});

    use crate::App;

    export!(App);
}

// Separating out the interface puts it in a sub-module
use bindings::exports::docs::adder::add::Guest;

struct App;

impl Guest for App {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
