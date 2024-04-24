pub struct Item {
    pub name: String,
    pub mods: Vec<Mod>,    
}


impl Item {
    pub fn new() -> Item {
        Item {
            name: "".to_string(),
            mods: Vec::new(),
        }
    }
    
    pub fn get_attribute_total_mod(self, attribute: Attribute) -> f32 {
        let mut sum: f32 = 0.0;

        for m in self.mods {
            if m.attribute == attribute {
                sum = sum + m.power as f32;
            }            
        }
        
        sum
    }
    
    pub fn print_debug(self) {
        println!("{}", self.name);
    }
}


#[derive(PartialEq, Eq)]
pub enum Attribute {

    Structure = 1,
    Agility = 2,
    Computation = 3,

    Integrity = 4,
    Energy = 5,
}


pub struct Mod {
    attribute: Attribute,
    power: i32,
}