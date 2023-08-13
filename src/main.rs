mod r#macro;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

use std::collections::HashSet;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let set = set![1, 2, 4, 5];
    println!("{:?}", set);

    Pancakes::hello_macro();
}
