#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

pub fn options() {
    println!("Option T and if let/while let-----------------------------------");

    let x = 3.0;
    let y = 2.0;
    //divide by zero
    //let y = 0.0;

    // option => Some(v)|None
    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("{}", "Division by zero"),
    }
    if let Some(z) = result {
        println!("result = {}", z);
    }
    /* infinite loop while let Some(z) = result {
        println!("result = {}", z);
    } */
}
