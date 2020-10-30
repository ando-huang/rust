//This file will concern variables

//vars hold primitive data or references
//vars are immutable by default
//classic scope stuff

pub fn run(){
    let name = "Joseph";
    let age = 66; //like swift, these are immutable
    
    let mut val = 22; //now this is mutable
    println!("The best joestar is {} and he was {} and his son was {}", name, age, val);
    val = 10;

    println!("The best joestar is {} and he was {} and his son was {}", name, age, val);

    //define const
    const ID: i32 = 001; //its a constant named ID, all uppercase var name, and then type of it
    println!("ID: {}", ID);

    //multiple assignment
    let (my_name, my_age) = ("Jotaro", 19);
    println!("{} was buff at {}", my_name, my_age);
}