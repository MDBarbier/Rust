#![allow(dead_code)]

mod sh;
mod conditionals_and_loops;

fn main() {
    sh::stack_and_heap();
    conditionals_and_loops::if_demo();
    conditionals_and_loops::while_loop();
    conditionals_and_loops::for_loop();
    conditionals_and_loops::match_statement();
}
