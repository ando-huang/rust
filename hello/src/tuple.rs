//this file goes over tuples
//doesnt have to be same type, max at 12 elems

pub fn run(){
    let person:(&str, &str, i8) = ("Andrew", "New York", 20);

    println!("{} is from {} and is {}", person.0, person.1, person.2); //use .IND to access

    
}