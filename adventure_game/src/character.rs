/**
 * This contains the definition for a character struct
 * 
 * 
 * 
 */

struct Character{
    name: String,
    attack: i32,
    defense: i32,
    max_health: i32,
    curr_health: i32,
    speed: i32,
    preset: i8
};

impl Character{
    fn new(name_: &str) -> Character{
        Character{
            name = name_.to_string();
            preset = 0; //new characters can still be set
        }
    }

    fn get_name(&self) -> String{
        format!("{}", self.name);
    }

    /**
     * This is the setter for a warrior type character
    */
    fn warrior_stats(&mut self){
        if preset == 1{
            println!("This character has already been set!");
        }
        else{
            self.attack = 70;
            self.defense = 90;
            self.max_health = 80;
            self.curr_health = 80;
            self.speed = 50;
            preset = 1;
        }
    }

    /**
     * This is the setter for a ranged type character 
    */
    fn ranged_stats(&mut self){
        if preset == 1{
            println!("This character has already been set!");
        }
        else{
            self.attack = 90;
            self.defense = 50;
            self.max_health = 60;
            self.curr_health = 60;
            self.speed = 90;
            preset = 1;
        }
    }

    fn get_attack(&self) -> i32{
        self.attack;
    }

    fn get_defense(&self) -> i32{
        self.defense;
    }

    fn get_health(&self) -> i32{
        self.curr_health;
    }

    fn get_speed(&self) -> i32{
        self.speed;
    }

    /**
     * uses healing potions
     */ 
    fn use_heal(&mut self, heal: i32) -> i32{
        self.curr_health += heal;
        if self.curr_health > self.max_health{
            self.curr_health = self.max_health;
        }
        self.curr_health
    }

}