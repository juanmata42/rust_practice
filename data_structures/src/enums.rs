#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

pub fn enums() {
    println!("Enums----------------------------------");
    /* let c: Color = Color::Red; */
    /* let c: Color = Color::RgbColor(255, 0, 0); */
    let c: Color = Color::CmykColor{cyan:0, magenta:0, yellow:0, black:255};
    match c{
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0, 0, 0) => println!("Black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("Black"),
        Color::CmykColor{cyan, magenta, yellow, black} => println!("cmyk({}, {}, {}, {})", cyan, magenta, yellow, black),
    }
}