#![allow(dead_code)]
#![allow(unused_variables)]

mod structs;
mod enums;
mod unions;
mod optiont_iflet_whilelet;

fn main() {
    structs::structs();
    enums::enums();
    unions::unions();
    optiont_iflet_whilelet::options();
}
