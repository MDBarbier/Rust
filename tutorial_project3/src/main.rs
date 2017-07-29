use std::mem;

fn main() {

    operators();

}

fn operators() {
    
    //arithmetic operators

    let mut a = 2+4*8;
    
    println!("{}",a);

    a = a + 1; //++ and -- are not supported!

    println!("{}",a);

    a += 1; 

    println!("{}",a); // -=, +=, *=, %= are available

    let a_cubed = i32::pow(a, 3); //cubing a number

    println!("a cubed is {}",a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("b = {}",b);
    println!("b cubed = {}",b_cubed);
    println!("b to pi = {}",b_to_pi);

    //bitwise operators (work on BITS)
    let c = 1 | 2; // | == OR, & == AND, ^ == XOR, ! == NOR
                   //this will evaluate to 3!
                   // 01 OR 10 == 11 == 3_10

    println!("c = {}",c);

    //shift operators

    let two_to_power_of_ten = 1 << 10; // >> also available

    println!("two to the power of ten = {}",two_to_power_of_ten);

    //logical operators >, <, <=, >=, ==

    let pi_less_than_4 = std::f64::consts::PI < 4.0; //true
    println!("pi is less than 4 = {}",pi_less_than_4);
}