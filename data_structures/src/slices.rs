#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn use_slice(slice: &mut [i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn slices() {
    println!("slices----------------------------------");
   let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
}
