#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn using_tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp is {:?}", sp);
}

pub fn tuples() {
    println!("tuples----------------------------------");
    using_tuples();
    println!("structures: ");
    println!("vectors: ");
    println!("arrays: ");
}
