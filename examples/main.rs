use std::sync::{Arc, Mutex};
use clone_move::clone_move;

fn main() {
    let v1 = Arc::new(String::from("hello"));
    let v2 = Arc::new(Mutex::new(String::from("hello")));

    clone_move!([v1, v2] |a: i32| {
        println!("v1 = {:?}", *v1);
        println!("v2 = {:?}", *v2);
        println!("a = {:?}", a);
    } )(3);
    println!("v1 = {:?}", *v1);
    println!("v2 = {:?}", *v2);
}
