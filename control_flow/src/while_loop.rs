#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

pub fn while_loop() {
    println!("While loop----------------------------------");
    let mut x = 0;
    while x < 10 {
        x += 1;
        if x == 5 {
            continue;
        }
        println!("{}", x);
    }
    let mut y = 1;
    loop {
        y *= 2;
        println!("{}", y);
        if y == 32 {
            break;
        }
    }
}
