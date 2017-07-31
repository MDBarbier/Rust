#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 90.0, y: 180.0}
}

pub fn stack_and_heap()
{
    println!("Stack and heap demo");

    let p1 = origin();
    let p2 = Box::new(origin()); //p2 is a pointer to where the value is on the heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    //p2 takes up fewer bytes because it's just a pointer to the memory address where
    //the actual value is stored

    let p3 = *p2; //following the ptr to unbox it

    println!("p3 = {}", p3.x);
}