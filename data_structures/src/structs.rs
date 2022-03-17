#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

struct Point {
    x:f64,
    y:f64
}

struct Line {
    start:Point,
    end:Point
}

pub fn structs() {
    println!("Structs----------------------------------");
    let p = Point { x: 1.0, y: 2.0 };
    println!("Point p is at  = ({}, {})", p.x, p.y);
    let p2 = Point { x: 3.0, y: 4.0 };
    let my_line = Line { start: p, end: p2 };
    println!("Line my_line is from ({}, {}) to ({}, {}) and has length of {}", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y, line_length(&my_line));
}
fn line_length(line: &Line) -> f64 {
    let dx = line.end.x - line.start.x;
    let dy = line.end.y - line.start.y;
    (dx * dx + dy * dy).sqrt()
}