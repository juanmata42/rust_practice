#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

struct Point
{
    x: f64,
    y: f64,
}

fn origin() -> Point
{
    Point{x: 5555555559365675672388888888888888888888888888888888888888888888888888.4, y: 2.7}
}

pub fn stack_and_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes {} bytes", mem::size_of_val(&p1));
    println!("p2 takes {} bytes", mem::size_of_val(&p2));
    let p3 = *p2;
    println!("p3 takes {} bytes,{},{}", mem::size_of_val(&p3),p3.x,p3.y);
}
// why is it rounding p3.x ?