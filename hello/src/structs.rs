//structs are like classes, like members of a class
//functions related to the struct
//std struct
struct Color {
    red: u8,
    blu: u8,
    grn: u8
}

//Tuple struct
struct Color2(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    //construct person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str ){
        self.last_name = last.to_string();
    }
}

pub fn run(){

    let mut p = Person::new("Andrew", "Huang");
    println!("Person: {}", p.full_name());
    p.set_last_name("Tseng");
    println!("Person {} {}", p.first_name, p.last_name);
    //creating two diff structs
    let mut c = Color{
        red: 255,
        blu: 0,
        grn: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.blu, c.grn);

    let mut c2 = Color2(255, 0, 0);
    c2.0 = 200;
    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);



}