use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Andrew";

    println!("Args: {:?}", args); //prints each of the args separately
    println!("Command: {}", command);
    if command == "Hello" {
        println!("Hi {}, how are you?", name);
    }
    else if command == "status"{
        println!("Status: 100%");
    }
}