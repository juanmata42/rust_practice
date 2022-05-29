fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);
    // use isize
    let idx: usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);
    //option
    match a.get(1) {
        Some(x) => println!("a[1] = {}", x),
        None => println!("a[6] is not defined"),
    }
    for x in &a {
        println!("{}", x);
    }
    a.push(44);
    println!("a = {:?}", a);
    let last_element = a.pop(); // option
                                // some (44)
    println!("last element is {:?}, a = {:?}", last_element, a);
}
use std::collections::HashMap;
fn shapes() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("triangle"), 3);
    println!("a square has {} sides", shapes["square"]);
    //this one checks if 
    shapes.insert("square".into(),5);
shapes.entry("circle".into()).or_insert(1);
    for (key, value) in &shapes {
        println!("{} has {} sides", key, value);
    }
}
fn main() {
    vectors();
    shapes();
}
