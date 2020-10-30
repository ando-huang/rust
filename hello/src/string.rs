//this file goes over strings
//primitive strings are arrays of chars, immutable
//String is a growable heap allocated data structure, mutable

pub fn run(){
    let mut hello = String::from("Hello "); //the String:: is for typing

    //getlen
    println!("Length: {}", hello.len());

    hello.push('W'); //push is for char

    hello.push_str("orld"); //this is for a string/mult chars same thing

    //lets find the capacity
    println!("Capacity: {}", hello.capacity()); //indicates byte capacity

    println!("isEmpty: {}", hello.is_empty()); //boolean check for empty

    println!("contains 'hello': {}", hello.contains("hello")); //case sensitive

    println!("replace 'world' for 'there': {}", hello.replace("World", "There"));//returns the modified string

    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create string with given capacity
    let mut s = String::with_capacity(10);
    s.push('a'); //single str
    s.push('b');

    //assertion
    assert_eq!(2, s.len());
    //only shows error if fails
    assert_eq!(10, s.capacity());
    
    println!("{}", s);
    //println!("{}", hello);
}