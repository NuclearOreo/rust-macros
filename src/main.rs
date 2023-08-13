mod r#macro;
use std::collections::HashSet;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
    let set = set![1, 2, 4, 5];
    println!("{:?}, {:?}", set, v);
}
