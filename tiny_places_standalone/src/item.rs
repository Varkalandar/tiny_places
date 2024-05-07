pub struct Item {
    
    // the ID must be unique in a game
    id: usize,

    pub name: String,
    pub mods: Vec<Mod>,    
}


impl Item {
    
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


struct ItemFactory
{
    next_id: usize,
}


impl ItemFactory {
    pub fn new() -> ItemFactory {
        ItemFactory {
            next_id: 0
        }
    }

    pub fn make_item(&mut self) -> Item {
        let id = self.next_id;
        self.next_id += 1;
        
        Item {
            id, 
            name: "".to_string(),
            mods: Vec::new(),
        }
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