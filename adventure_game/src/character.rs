/**
 * This contains the definition for a character struct
 * 
 * 
 * 
 */

pub struct Character{
    name: String,
    attack: i32,
    defense: i32,
    max_health: i32,
    curr_health: i32,
    speed: i32,
    preset: i8
}

impl Character{
    pub fn new(name_: &str) -> Character{
        Character{
            name: name_.to_string(),
            attack: 0,
            defense: 0,
            max_health: 0,
            curr_health: 0,
            speed: 0,
            preset: 0 //new characters can still be set
        }
    }

    fn get_name(&self) -> String{
        format!("{}", self.name)
    }

    /**
     * This is the setter for a warrior type character
    */
    pub fn warrior_stats(&mut self){
        if self.preset == 1{
            println!("This character has already been set!");
        }
        else{
            self.attack = 70;
            self.defense = 90;
            self.max_health = 80;
            self.curr_health = 80;
            self.speed = 50;
            self.preset = 1;
        }
    }

    /**
     * This is the setter for a ranged type character 
    */
    pub fn ranged_stats(&mut self){
        if self.preset == 1{
            println!("This character has already been set!");
        }
        else{
            self.attack = 90;
            self.defense = 50;
            self.max_health = 60;
            self.curr_health = 60;
            self.speed = 90;
            self.preset = 1;
        }
    }

    pub fn get_attack(&self) -> i32{
        self.attack
    }

    pub fn get_defense(&self) -> i32{
        self.defense
    }

    pub fn get_health(&self) -> i32{
        self.curr_health
    }

    pub fn get_speed(&self) -> i32{
        self.speed
    }

    /**
     * uses healing potions
     */ 
    pub fn use_heal(&mut self, heal: i32) -> i32{
        self.curr_health += heal;
        if self.curr_health > self.max_health{
            self.curr_health = self.max_health;
        }
        self.curr_health
    }

}