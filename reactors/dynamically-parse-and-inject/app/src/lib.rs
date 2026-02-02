mod bindings {
    wit_bindgen::generate!();
}

use bindings::sammyne::app::greeter;
use bindings::sammyne::calculator::ops;
use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        const WHO: &str = "sammyne";

        greeter::greet(WHO);

        let sum = ops::add(1, 1);
        println!("1 + 1 = {sum}");

        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
