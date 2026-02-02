mod bindings {
    wit_bindgen::generate!({});

    use crate::App;

    export!(App);
}

use bindings::exports::docs::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::docs::adder::add;

struct App;

impl Guest for App {
    fn eval_expression(_expr: String) -> u32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add::add(123, 456)
    }
}
