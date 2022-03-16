#![allow(dead_code)]
#![allow(unused_variables)]
mod if_statements;
mod while_loop;
mod for_loop;
mod match_statement;
mod combination_lock;
fn main() {
    if_statements::if_statements();
    while_loop::while_loop();
    for_loop::for_loop();
    match_statement::match_statement();
    combination_lock::combination_lock();
}
