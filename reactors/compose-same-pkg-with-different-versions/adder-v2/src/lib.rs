mod bindings {
    wit_bindgen::generate!();
}

use bindings::exports::docs::adder0_2_0::add::Guest;

// Bring the imported add function into scope
use bindings::docs::adder0_1_0::add;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        add::add(a, b) + 100
    }
}

bindings::export!(Component with_types_in bindings);
