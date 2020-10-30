//functions

pub fn run(){
    greeting("Hi", "Andrew");
    let get_sum = add(2, 54);
    println!("{}", get_sum);
    //bind func value to variables

    //closure vairables
    let n3:i32 = 33;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //we just declared a function here, allows us to use outside vars
    println!("C sum: {}", add_nums(3, 5));
}

fn greeting(greet: &str, name: &str){ //has the types with it
    println!("{}, {}! Nice to meet you!", greet, name);
}

fn add(n1:i32, n2:i32) -> i32{ //now it has the return type
    n1+n2
}