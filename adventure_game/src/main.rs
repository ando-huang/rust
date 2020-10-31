//mod in any of the other files

/**
 * This is the primary driver for the Rust Adventure game
 * 
 * @ando-huang 
 * Initialized 10/30/2020 (mandalorian chapter 9 release day)
 *  ***This is the way***
**/
mod character;
//use std::env;

fn main(){

    println!("Welcome to the Rust Adventure game!");
    let mut character = character::Character::new("Andrew");
    
}