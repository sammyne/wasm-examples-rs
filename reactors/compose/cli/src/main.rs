mod bindings {
    wit_bindgen::generate!({});
}

use bindings::docs::adder::add;
use bindings::docs::calculator::calculate;

fn main() {
    let result = calculate::eval_expression("1 + 1");
    println!("1 + 1 = {result}");

    let sum = add::add(1, 1);
    println!("1 + 1 = {sum}");
}
