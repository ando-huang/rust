pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    //FORMATTING
    println!("Number: {0}", 1); //a lot like python's str.format()
    println!("{0} is from {1} and his name is {0}", "Brad", "Brooklyn"); //formatting pt 2
    println!("{name} likes to eat {food}",
        name = "john",
        food = "tacos"
    );
    println!("Binary uses {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic mathematics
    println!("10 + 10 = {}", 10+10);
}