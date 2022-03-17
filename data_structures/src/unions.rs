#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

// Union is 32bits
union IntOrFloat {
    i: i32, //integer
    f: f32, //float
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("The meaning of life is 42"),
            IntOrFloat { f: 3.14 } => println!("The meaning of life is 3.14"),
            _ => println!("The meaning of life is unknown"),
        }
    }
    /* println!("{}", iof.i);
    println!("{}", iof.f); */
}

pub fn unions() {
    println!("Unions----------------------------------");
    
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 42;
    iof.f = 42.0;
    
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);
    process_value(iof);
}
