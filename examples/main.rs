use clone_move::*;
use std::sync::{Arc, Mutex};

fn func(_: &String) {}
async fn async_func(_: &String) {}

fn main() {
    let v1 = Arc::new(String::from("v1"));
    let v2 = Arc::new(Mutex::new(String::from("v2")));
    let _ = clone_move!([v1, v2] |_: i32| {
        func(&v1);
        func(&*v2.lock().unwrap());
    });
    let _ = async_move!([v1, v2] {
        async_func(&v1).await;
        async_func(&*v2.lock().unwrap()).await;
    });
    println!("v1 = {:?}", *v1);
    println!("v2 = {:?}", *v2);
}
