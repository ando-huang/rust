//This file will go over types in rs

/*
    integers u8, i8, etc through 128. u/i means unsigned/int
    floats: f32, f64
    bool
    char
    tuples
    arrays
*/

pub fn run(){
    let x = 1; //by default it assumes i32

    let f = 3.5; //by default this is f64

    let e:i64 = 100; //explicitly typed, we declared it

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_on = true;

    //get bool from expression
    let is_greater:bool = 10 > 100; //it figures this out from that

    //chars MUST BE SINGLE CHAR, not MULTIPLE. or use unicode
    let a1 = 'a';
    let face = '\u{1F600}'; //unicode for emoji

    println!("{:?}", (x, f, e, is_on, is_greater, a1, face));
}