use std::mem; //imports the mem library operations

fn main() {
    let a:u8 = 123; //8bits
                    //a is the var name
                    //:u8 - u is for unsigned (zero or positive),  
                    //i is for signed
                    //value of a can be 0 - 255
    
    println!("a = {}", a); //this is a macro
                           //curlies are replacement for the latter params

    
    //a = 456; //will not work as vars are immutable by default!!

    //mut (mutable) keyword will allow you to change a var
    let mut b:i8 = 0;
    println!("b = {}", b);

    //b can be changed because it's mutable
    b = 44;
    println!("now b = {}", b);

    //if we leave the memory size delcaration off, Rust will guess
    let mut c = 123456789; //this will end up as a 32-bit signed i32
    println!("c = {}, size = {} bytes",c,mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification.", c);
    // i8/u8 - i16/u16 - i32/u32 - i64/u64

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, running on a {}-bit OS",
        z, size_of_z, size_of_z * 8);

    
    
    let d:char = 'x'; //:char is optional, compiler will figure it out
    println!("d = {}", d);



}