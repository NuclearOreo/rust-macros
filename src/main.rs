mod r#macro;

use hello_macro::HelloMacro;

use std::collections::HashSet;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancake");
    }
}

fn main() {
    let set = set![1, 2, 4, 5];
    println!("{:?}", set);

    Pancakes::hello_macro();
}
