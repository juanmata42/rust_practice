#![allow(dead_code)]
#![allow(unused_variables)]

mod structs;
mod enums;
mod unions;
mod optiont_iflet_whilelet;
mod arrays;
mod slices;
mod tuples;
mod pattern_matching;

fn main() {
    structs::structs();
    enums::enums();
    unions::unions();
    optiont_iflet_whilelet::options();
    arrays::arrays();
    slices::slices();
    tuples::tuples();
    pattern_matching::pattern_matching();
}
