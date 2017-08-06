#![allow(dead_code)]
#![allow(unused_variables)]

pub fn structs_demo()
{
    println!("Hello from structs.rs");

    structures();
}

struct Point
{
    x: f64, 
    y: f64
}

fn structures()
{
    let p = Point { x: 3.0, y: 7.54};
    let p2 = Point {x: 4.9, y: 18.4};
    let l = Line {start: p, end: p2};

    //println!("point p is at ({}, {})", p.x, p.y);

    println!("Line starts at ({},{}) and ends at ({},{})", l.start.x,
    l.start.y, l.end.x, l.end.y);

}

struct Line 
{
    start: Point,
    end: Point
}