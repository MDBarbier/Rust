use std::mem;

//This is a global constant
const MY_CONST:u8 = 42; //no fixed mem address

//static global
static STATIC_GLOBAL:i32 = 123;

//static mutable global -- this can compromise memory safety so Rust will throw error
// by default you must declare uses of it unsafe
static mut STATIC_MUTABLE:i32 = 456;

fn main() {

    let a = 1;
    scope_demo();

println!("hello");

    println!("STATIC_GLOBAL = {}", STATIC_GLOBAL);

    unsafe
    {
        println!("STATIC_MUTABLE = {}", STATIC_MUTABLE);
        STATIC_MUTABLE = 789;
        println!("STATIC_MUTABLE is now = {}", STATIC_MUTABLE);
    }    
}


fn scope_demo() {

    println!("Scope demo");

    //println!("a = {}", a); //this will not work because a is out of scope here!
                            // a has ceased to exist!

    let a = 2;
    let a = 4; //this overrides the previous version, does not throw error like other
            // languages might!

    println!("a = {}", a);


    {
        println!("a = {}", a); //we can still access a here because it's a sub scope
        let a = 3; 
        println!("a = {}", a); //here we see that the variable a has changed
    }

    println!("a = {}", a); //but here we see the change was only to the inner 
                            //scope var because of Shadowing

}