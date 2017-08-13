#![allow(dead_code)]
#![allow(unused_variables)]

///Unions
//The main purpose of unions seems to be saving memory by allowing multiple
//variables (whose lifetimes do not overlap) to exist in one allocation of 
//memory. Akin to people all using the same hotel room.
//Another purpose I've seen mentioned online is "type punning" but opinion on whether
//this is bad practice seems split.

union IntOrFloat
{
    i: i32,
    f: f32
}

fn main()
{
    unions();
    option();
}

fn option()
{
    //Option<T>

    let x = 3.0;
    let y = 2.0;
    //let y = 0.0;

    let result:Option<f64> = if y != 0.0 {Some(x/y)} else {None};

    match result
    {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("Cannot divide {} by {}!", x,y)
    }

    //this is using debug output to print "result" as is
    //println!("{:?}", result);

    //a "none" result evaluates to false for the purpose of statements such as if and while
    if let Some(z) = result {println!("z = {}", z);} //wouldn't println if y = 0.0
}

fn unions()
{
    //unsafe becase not checking types etc
    let mut iof = IntOrFloat { i: 123}; //define one or the other

    unsafe 
    {
        iof.i = 42;
    }

    process_value(iof);
    process_value(IntOrFloat {f: 1.1});

    //assignment to union
    //let value = unsafe {iof.i};

}

fn process_value(iof: IntOrFloat)
{
    unsafe 
    {
        match iof {
            IntOrFloat {i:42} => {println!("42 was entered");}
            IntOrFloat {f} => {println!("A float was entered = {}", f);}            
        }
    }
}