#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

//Arrays

fn main() 
{
    arrays();
}

fn arrays()
{
    //declares a mutable array of five 32 bit ints and initialising it
    let mut a:[i32;5] = [1,2,3,4,5];

    //could also write just:
    let mut b = [1,2,3,4,5];

    println!("a has {} elements and the first element contains {}",
        a.len(),
        a[0]);

    a[0] = 11;

    println!("a[0] now contains {}", a[0]);

    //debug print the array itself
    println!("{:?}", a);

    //sets b to an array with 10 elements, all containing "1"
    let c = [1;10];

    for i in 0..c.len()
    {
        println!("{}", c[i]);
    }

    println!("c took up {} bytes", mem::size_of_val(&c));

    //multi array
    let mtx:[[f32;3];3] = [[2.3, 3.5,6.2],[8.4,2.8,9.0],[1.8,7.4,7.7]];

    println!("{:?}", mtx);


}