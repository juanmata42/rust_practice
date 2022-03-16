#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

pub fn for_loop() {
    println!("For loop----------------------------------");
    for x in 1..11 {
        if x == 5 {
            continue;
        }
        /* if x == 8 {
            break;
        } */
        println!("{}", x);
    }   
    // enumerate function returns an iterator over (index, value) pairs
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}
