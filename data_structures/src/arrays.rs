#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

pub fn arrays() {
    println!("Arrays----------------------------------");
    let mut a: [i32; 5] = [1, 2, 3, 4, 5]; // a is mut variable array of 5 elements of type i32, we initialize it with values 1, 2, 3, 4, 5. we can take out the type here since it is implied in the value
    println!("a: {:?}", a);
    //this prints the whole array
    println!("a[0]: {} and has {} elements", a[0], a.len());
    a[0] = 765;
    println!("a[0]: {} and has {} elements", a[0], a.len());
    if a != [765, 2, 3, 4, 8] {
        println!("a is not equal to [765, 2, 3, 4, 8]");
    }
    let b:[i64; 10] = [123456789123456789; 10]; // this is an array of 10 items, all with same value
    for i in 0..b.len() {
        println!("b[{}] = {}", i, b[i]);
    };
    println!("b i64 took up {} bytes", mem::size_of_val(&b)); //& is the reference symbol
    // b takes 80 bytes because it is a fixed size array of 10 items of type i64
    // if we change the size of the array to a different size or type, it will take up more or less memory
    let c:[i32; 10] = [1; 10];
    println!("c i32,whose range is -2147483648..=2147483647, took up {} bytes", mem::size_of_val(&c));
    let d:[u8; 10] = [1; 10];
    println!("d u8,whose range is 0..=255, took up {} bytes", mem::size_of_val(&d));
    let e = [1u16; 10]; //can also specify the type here
    println!("e u16,whose range is 0..=65535, took up {} bytes", mem::size_of_val(&e));

    let mtx:[[f32; 3]; 3] = [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0]
    ];
    println!("mtx: {:?}", mtx);
    let stringey = ["\n\n\nHola", "Mu\n\nndo", "!
    kepasa
    bien"];
    println!("stringey: {:?}", stringey);
    println!("{} {} {}", stringey[0], stringey[1], stringey[2]);
}
