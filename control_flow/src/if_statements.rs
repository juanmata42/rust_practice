#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

pub fn if_statements() {
    println!("If statements----------------------------------");
    let temp = 35;
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold outside");
    } else {
        println!("ok outside");
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);
    println!("it is {}", if temp > 20 {"hot"} else {"cold"});
}
