mod r#macro;

use std::collections::HashSet;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let s: HashSet<&str> = set!["hello", "World", "World"];

    println!("Vector: {:?}", v);
    println!("Set: {:?}", s);

    Pancakes::hello_macro();
}
