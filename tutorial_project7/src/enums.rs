#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub fn enums_demo()
{
    println!("Hello from enums.rs");

    enums();
}

fn enums()
{
    //let c:Actresses = Actresses::NatalieDormer(5);
    let c:Actresses = Actresses::MichelleFairley{SeasonStarted: 1, SeasonDied: 3};
    
    match c
    {
        Actresses::NathalieEmmanuel => println!("Missandei"),
        Actresses::EmiliaClarke => println!("Danerys"),
        Actresses::SophieTurner => println!("Sansa"),
        Actresses::LenaHeadey => println!("Cersei"),
        Actresses::NatalieDormer(x) => println!("Margery, died in series {}", x),
        Actresses::MichelleFairley{SeasonStarted: _, SeasonDied: 4} =>
            println!("Catelyn"),
        Actresses::MichelleFairley{SeasonStarted: season_started, SeasonDied: season_died} =>
            println!("Catelyn started: {}, died: {}", season_started, season_died)
    }

    let x = "Hello";
    println!("{}", x);
    
}

enum Actresses {
    LenaHeadey,
    EmiliaClarke,
    SophieTurner,
    NathalieEmmanuel,
    NatalieDormer(i32), //using a tuple here
    MichelleFairley{ SeasonStarted: i32, SeasonDied: i32} //struct
}