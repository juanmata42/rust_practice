#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

/* fn pm() {
   
}
fn sh() {
   
} */
// going to return a static string
fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        //con esto le damos el nombre z al rango 9 a 13
        z @ 9..=13 => "lots of",
        5 => "five",
        _ if (x%2==0) => "an even number of them",
        _ => "a few",
    }
}
// 0..12 rango 0 a 11 incluido, 0..=12 rango 0 a 12 incluido
pub fn pattern_matching() {
    println!("Pattern Matching----------------------------------");
    /* pm();
    sh(); */
    for x in 0..=13{
        println!("{}: I have {} oranges", x, how_many(x));
    }
    let point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (x,y) => println!("{}, {}", x, y),
        // (_,y) => println!("?, {}", y),
    }
}
