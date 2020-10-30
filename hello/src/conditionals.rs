//this file goes over the conditional statements in rs

pub fn run(){
    let age:u8 = 22;
    let check_id:bool = false;
    let knows_person = true;

    //ifelse
    if age >= 21 && check_id || knows_person {
        println!("Bartender: what would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("Bartender: sorry please leave");
    }
    else {
        println!("Bartender: I'll need to see your ID");
    }


}