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
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    println!("sp is {0:?} {1:?}", sp.0, sp.1);
    //destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("first combined: {:?}", combined);
}

pub fn tuples() {
    println!("tuples----------------------------------");
    using_tuples();
}
