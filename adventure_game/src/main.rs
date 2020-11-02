//mod in any of the other files

/**
 * This is the primary driver for the Rust Adventure game
 * 
 * @ando-huang 
 * Initialized 10/30/2020 (mandalorian chapter 9 release day)
 *  ***This is the way***
**/
mod character;
use std::io;

fn main(){

    println!("Welcome to the Rust Adventure game!");
    let mut player = character::Character::new("Andrew");
    println!("Is your character's name {}?", character::Character::get_name(&player));
    //Yes or no check, change if no
    let input = get_player_input();
    
     
}

fn get_player_input() -> bool {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("std read failed");
    let reduced = input_text.trim();
    match reduced.parse::<bool>(){
        Ok(i) => {
            return i;
        },
        Err(..) => {
            println!("Wrong format, enter again.");
            return get_player_input();
        },
    };
}