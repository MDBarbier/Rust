#![allow(dead_code)]
#![allow(unused_variables)]

pub fn if_demo()
{
    let temp = 7;

    if temp > 30
    {
        println!("It's hot!!");
    }
    else if temp < 30 && temp > 10
    {
        println!("It's clement.");
    }
    else {
        println!("It's cold.");
    }

    let day = if temp > 20 {"nice."} else if temp >10 && temp <20 {"not so nice."} 
        else {"nasty!"};

    println!("{}", day);
}

pub fn while_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *=2;

        if x == 64 {continue;}
        if x > 1000 {break;}

        println!("x = {}", x);
    }

    let mut y = 1;
    
    loop // equivalent of "while true" - will never stop until you break
    {
        y *= 2;

        if y > 1000
        {
            break;
        }
        else {
            println!("y = {}", y);
        }

    }
}

pub fn for_loop()
{
    for i in 1..11 //loop from 1 to 10 (1..11 is know as a "range", special construct)
    {
        println!("i = {}", i);
    }

    for (pos,j) in (30..40).enumerate()
    {
        println!("the index, pos, = {}", pos);
        println!("the value, j, = {}", j);
    }
}

pub fn match_statement()
{
    let country_code = 44;

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        47 => "Russia",
        //1..999 => "Invalid code", //if value is not any of the above but is beteen 1 and 
        //999 then this will execute
        1...999 => "X", //everything not named between 0 and 1000
        _ => "Error" //equivalent of "default" in switch statement
    };

    println!("Country = {}, code = {}", country, country_code);
}